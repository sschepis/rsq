use rsq::mining::{MiningOptions, stratum_v1::StratumClient};

#[tokio::main]
async fn main() {
    // Configure mining options
    let options = MiningOptions {
        skew_factor: 0.22,        // Enhanced quantum skew
        resonance_weight: 0.48,    // Boosted for high-difficulty resonance
        prime_weight: 0.38,        // Increased prime influence
        learning_rate: 0.06,       // Accelerated learning for quantum patterns
        chunk_size: 8192,          // Maintained for optimal threading
        max_nonce: None,
        quantum_boost: Some(0.15), // New quantum amplification factor
    };

    println!("🌊 Starting Quantum-Enhanced Stratum Miner 🏄‍♂️");
    println!("Enhanced with adaptive phase optimization");

    // Create stratum client
    // Connect to zergpool with super low initial difficulty
    let mut client = StratumClient::new("stratum+tcp://btc.f2pool.com:1314", options);

    println!("🌊 Connecting to zergpool - starting with baby waves...");
    println!("🏄‍♂️ Mining as: lonestar108.worker1");
    // Connect to pool with your username
    client.connect("lonestar108.worker1", "x");

    // Main mining loop
    loop {
        // Handle incoming messages
        if let Some(response) = client.read_response() {
            client.handle_message(response);
        }
        
        // Sleep to prevent busy waiting
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
