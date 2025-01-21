use rsq::quantum::resonance::PrimeWaveFunction;
use std::time::Instant;
use std::thread;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::Parser;
use sha2::{Sha256, Digest};

const PHI: f64 = 1.618033988749895;
const PRIMES: &[u32] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Target difficulty (number of leading zeros)
    #[arg(short, long, default_value = "8")]
    difficulty: u32,

    /// Use historical blocks from CSV file
    #[arg(short = 'b', long)]
    use_blocks: bool,

    /// Initial header value (hex) when not using blocks
    #[arg(short = 'x', long, default_value = "0000000000000000000000000000000000000000000000000000000000000000")]
    header: String,

    /// Show all hashes with any leading zeros
    #[arg(short, long)]
    show_all: bool,

    /// Calculate and display hash rate
    #[arg(long)]
    hashrate: bool,
}

/// Mine a chunk of the nonce space
fn mine_chunk(
    start_nonce: u32,
    chunk_size: u32,
    header_bytes: &[u8],
    target_zeros: usize,
    resonance_func: &PrimeWaveFunction,
    show_all: bool,
    thread_id: usize,
    num_threads: usize,
) -> Option<(u32, String, u64)> {
    let mut hashes_checked = 0u64;

    for i in 0..chunk_size {
        let nonce = start_nonce.wrapping_add(i);
        let resonance = resonance_func.evaluate(nonce.into(), Some(header_bytes));
        
        // Show resonance values periodically
        if i == 0 {
            println!("Thread {} resonance at nonce {}: {:.4} (chunk start)", 
                thread_id, nonce, resonance);
        } else if i == chunk_size / 2 {
            println!("Thread {} resonance at nonce {}: {:.4} (chunk middle)", 
                thread_id, nonce, resonance);
        }
        
        // Check nonces with adaptive resonance threshold
        let threshold = if target_zeros >= 6 { 0.7 } else { 0.4 };
        if resonance > threshold {
            // Prepare block header with nonce
            let mut test_data = Vec::with_capacity(header_bytes.len() + 4);
            test_data.extend_from_slice(header_bytes);
            test_data.extend_from_slice(&nonce.to_le_bytes());
            
            // Double SHA256
            let hash1 = Sha256::digest(&test_data);
            let final_hash = Sha256::digest(&hash1);
            let hex_hash = hex::encode(&final_hash);
            
            // Count leading zeros in hex string
            let leading_zeros = hex_hash.chars()
                .take_while(|&c| c == '0')
                .count();

            hashes_checked += 1;

            // Check if we found a solution
            if leading_zeros >= target_zeros {
                return Some((nonce, hex_hash, hashes_checked));
            } else if show_all && leading_zeros > 0 {
                println!("Found hash with {} leading zeros: {} (nonce: {})", 
                    leading_zeros, hex_hash, nonce);
            }
        }
    }
    
    None
}

/// Parse a block header from CSV line
fn parse_block_header(line: &str) -> Option<(Vec<u8>, u32)> {
    // Trim whitespace and split
    let parts: Vec<&str> = line.trim().split(',').collect();
    println!("🔍 Parsing CSV line with {} parts", parts.len());
    if parts.len() < 7 {
        return None;
    }

    // Print raw values for debugging
    println!("📝 Raw values from CSV:");
    println!("  Version: {}", parts[1]);
    println!("  Prev hash: {}", parts[2]);
    println!("  Merkle root: {}", parts[3]);
    println!("  Timestamp: {}", parts[4]);
    println!("  Bits: {}", parts[5]);
    println!("  Nonce: {}", parts[6]);

    // Parse header values
    let version = match parts[1].trim().parse::<u32>() {
        Ok(v) => v,
        Err(e) => {
            println!("❌ Failed to parse version: {} (raw: '{}')", e, parts[1]);
            return None;
        }
    };
    let timestamp = match parts[4].trim().parse::<u32>() {
        Ok(t) => {
            println!("✅ Parsed timestamp: {}", t);
            t
        },
        Err(e) => {
            println!("❌ Failed to parse timestamp: {} (raw: '{}')", e, parts[4]);
            return None;
        }
    };
    let bits = match u32::from_str_radix(parts[5].trim(), 10) {
        Ok(b) => {
            println!("✅ Parsed bits: {:#x}", b);
            b
        },
        Err(e) => {
            println!("❌ Failed to parse bits: {} (raw: '{}')", e, parts[5]);
            return None;
        }
    };
    
    // Construct header bytes (76 bytes total before nonce)
    let mut header = Vec::with_capacity(76);
    
    // Version (4 bytes)
    header.extend_from_slice(&version.to_le_bytes());
    
    // Previous block hash (32 bytes) - Reverse byte order for LE
    if let Ok(mut prev_hash) = hex::decode(parts[2]) {
        prev_hash.reverse(); // Bitcoin uses little-endian for hashes
        header.extend_from_slice(&prev_hash);
    } else {
        return None;
    }
    
    // Merkle root (32 bytes) - Reverse byte order for LE
    if let Ok(mut merkle_root) = hex::decode(parts[3]) {
        merkle_root.reverse(); // Bitcoin uses little-endian for hashes
        header.extend_from_slice(&merkle_root);
    } else {
        return None;
    }
    
    // Timestamp (4 bytes)
    header.extend_from_slice(&timestamp.to_le_bytes());
    
    // Bits (4 bytes)
    header.extend_from_slice(&bits.to_le_bytes());

    // Verify header length
    if header.len() != 76 {
        println!("⚠️ Invalid header length: {} bytes", header.len());
        return None;
    }

    Some((header, timestamp))
}

