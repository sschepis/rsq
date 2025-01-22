use clap::Parser;
use rsq::mining::{QuantumMiner, HashAlgorithm};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Hash algorithm to use (sha256, equihash, scrypt)
    #[arg(short, long, default_value = "sha256")]
    algorithm: String,

    /// Number of leading zeros required for mining difficulty
    #[arg(short, long, default_value_t = 4)]
    difficulty: u32,

    /// Resolution for quantum state (higher = more precise but slower)
    #[arg(short, long, default_value_t = 512)]
    resolution: usize,

    /// Maximum nonce to try before giving up (optional)
    #[arg(short, long)]
    max_nonce: Option<u32>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Parse hash algorithm
    let algorithm = match HashAlgorithm::from_str(&args.algorithm) {
        Some(alg) => alg,
        None => {
            eprintln!("Error: Invalid hash algorithm '{}'. Valid options are: sha256, equihash, scrypt", args.algorithm);
            std::process::exit(1);
        }
    };

    println!("Initializing quantum miner with:");
    println!("Algorithm: {:?}", algorithm);
    println!("Difficulty: {} leading zeros", args.difficulty);
    println!("Resolution: {}", args.resolution);
    if let Some(max) = args.max_nonce {
        println!("Max nonce: {}", max);
    }

    // Initialize miner
    let mut miner = QuantumMiner::new(args.resolution, algorithm);

    // Create test header (in practice this would come from the blockchain)
    let header = vec![0u8; 76];

    // Start mining
    match miner.mine_block(&header, args.difficulty, args.max_nonce).await {
        Some((nonce, hash, elapsed)) => {
            if !hash.is_empty() {
                println!("\nSuccess! Found block:");
                println!("Nonce: {}", nonce);
                println!("Hash: {}", hash);
                println!("Time: {:.2}s", elapsed);
            } else {
                println!("\nMining completed without finding a valid block");
                println!("Time elapsed: {:.2}s", elapsed);
            }
        }
        None => {
            println!("\nMining failed to complete");
        }
    }
}
