use tokio::net::TcpStream;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use serde_json::{json, Value};
use std::error::Error;
use clap::{Parser};
use sha2::{Sha256, Digest};
use scrypt::{scrypt, Params};
use rand::Rng;
use std::time::{Instant, Duration};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

// Miner statistics
#[derive(Debug)]
struct MinerStats {
    shares_accepted: AtomicU64,
    shares_rejected: AtomicU64,
    hashes: AtomicU64,
    start_time: Instant,
}

impl Default for MinerStats {
    fn default() -> Self {
        Self {
            shares_accepted: AtomicU64::new(0),
            shares_rejected: AtomicU64::new(0),
            hashes: AtomicU64::new(0),
            start_time: Instant::now(),
        }
    }
}

impl MinerStats {
    fn new() -> Self {
        Self {
            start_time: Instant::now(),
            ..Default::default()
        }
    }

    fn hashrate(&self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            self.hashes.load(Ordering::Relaxed) as f64 / elapsed
        } else {
            0.0
        }
    }
}

// ANSI escape codes for UI
const CLEAR_SCREEN: &str = "\x1b[2J\x1b[H";
const MOVE_CURSOR: &str = "\x1b[1;1H";

// ANSI color codes
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const RESET: &str = "\x1b[0m";

fn display_stats(stats: &MinerStats, algorithm: MiningAlgorithm) {
    let hashrate = stats.hashrate();
    let elapsed = stats.start_time.elapsed();
    let accepted = stats.shares_accepted.load(Ordering::Relaxed);
    let rejected = stats.shares_rejected.load(Ordering::Relaxed);
    
    // Calculate success rate
    let total_shares = accepted + rejected;
    let success_rate = if total_shares > 0 {
        accepted as f64 / total_shares as f64 * 100.0
    } else {
        0.0
    };

    print!("{CLEAR_SCREEN}{MOVE_CURSOR}");
    println!("┌──────────────────────────────────────────────────────────────┐");
    println!("│ {GREEN}Quantum Miner v1.0{RESET} {:>40} │", 
        match algorithm {
            MiningAlgorithm::Sha256 => format!("{BLUE}SHA-256{RESET}"),
            MiningAlgorithm::Scrypt => format!("{YELLOW}Scrypt{RESET}"),
        });
    println!("├──────────────────────────────────────────────────────────────┤");
    println!("│ {GREEN}Accepted{RESET}: {: <10} {RED}Rejected{RESET}: {: <10} Success: {:.1}% │", 
        accepted,
        rejected,
        success_rate);
    println!("│ Hashrate: {:.2} H/s {:>30} │", hashrate,
        match hashrate {
            h if h > 1000.0 => format!("{GREEN}⚡ Excellent{RESET}"),
            h if h > 500.0 => format!("{YELLOW}⏱️  Good{RESET}"),
            _ => format!("{RED}⚠️  Low{RESET}"),
        });
    println!("│ Uptime: {:02}:{:02}:{:02} {:>35} │",
        elapsed.as_secs() / 3600,
        (elapsed.as_secs() % 3600) / 60,
        elapsed.as_secs() % 60,
        match elapsed.as_secs() {
            t if t > 3600 => format!("{GREEN}🟢 Stable{RESET}"),
            t if t > 600 => format!("{YELLOW}🟡 Warming{RESET}"),
            _ => format!("{RED}🔴 Starting{RESET}"),
        });
    println!("├──────────────────────────────────────────────────────────────┤");
    println!("│ {BLUE}Quantum Stats{RESET} {:>45} │",
        match algorithm {
            MiningAlgorithm::Sha256 => format!("Resonance: {:.5}", RESONANCE_THRESHOLD),
            MiningAlgorithm::Scrypt => format!("N Factor: 14"),
        });
    println!("└──────────────────────────────────────────────────────────────┘");
}

// Mining algorithm configuration
#[derive(Debug, Clone, Copy, PartialEq)]
enum MiningAlgorithm {
    Sha256,
    Scrypt,
}

/// Stratum miner with support for multiple algorithms
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Mining algorithm to use
    #[arg(short, long, default_value = "sha256")]
    algorithm: String,
    
    /// Port to connect to
    #[arg(short, long, default_value = "9995")]
    port: u16,
}

// Quantum resonance calculation
fn calculate_quantum_resonance(hash: &[u8]) -> f64 {
    // Convert first 8 bytes to f64 for resonance calculation
    let mut value: u64 = 0;
    for i in 0..8 {
        value = (value << 8) | hash[i] as u64;
    }
    
    // Calculate resonance based on bit patterns and prime factors
    let mut resonance = value as f64 / u64::MAX as f64;
    
    // Apply quantum wave function collapse simulation
    let phase = (value % 360) as f64 * std::f64::consts::PI / 180.0;
    resonance *= phase.cos().abs();
    
    resonance.abs()
}

// Quantum mining params
const RESONANCE_THRESHOLD: f64 = 0.00001;
const QUANTUM_BATCH_SIZE: usize = 1000;

