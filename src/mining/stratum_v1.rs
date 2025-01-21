use std::net::TcpStream;
use std::io::{BufReader, BufWriter, Write, BufRead};
use std::sync::{Arc, Mutex};
use serde_json::{Value, json};
use sha2::{Sha256, Digest};
use colored::*;
use crate::mining::{MiningOptions, NonceResult};

pub async fn mine_async(header: &[u8], target_zeros: u32, _options: MiningOptions) -> Option<NonceResult> {
    // Create a copy of the header to modify
    let work_header = header.to_vec();
    
    // Get the nonce position (last 4 bytes)
    let nonce_pos = work_header.len() - 4;
    
    // Calculate the number of iterations per thread
    let threads = 4; // Fixed number of threads
    let iterations = u32::MAX / threads as u32;
    
    // Create a channel for results
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    
    // Spawn worker threads
    for thread_id in 0..threads {
        let tx = tx.clone();
        let mut header_clone = work_header.clone();
        
        tokio::spawn(async move {
            let mut hasher = Sha256::new();
            let mut local_nonce = thread_id as u32 * iterations;
            let end_nonce = local_nonce + iterations;
            
            while local_nonce < end_nonce {
                // Update nonce in header
                header_clone[nonce_pos..nonce_pos+4].copy_from_slice(&local_nonce.to_le_bytes());
                
                // Double SHA-256 hash
                hasher.update(&header_clone);
                let first_hash = hasher.finalize_reset();
                hasher.update(&first_hash);
                let final_hash = hasher.finalize_reset();
                
                // Check if hash meets target
                let leading_zeros = final_hash.iter()
                    .take_while(|&&b| b == 0)
                    .count() as u32;
                
                if leading_zeros >= target_zeros {
                    let _ = tx.send(NonceResult {
                        nonce: local_nonce,
                        hash: hex::encode(final_hash),
                        mining_time: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs_f64(),
                    }).await;
                    return;
                }
                
                local_nonce += 1;
            }
        });
    }
    
    // Wait for first successful result
    rx.recv().await
}

#[derive(Clone)]
pub struct StratumClient {
    stream: Arc<Mutex<TcpStream>>,
    reader: Arc<Mutex<BufReader<TcpStream>>>,
    writer: Arc<Mutex<BufWriter<TcpStream>>>,
    job_id: Option<String>,
    extranonce1: Option<String>,
    extranonce2_size: Option<usize>,
    difficulty: u32,
    current_job: Option<StratumJob>,
    mining_options: MiningOptions,
}

#[derive(Clone)]
struct StratumJob {
    job_id: String,
    prev_hash: String,
    coinbase1: String,
    coinbase2: String,
    merkle_branch: Vec<String>,
    version: String,
    nbits: String,
    ntime: String,
    clean_jobs: bool,
}

impl StratumClient {
    pub fn new(pool_url: &str, mining_options: MiningOptions) -> Self {
        let addr = pool_url.trim_start_matches("stratum+tcp://");
        let stream = Arc::new(Mutex::new(TcpStream::connect(addr).expect("Failed to connect to pool")));
        let reader = Arc::new(Mutex::new(BufReader::new(stream.lock().unwrap().try_clone().unwrap())));
        let writer = Arc::new(Mutex::new(BufWriter::new(stream.lock().unwrap().try_clone().unwrap())));
        
        Self {
            stream: stream.clone(),
            reader: reader.clone(),
            writer: writer.clone(),
            job_id: None,
            extranonce1: None,
            extranonce2_size: None,
            difficulty: 1,
            current_job: None,
            mining_options,
        }
    }

    pub fn connect(&mut self, username: &str, password: &str) {
        // Send subscription request
        let subscribe_msg = json!({
            "id": 1,
            "method": "mining.subscribe",
            "params": []
        });
        
        if let Err(e) = self.send_message(&subscribe_msg) {
            eprintln!("🌊 {} {}", "Wipeout! Failed to send subscription:".bright_red().bold(), e.to_string().red());
            return;
        }

        // Wait for subscription response
        if let Some(response) = self.read_response() {
            println!("🏄 {} {}", "Subscription response:".bright_cyan().bold(), response.to_string().cyan());
            if let Some(result) = response["result"].as_array() {
                if result.len() >= 2 {
                    // Extract extranonce1 and extranonce2_size
                    if let Some(extranonce1) = result[1].as_str() {
                        self.extranonce1 = Some(extranonce1.to_string());
                    }
                    if let Some(size) = result[2].as_u64() {
                        self.extranonce2_size = Some(size as usize);
                    }
                }
            }
        }
        
        // Send authorization request
        let auth_msg = json!({
            "id": 2,
            "method": "mining.authorize",
            "params": [username, password]
        });
        
        if let Err(e) = self.send_message(&auth_msg) {
            eprintln!("🌊 {} {}", "Wipeout! Failed to send auth message:".bright_red().bold(), e.to_string().red());
            return;
        }

        // Wait for authorization response
        if let Some(response) = self.read_response() {
            println!("🔑 {} {}", "Authorization response:".bright_yellow().bold(), response.to_string().yellow());
        }
    }