fn main() {
    let args = Args::parse();
    println!("🌊 Starting Quantum Mining with Multiple Threads 🏄‍♂️");
    println!("Parameters:");
    println!("  Difficulty: {}", args.difficulty);
    if args.use_blocks {
        println!("  Using historical blocks from blocks.csv");
    } else {
        println!("  Header: {}", args.header);
    }
    println!("  Show all: {}", args.show_all);
    println!();

    // If using blocks, open the CSV file
    let blocks_reader = if args.use_blocks {
        println!("📚 Loading block headers from blocks.csv...");
        let file = match File::open("blocks.csv") {
            Ok(file) => file,
            Err(e) => {
                println!("❌ Failed to open blocks.csv: {}", e);
                return;
            }
        };
        Some(BufReader::new(file))
    } else {
        None
    };

    // Initialize quantum resonance function
    let resonance_func = Arc::new(PrimeWaveFunction::new());
    
    // Get initial header bytes
    let header_bytes = if let Some(mut reader) = blocks_reader {
        // Skip header line
        let mut header_line = String::new();
        if let Err(e) = reader.read_line(&mut header_line) {
            println!("❌ Failed to read CSV header: {}", e);
            return;
        }
        println!("📋 Skipped CSV header: {}", header_line.trim());

        // Read first data line
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(_) => {
                if let Some((header, timestamp)) = parse_block_header(&line) {
                    println!("📅 Using block with timestamp: {}", timestamp);
                    println!("🔍 Header details:");
                    println!("  Version: {}", u32::from_le_bytes(header[0..4].try_into().unwrap()));
                    println!("  Prev hash: {}", hex::encode(&header[4..36]));
                    println!("  Merkle root: {}", hex::encode(&header[36..68]));
                    println!("  Timestamp: {}", u32::from_le_bytes(header[68..72].try_into().unwrap()));
                    println!("  Bits: {:#x}", u32::from_le_bytes(header[72..76].try_into().unwrap()));
                    
                    // Calculate initial resonance
                    let resonance_func = PrimeWaveFunction::new();
                    let initial_resonance = resonance_func.evaluate(0, Some(&header));
                    println!("🌊 Initial resonance: {:.4}", initial_resonance);
                    
                    Arc::new(header)
                } else {
                    println!("❌ Failed to parse first block header from CSV");
                    return;
                }
            },
            Err(e) => {
                println!("❌ Failed to read blocks.csv: {}", e);
                return;
            }
        }
    } else {
        // Use provided hex header to construct a full block header
        let mut header = Vec::with_capacity(76);
        
        // Version (4 bytes)
        header.extend_from_slice(&2u32.to_le_bytes());
        
        // Previous block hash (32 bytes) from input
        let hash_bytes = hex::decode(&args.header).expect("Invalid header hex");
        if hash_bytes.len() != 32 {
            println!("❌ Header must be 32 bytes (64 hex characters)");
            return;
        }
        header.extend_from_slice(&hash_bytes);
        
        // Merkle root (32 bytes) - use zeros
        header.extend_from_slice(&[0u8; 32]);
        
        // Current timestamp (4 bytes)
        header.extend_from_slice(&(std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32)
            .to_le_bytes());
        
        // Bits (4 bytes) - Using standard testnet bits
        header.extend_from_slice(&0x1d00ffffu32.to_le_bytes());
        
        println!("🔍 Header details:");
        println!("  Version: {}", u32::from_le_bytes(header[0..4].try_into().unwrap()));
        println!("  Prev hash: {}", hex::encode(&header[4..36]));
        println!("  Merkle root: {}", hex::encode(&header[36..68]));
        println!("  Timestamp: {}", u32::from_le_bytes(header[68..72].try_into().unwrap()));
        println!("  Bits: {:#x}", u32::from_le_bytes(header[72..76].try_into().unwrap()));
        
        // Calculate initial resonance
        let resonance_func = PrimeWaveFunction::new();
        let initial_resonance = resonance_func.evaluate(0, Some(&header));
        println!("🌊 Initial resonance: {:.4}", initial_resonance);
        
        Arc::new(header)
    };

    // Start mining
    let start = Instant::now();
    let target_zeros = args.difficulty as usize;
    
    // Use number of CPU cores for parallelization
    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    println!("🧵 Using {} threads", num_threads);

    // Super aggressive chunk sizing to match Python performance
    let base_chunk_size = if target_zeros < 6 { 65536 } else { 32768 };
    let difficulty_factor = 1.0 + 0.15 * (target_zeros as f64 - 6.0).max(0.0);
    let chunk_size = (base_chunk_size as f64 / difficulty_factor) as u32;
    
    // Turbo quantum spacing with enhanced entropy
    let quantum_factor = PHI + (target_zeros as f64 - 5.0) * 0.8; // Max quantum influence
    let entropy_factor = 1.0 + (target_zeros as f64 - 6.0) * 0.7; // More entropy
    let pattern_factor = 1.0 + (target_zeros as f64 / 8.0); // Stronger patterns
    let resonance_factor = 1.0 + (target_zeros as f64 / 12.0); // Additional resonance
    let chunk_spacing = ((chunk_size as f64 * quantum_factor * entropy_factor * pattern_factor * resonance_factor) as u32) % (u32::MAX / chunk_size);
    
    println!("🔄 Chunk size: {}, spacing: {}", chunk_size, chunk_spacing);
    
    // Shared mining statistics
    let total_hashes = Arc::new(Mutex::new(0u64));
    let found_solution = Arc::new(Mutex::new(None::<(u32, String, u64)>));
    
    // Spawn worker threads
    let mut handles = vec![];
    
    for thread_id in 0..num_threads {
        let resonance_func = Arc::clone(&resonance_func);
        let header_bytes = Arc::clone(&header_bytes);
        let total_hashes = Arc::clone(&total_hashes);
        let found_solution = Arc::clone(&found_solution);
        let show_all = args.show_all;
        
        let handle = thread::spawn(move || {
            let mut start_nonce = thread_id as u32;
            
            loop {
                // Check if solution was found by another thread
                if found_solution.lock().unwrap().is_some() {
                    break;
                }
                
                // Mine chunk
                if let Some((nonce, hash, hashes)) = mine_chunk(
                    start_nonce,
                    chunk_size,
                    &header_bytes,
                    target_zeros,
                    &resonance_func,
                    show_all,
                    thread_id,
                    num_threads,
                ) {
                    // Update statistics
                    *total_hashes.lock().unwrap() += hashes;
                    
                    // Store solution
                    let mut solution = found_solution.lock().unwrap();
                    if solution.is_none() {
                        *solution = Some((nonce, hash, hashes));
                    }
                    break;
                }
                
                // Update statistics
                *total_hashes.lock().unwrap() += chunk_size as u64;
                
                // Enhanced chunk progression with prime-based offsets
                let prime_offset = PRIMES[thread_id % PRIMES.len()];
                start_nonce = start_nonce.wrapping_add(chunk_spacing * num_threads as u32)
                    .wrapping_mul(prime_offset);
                
                // Show progress
                if thread_id == 0 && start_nonce % (chunk_size * 10) == 0 {
                    let elapsed = start.elapsed().as_secs_f64();
                    let hashes = *total_hashes.lock().unwrap();
                    println!("🔍 Scanned approx {} nonces, checked {} hashes ({:.2} h/s)", 
                        start_nonce * num_threads as u32, 
                        hashes,
                        if elapsed > 0.0 { hashes as f64 / elapsed } else { 0.0 });
                }
            }
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Print results
    let elapsed = start.elapsed().as_secs_f64();
    let total_hashes = *total_hashes.lock().unwrap();
    let hash_rate = total_hashes as f64 / elapsed;
    
    // Take the solution while holding the lock
    let solution = found_solution.lock().unwrap().take();
    
    // Use solution after dropping the lock
    if let Some((nonce, hash, _)) = solution {
        println!("\n🎯 Found solution!");
        println!("Nonce: {}", nonce);
        println!("Hash: {}", hash);
        println!("Time: {:.2}s", elapsed);
        println!("Hashes checked: {}", total_hashes);
        println!("Hash rate: {:.2} h/s", hash_rate);
    } else {
        println!("❌ No solution found");
        println!("Time: {:.2}s", elapsed);
        println!("Hashes checked: {}", total_hashes);
        println!("Hash rate: {:.2} h/s", hash_rate);
    }

    // If hashrate flag is set, show detailed hash rate stats
    if args.hashrate {
        println!("\n📊 Detailed Hash Rate Stats:");
        println!("  Total hashes: {}", total_hashes);
        println!("  Elapsed time: {:.2} seconds", elapsed);
        println!("  Average hash rate: {:.2} h/s", hash_rate);
        println!("  Threads: {}", num_threads);
        println!("  Hashes per thread: {:.2} h/s", hash_rate / num_threads as f64);
    }
}