// Your BTC wallet address here
const WALLET_ADDRESS: &str = "lonestar108.worker1";
const WORKER_NAME: &str = "x";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Parse command line arguments
    let args = Args::parse();
    
    // Select mining algorithm based on input
    let algorithm = match args.algorithm.to_lowercase().as_str() {
        "sha256" => MiningAlgorithm::Sha256,
        "scrypt" => MiningAlgorithm::Scrypt,
        _ => {
            eprintln!("Invalid algorithm specified. Supported algorithms: sha256, scrypt");
            std::process::exit(1);
        }
    };

    let stats = Arc::new(MinerStats::new());
    let stats_clone = stats.clone();
    let algorithm_clone = algorithm.clone();
    
    // Start stats display task
    tokio::spawn(async move {
        loop {
            display_stats(&stats_clone, algorithm_clone);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });
    
    // Connect to Mining-Dutch's BTC stratum server
    let stream = TcpStream::connect(format!("americas.mining-dutch.nl:{}", args.port)).await?;
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();

    // Subscribe to the pool
    let subscribe = json!({
        "id": 1,
        "method": "mining.subscribe",
        "params": ["quantum-miner/1.0.0"]
    });
    writer.write_all(format!("{}\n", subscribe.to_string()).as_bytes()).await?;

    // Authorize with your wallet
    let auth = json!({
        "id": 2,
        "method": "mining.authorize",
        "params": [format!("{}.{}", WALLET_ADDRESS, WORKER_NAME), "d=0.002428"]
    });
    writer.write_all(format!("{}\n", auth.to_string()).as_bytes()).await?;

    // Main mining loop
    loop {
        line.clear();
        reader.read_line(&mut line).await?;
        
        if let Ok(v) = serde_json::from_str::<Value>(&line) {
            match v["method"].as_str() {
                Some("mining.notify") => {
                    if let Some(params) = v["params"].as_array() {
                        let job_id = params[0].as_str().unwrap_or("");
                        let prev_block_hash = params[1].as_str().unwrap_or("");
                        let version = params[5].as_str().unwrap_or("");
                        let nbits = params[6].as_str().unwrap_or("");
                        let ntime = params[7].as_str().unwrap_or("");

                        // Quantum mining approach
                        let mut rng = rand::thread_rng();
                        let mut nonce = rng.gen_range(0..u32::MAX);
                        // Parse pool difficulty
                        let difficulty = v["params"][0].as_f64().unwrap_or(1.0);
                        // Calculate target based on difficulty
                        let target = if difficulty > 1.0 {
                            (0x0000ffffu64 << 48) / difficulty as u64
                        } else {
                            0x0000ffffu64 << 48
                        };
                        
                        // Apply quantum resonance
                        for _ in 0..QUANTUM_BATCH_SIZE {
                            stats.hashes.fetch_add(1, Ordering::Relaxed);
                            let result = match algorithm {
                                MiningAlgorithm::Sha256 => {
                                    let mut hasher = Sha256::new();
                                    let work = format!("{}{}{}{:08x}", version, prev_block_hash, ntime, nonce);
                                    hasher.update(work.as_bytes());
                                    hasher.finalize().to_vec()
                                }
                                MiningAlgorithm::Scrypt => {
                                    let work = format!("{}{}{}{:08x}", version, prev_block_hash, ntime, nonce);
                                    let mut output = [0u8; 32];
                                    let params = Params::new(14, 8, 1, 32).unwrap();
                                    scrypt(work.as_bytes(), &[], &params, &mut output).unwrap();
                                    output.to_vec()
                                }
                            };
                            
                            // Check quantum resonance pattern (only for SHA-256)
                            let resonance = match algorithm {
                                MiningAlgorithm::Sha256 => calculate_quantum_resonance(&result),
                                MiningAlgorithm::Scrypt => 0.0, // No quantum resonance for scrypt
                            };
                            
                            // Validate share against pool difficulty
                            // Calculate hash value with proper endianness
                            let hash_value = match algorithm {
                                MiningAlgorithm::Sha256 => {
                                    u64::from_le_bytes(result[0..8].try_into().unwrap())
                                }
                                MiningAlgorithm::Scrypt => {
                                    u64::from_le_bytes(result[24..32].try_into().unwrap())
                                }
                            };
                            
                            if (hash_value < target && resonance < RESONANCE_THRESHOLD) || 
                               (algorithm == MiningAlgorithm::Scrypt && hash_value < target) {
                                // Submit valid share
                                let submit = json!({
                                    "id": 4,
                                    "method": "mining.submit",
                                    "params": [
                                        format!("{}.{}", WALLET_ADDRESS, WORKER_NAME),
                                        job_id,
                                        format!("{:08x}", nonce)
                                    ]
                                });
                                writer.write_all(format!("{}\n", submit.to_string()).as_bytes()).await?;
                                break;
                            }
                            nonce = nonce.wrapping_add(1);
                        }
                    }
                },
                Some("mining.set_difficulty") => {
                    if let Some(difficulty) = v["params"][0].as_f64() {
                        println!("🎚️ Difficulty changed to: {}", difficulty);
                    }
                },
                _ => {
                    if let Some(result) = v["result"].as_array() {
                        if !result.is_empty() {
                            stats.shares_accepted.fetch_add(1, Ordering::Relaxed);
                        } else {
                            stats.shares_rejected.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
            }
        }
    }
}