    fn send_message(&mut self, message: &Value) -> Result<(), String> {
        let msg_str = message.to_string() + "\n";
        let mut writer = self.writer.lock().map_err(|e| format!("Failed to lock writer: {}", e))?;
        writer.write_all(msg_str.as_bytes())
            .map_err(|e| format!("Failed to write message: {}", e))?;
        writer.flush()
            .map_err(|e| format!("Failed to flush writer: {}", e))?;
        Ok(())
    }

    pub fn handle_message(&mut self, message: Value) {
        match message["method"].as_str() {
            Some("mining.set_difficulty") => {
                if let Some(params) = message["params"].as_array() {
                    if let Some(diff) = params[0].as_f64() {
                        self.difficulty = diff as u32;
                        println!("🏄‍♂️ {} {}", "Difficulty set to:".bright_cyan().bold(), diff.to_string().cyan());
                    } else {
                        eprintln!("🏄‍♂️ {}", "Bogus difficulty value received, dude!".bright_red().bold());
                    }
                }
            }
            Some("mining.notify") => {
                self.handle_new_job(message);
            }
            _ => {}
        }
    }

    fn handle_new_job(&mut self, message: Value) {
        println!("🎯 {} {}", "New mining job incoming:".bright_magenta().bold(), message.to_string().magenta());
        if let Some(params) = message["params"].as_array() {
            if params.len() >= 9 {
                let job = StratumJob {
                    job_id: params[0].as_str().unwrap_or("").to_string(),
                    prev_hash: params[1].as_str().unwrap_or("").to_string(),
                    coinbase1: params[2].as_str().unwrap_or("").to_string(),
                    coinbase2: params[3].as_str().unwrap_or("").to_string(),
                    merkle_branch: params[4].as_array()
                        .map(|arr| arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect())
                        .unwrap_or_default(),
                    version: params[5].as_str().unwrap_or("").to_string(),
                    nbits: params[6].as_str().unwrap_or("").to_string(),
                    ntime: params[7].as_str().unwrap_or("").to_string(),
                    clean_jobs: params[8].as_bool().unwrap_or(false),
                };
                
                self.current_job = Some(job);
                self.start_mining();
            }
        }
    }

    fn start_mining(&mut self) {
        if let Some(job) = &self.current_job {
            if let Some(header) = self.build_block_header(job) {
                let target_zeros = self.calculate_target_zeros();
                
                let mining_options = self.mining_options.clone();
                let header_clone = header.clone();
                
                let mut client_clone = self.clone();
                tokio::spawn(async move {
                    if let Some(result) = mine_async(&header_clone, target_zeros, mining_options).await {
                        println!("🏄‍♂️ {} nonce={}, hash={}", 
                            "Gnarly share found:".bright_green().bold(), 
                            result.nonce.to_string().cyan(), 
                            result.hash.bright_blue());
                        client_clone.submit_share(result);
                    }
                });
            } else {
                eprintln!("🌊 {} {}", "Totally wiped out! Failed to build block header for job:".bright_red().bold(), job.job_id.red());
            }
        }
    }

    fn build_block_header(&self, job: &StratumJob) -> Option<Vec<u8>> {
        let mut header = Vec::with_capacity(80);
        
        // Version (4 bytes, little-endian)
        let version = match u32::from_str_radix(&job.version, 16) {
            Ok(v) => v,
            Err(_) => {
                eprintln!("🌊 {} {}", "Wipeout! Failed to parse version:".bright_red().bold(), job.version.red());
                return None;
            }
        };
        header.extend_from_slice(&version.to_le_bytes());
        
        // Previous block hash (32 bytes, reverse byte order)
        let mut prev_hash = match hex::decode(&job.prev_hash) {
            Ok(h) => h,
            Err(_) => {
                eprintln!("🏄‍♂️ {} {}", "Bummer! Failed to decode prev_hash:".bright_red().bold(), job.prev_hash.red());
                return None;
            }
        };
        prev_hash.reverse();
        header.extend_from_slice(&prev_hash);
        
        // Merkle root (32 bytes)
        let coinbase = match self.build_coinbase(job) {
            Some(c) => c,
            None => return None,
        };
        let mut merkle_root = self.calculate_merkle_root(&coinbase, &job.merkle_branch)?;
        merkle_root.reverse();
        header.extend_from_slice(&merkle_root);
        
        // Timestamp (4 bytes, little-endian)
        let timestamp = match u32::from_str_radix(&job.ntime, 16) {
            Ok(t) => t,
            Err(_) => {
                eprintln!("🏄‍♂️ {} {}", "Bummer! Failed to parse ntime:".bright_red().bold(), job.ntime.red());
                return None;
            }
        };
        header.extend_from_slice(&timestamp.to_le_bytes());
        
        // Bits (4 bytes, little-endian)
        let bits = match u32::from_str_radix(&job.nbits, 16) {
            Ok(b) => b,
            Err(_) => {
                eprintln!("🌊 {} {}", "Totally bogus! Failed to parse nbits:".bright_red().bold(), job.nbits.red());
                return None;
            }
        };
        header.extend_from_slice(&bits.to_le_bytes());
        
        // Nonce (4 bytes, will be filled during mining)
        header.extend_from_slice(&[0u8; 4]);
        
        Some(header)
    }
    
