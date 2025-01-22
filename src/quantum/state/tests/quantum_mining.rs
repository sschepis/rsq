use crate::quantum::state::PrimeQuantumState;
use crate::mining::QuantumMiner;
use std::time::Instant;

#[tokio::test]
async fn test_quantum_state_initialization() {
    let state = PrimeQuantumState::new(256);
    assert_eq!(state.size, 256);
}

#[tokio::test]
async fn test_quantum_mining_performance() {
    let mut miner = QuantumMiner::new(512);
    
    // Test header similar to Python implementation
    let version = 2u32;
    let prev_block = vec![0u8; 32];
    let merkle_root = hex::decode("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b").unwrap();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;
    let bits = 0x1d00ffff_u32;

    let mut header = Vec::new();
    header.extend_from_slice(&version.to_le_bytes());
    header.extend_from_slice(&prev_block);
    header.extend_from_slice(&merkle_root);
    header.extend_from_slice(&timestamp.to_le_bytes());
    header.extend_from_slice(&bits.to_le_bytes());

    // Test mining at different difficulties
    let difficulties = [4, 5, 6];
    let mut results = Vec::new();

    for &target_zeros in &difficulties {
        println!("\nTesting difficulty {} zeros", target_zeros);
        let start = Instant::now();
        
        let result = miner.mine_block(&header, target_zeros, Some(10_000_000)).await;
        assert!(result.is_some(), "Failed to find block at difficulty {}", target_zeros);
        
        let (nonce, hash, mining_time) = result.unwrap();
        let leading_zeros = hash.chars()
            .take_while(|&c| c == '0')
            .count();
        
        assert!(
            leading_zeros >= target_zeros as usize,
            "Expected {} leading zeros, got {} for hash {}",
            target_zeros,
            leading_zeros,
            hash
        );

        let hashrate = nonce as f64 / mining_time;
        results.push((target_zeros, mining_time, hashrate));
        
        println!("Difficulty: {} zeros", target_zeros);
        println!("Nonce: {}", nonce);
        println!("Hash: {}", hash);
        println!("Mining time: {:.2} seconds", mining_time);
        println!("Hash rate: {:.2} hashes/second", hashrate);
    }

    // Verify performance scaling
    for i in 1..results.len() {
        let (prev_diff, prev_time, prev_rate) = results[i-1];
        let (curr_diff, curr_time, curr_rate) = results[i];
        
        println!("\nPerformance scaling from {} to {} zeros:", prev_diff, curr_diff);
        println!("Time ratio: {:.2}x", curr_time / prev_time);
        println!("Hashrate ratio: {:.2}x", curr_rate / prev_rate);
        
        // Verify reasonable scaling (each additional zero should take ~16x longer)
        assert!(
            curr_time / prev_time > 8.0,
            "Expected significant time increase for higher difficulty"
        );
    }
}

#[test]
fn test_quantum_state_normalization() {
    let mut state = PrimeQuantumState::new(256);
    state.initialize_with_primes(7);
    
    // Calculate norm of state vector
    let norm: f64 = state.state.iter()
        .map(|c| c.norm().powi(2))
        .sum::<f64>()
        .sqrt();
    
    // Verify state is normalized (norm should be very close to 1.0)
    assert!(
        (norm - 1.0).abs() < 1e-10,
        "State vector not properly normalized, norm = {}",
        norm
    );
}

#[test]
fn test_prime_resonance_patterns() {
    let mut state = PrimeQuantumState::new(256);
    let x: Vec<f64> = (0..256).map(|i| i as f64).collect();
    let resonance = state.apply_prime_resonance(&x);
    
    // Verify resonance peaks at prime positions
    for (i, &prime) in state.prime_resonances.iter().enumerate() {
        if prime >= 256 {
            break;
        }
        
        let peak_magnitude = resonance[prime as usize].norm();
        
        // Check neighboring positions
        let left = if prime > 0 {
            resonance[(prime - 1) as usize].norm()
        } else {
            0.0
        };
        
        let right = if (prime + 1) < 256 {
            resonance[(prime + 1) as usize].norm()
        } else {
            0.0
        };
        
        // Verify local maximum at prime positions
        assert!(
            peak_magnitude > left && peak_magnitude > right,
            "No resonance peak at prime {}, magnitudes: left={}, peak={}, right={}",
            prime,
            left,
            peak_magnitude,
            right
        );
    }
}

#[test]
fn test_phase_transition_effects() {
    let mut state = PrimeQuantumState::new(256);
    
    // Test phase transitions at different difficulties
    for difficulty in 6..=9 {
        state.initialize_with_primes(difficulty);
        let initial_state = state.state.clone();
        
        state.apply_phase_transition_correction(difficulty);
        
        // Verify state changed after phase transition
        assert!(
            state.state.iter().zip(initial_state.iter())
                .any(|(a, b)| (a.real - b.real).abs() > 1e-10 || (a.imag - b.imag).abs() > 1e-10),
            "Phase transition had no effect at difficulty {}",
            difficulty
        );
        
        // Verify state remains normalized
        let norm: f64 = state.state.iter()
            .map(|c| c.norm().powi(2))
            .sum::<f64>()
            .sqrt();
        assert!(
            (norm - 1.0).abs() < 1e-10,
            "State not normalized after phase transition at difficulty {}",
            difficulty
        );
    }
}