    fn build_coinbase(&self, job: &StratumJob) -> Option<Vec<u8>> {
        let mut coinbase = Vec::new();
        
        // Coinbase version (4 bytes)
        coinbase.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]);
        
        // Extranonce1
        if let Some(extranonce1) = &self.extranonce1 {
            if let Ok(decoded) = hex::decode(extranonce1) {
                coinbase.extend_from_slice(&decoded);
            } else {
                eprintln!("🌊 {} {}", "Totally bogus! Failed to decode extranonce1:".bright_red().bold(), extranonce1.red());
                return None;
            }
        }
        
        // Extranonce2 (placeholder)
        if let Some(size) = self.extranonce2_size {
            coinbase.extend_from_slice(&vec![0u8; size]);
        }
        
        // Coinbase script (arbitrary data)
        if let Ok(decoded1) = hex::decode(&job.coinbase1) {
            coinbase.extend_from_slice(&decoded1);
        } else {
            eprintln!("🏄‍♂️ {} {}", "Totally bogus! Failed to decode coinbase1:".bright_red().bold(), job.coinbase1.red());
            return None;
        }
        
        if let Ok(decoded2) = hex::decode(&job.coinbase2) {
            coinbase.extend_from_slice(&decoded2);
        } else {
            eprintln!("🌊 {} {}", "Totally bogus! Failed to decode coinbase2:".bright_red().bold(), job.coinbase2.red());
            return None;
        }
        
        Some(coinbase)
    }
    
    fn calculate_merkle_root(&self, coinbase: &[u8], merkle_branch: &[String]) -> Option<Vec<u8>> {
        let mut hasher = Sha256::new();
        hasher.update(coinbase);
        let mut hash = hasher.finalize().to_vec();
        hasher = Sha256::new();
        hasher.update(&hash);
        hash = hasher.finalize().to_vec();
        
        for branch in merkle_branch {
            let branch_hash = match hex::decode(branch) {
                Ok(h) => h,
                Err(_) => {
                    eprintln!("🏄‍♂️ {} {}", "Totally bogus! Failed to decode merkle branch:".bright_red().bold(), branch.red());
                    return None;
                }
            };
            let mut reversed = branch_hash.clone();
            reversed.reverse();
            hash.extend_from_slice(&reversed);
            
            let mut hasher = Sha256::new();
            hasher.update(&hash);
            hash = hasher.finalize().to_vec();
            hasher = Sha256::new();
            hasher.update(&hash);
            hash = hasher.finalize().to_vec();
        }
        
        Some(hash)
    }

    fn calculate_target_zeros(&self) -> u32 {
        // Calculate target based on difficulty
        // Bitcoin difficulty 1 target is 0x1d00ffff
        let max_target = 0x1d00ffffu32;
        let target = max_target / self.difficulty;
        
        // Count leading zeros in target
        let mut zeros = 0;
        let mut mask = 0x80000000u32;
        while (target & mask) == 0 && mask != 0 {
            zeros += 1;
            mask >>= 1;
        }
        
        zeros
    }

    fn submit_share(&mut self, result: NonceResult) {
        if let Some(job) = &self.current_job {
            // Convert nonce to little-endian bytes
            let nonce_bytes = result.nonce.to_le_bytes();
            let nonce_hex = hex::encode(nonce_bytes);
            
            // Convert hash to little-endian
            let mut hash_bytes = hex::decode(&result.hash).unwrap();
            hash_bytes.reverse();
            let hash_hex = hex::encode(hash_bytes);
            
            // Get current timestamp
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            let submit_msg = json!({
                "id": 3,
                "method": "mining.submit",
                "params": [
                    "lonestar108",
                    job.job_id,
                    "extranonce2",
                    format!("{:08x}", timestamp),
                    nonce_hex,
                    hash_hex
                ]
            });
            
            if let Err(e) = self.send_message(&submit_msg) {
                eprintln!("🏄‍♂️ {} {}", "Bummer! Failed to submit share:".bright_red().bold(), e.to_string().red());
                return;
            }
            
            // Check if share was accepted
            if let Some(response) = self.read_response() {
                if response["result"].as_bool().unwrap_or(false) {
                    println!("🎉 {} {}", "Share accepted!".bright_green().bold(), "Cowabunga!".bright_yellow());
                } else {
                    eprintln!("🌊 {} {:?}", "Wipeout! Share rejected:".bright_red().bold(), response["error"]);
                }
            } else {
                eprintln!("🌫️ {}", "No response from the pool, bummer!".bright_yellow().bold());
            }
        }
    }
    
    pub fn read_response(&mut self) -> Option<Value> {
        let mut line = String::new();
        if let Ok(mut reader) = self.reader.lock() {
            if reader.read_line(&mut line).is_ok() {
                serde_json::from_str(&line).ok()
            } else {
                None
            }
        } else {
            eprintln!("🌊 {}", "Gnarly wipeout! Failed to lock reader!".bright_red().bold());
            None
        }
    }
}
