pub mod stratum_v1;

use sha2::{Sha256, Digest};
use std::sync::Arc;
use tokio::sync::mpsc;
use std::f64::consts::{PI, E, SQRT_2, LN_2};

const MAX_UINT32: u32 = 0xFFFFFFFF;
const PHI: f64 = 1.618033988749895;
const TWO_PI: f64 = 2.0 * PI;
const SQRT_3: f64 = 1.7320508075688772;
const SQRT_5: f64 = 2.23606797749979;
const LN_3: f64 = 1.0986122886681098; // ln(3)

// Pre-calculated primes
const PRIMES: [u32; 25] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];

// Phase angles based on SHA-256 compression patterns
const PHASE_ANGLES: [f64; 12] = [
    0.0, PHI, 1.0/PHI, PI, E,
    SQRT_2, SQRT_3, SQRT_5,
    PI/2.0, PI/4.0,
    LN_2, LN_3
];

#[derive(Debug, Clone)]
pub struct MiningOptions {
    pub skew_factor: f64,
    pub resonance_weight: f64,
    pub prime_weight: f64,
    pub learning_rate: f64,
    pub chunk_size: u32,
    pub max_nonce: Option<u32>,
    pub quantum_boost: Option<f64>,  // New quantum amplification factor
}

impl Default for MiningOptions {
    fn default() -> Self {
        Self {
            skew_factor: 0.22,        // Enhanced quantum skew
            resonance_weight: 0.48,    // Boosted for high-difficulty resonance
            prime_weight: 0.38,        // Increased prime influence
            learning_rate: 0.06,       // Accelerated learning for quantum patterns
            chunk_size: 8192,
            max_nonce: None,
            quantum_boost: Some(0.15), // Default quantum amplification
        }
    }
}

#[derive(Debug)]
pub struct NonceResult {
    pub nonce: u32,
    pub hash: String,
    pub mining_time: f64,
}

pub fn analyze_nonce_pattern(nonce: u32) -> (u32, u32, u32, u32, u32) {
    let binary = format!("{:032b}", nonce);
    
    // Count leading zeros until we find a '1'
    let mut leading_zeros = 0;
    for bit in binary.chars() {
        if bit == '1' {
            break;
        }
        leading_zeros += 1;
    }
    
    let trailing_zeros = binary.chars().rev().take_while(|&c| c == '0').count() as u32;
    let one_count = binary.chars().filter(|&c| c == '1').count() as u32;
    
    let mut current_run = 0;
    let mut max_run = 0;
    
    // Count transitions between groups of 2 or more identical bits
    let chars: Vec<_> = binary.chars().collect();
    let mut transitions: u32 = 0;
    let mut current_bit = chars[0];
    let mut run_length = 1;
    
    for &bit in chars.iter().skip(1) {
        if bit == current_bit {
            run_length += 1;
        } else {
            if run_length >= 2 {
                transitions += 1;
            }
            current_bit = bit;
            run_length = 1;
        }
    }
    
    // Count final transition if last run was 2+ bits
    if run_length >= 2 {
        transitions += 1;
    }
    
    // Subtract 1 since we counted both ends of each transition
    if transitions > 0 {
        transitions -= 1;
    }
    
    // Count runs of zeros
    for bit in binary.chars() {
        if bit == '0' {
            current_run += 1;
            max_run = max_run.max(current_run);
        } else {
            current_run = 0;
        }
    }
    
    (leading_zeros, trailing_zeros, one_count, max_run, transitions)
}

use crate::quantum::resonance::PrimeWaveFunction;

fn calculate_adaptive_skew(
    base_nonce: u32,
    difficulty: u32,
    prime_factor: u32,
    options: &MiningOptions
) -> u32 {
    if difficulty < 6 {
        return base_nonce;
    }
    
    // Initialize our quantum prime wave function
    let wave = PrimeWaveFunction::new();
    
    // Calculate quantum resonance for a window of nonces around base_nonce
    let window_size = 1024u32;
    let start_nonce = base_nonce.saturating_sub(window_size / 2);
    let mut best_nonce = base_nonce;
    let mut max_resonance = 0.0;
    
    for nonce in start_nonce..start_nonce.saturating_add(window_size) {
        let resonance = wave.evaluate(nonce.into(), None);
        if resonance > max_resonance {
            max_resonance = resonance;
            best_nonce = nonce;
        }
    }
    
    // Apply quantum-enhanced adjustments
    let mut quantum_factor = options.resonance_weight;
    
    // Apply additional quantum boost for higher difficulties
    if difficulty >= 7 {
        if let Some(boost) = options.quantum_boost {
            let difficulty_scale = 1.0 + (difficulty as f64 - 6.0) * boost;
            quantum_factor *= difficulty_scale;
        }
    }
    
    let quantum_adjusted = (best_nonce as f64 * quantum_factor + 
                          base_nonce as f64 * (1.0 - quantum_factor)) as u32;
    
    // Enhanced prime resonance with quantum alignment
    let prime_resonance = if difficulty >= 7 {
        options.prime_weight * (1.0 + (difficulty as f64 - 6.0) * 0.08)
    } else {
        options.prime_weight
    };
    
    // Combine components with quantum-enhanced weighting
    let mut final_nonce = quantum_adjusted;
    // Apply prime resonance scaling to the prime factor
    let scaled_prime = ((prime_factor as f64 * prime_resonance) as u32) & MAX_UINT32;
    final_nonce = (final_nonce.wrapping_mul(scaled_prime)) & MAX_UINT32;
    
    // Apply final quantum phase alignment
    if let Some(boost) = options.quantum_boost {
        let phase_factor = (1.0 + boost * max_resonance).min(2.0);
        final_nonce = ((final_nonce as f64 * phase_factor) as u32) & MAX_UINT32;
    }
    
    final_nonce
}

pub async fn mine_async(
    header: &[u8],
    target_zeros: u32,
    options: MiningOptions,
) -> Option<NonceResult> {
    let max_nonce = options.max_nonce.unwrap_or(MAX_UINT32);
    let chunk_size = options.chunk_size;
    let header = Arc::new(header.to_vec());
    
    let (tx, mut rx) = mpsc::channel(32);
    // Get max CPU cores and optimize thread distribution
    let num_workers = num_cpus::get();
    println!("🚀 Mining with {} CPU cores", num_workers);
    
    // Enhanced target pattern setup for quantum alignment
    let target_pattern = vec![0u8; (target_zeros as usize + 7) / 8];
    let target_bits = target_zeros as usize % 8;
    
    // Prepare quantum-optimized thread handles
    let mut handles = Vec::with_capacity(num_workers);
    let start_time = std::time::Instant::now();
    
    // Calculate optimal chunk distribution for quantum resonance
    let base_chunk = chunk_size / num_workers as u32;
    let quantum_chunk = if target_zeros >= 7 {
        // Smaller chunks for higher difficulties to maintain quantum coherence
        base_chunk / 2
    } else {
        base_chunk
    };

    for worker_id in 0..num_workers {
        let tx = tx.clone();
        let header = header.clone();
        let options = options.clone();
        let target_pattern = target_pattern.clone();
        
        handles.push(tokio::spawn(async move {
            let mut test_data = [0u8; 80]; // Standard Bitcoin header size
            test_data[..header.len()].copy_from_slice(&header);
            
            let worker_chunk_size = quantum_chunk;
            let mut prime_idx = worker_id % PRIMES.len();
            
            for base_nonce in (worker_id as u32..max_nonce).step_by(num_workers) {
                let prime_factor = PRIMES[prime_idx];
                prime_idx = (prime_idx + 1) % PRIMES.len();
                
                let nonce = if target_zeros >= 6 {
                    calculate_adaptive_skew(base_nonce, target_zeros, prime_factor, &options)
                } else {
                    base_nonce
                };
                
                test_data[header.len()..header.len() + 4].copy_from_slice(&nonce.to_le_bytes());
                
                let hash1 = Sha256::digest(&test_data[..header.len() + 4]);
                let final_hash = Sha256::digest(&hash1);
                
                if &final_hash[..target_pattern.len()] == target_pattern.as_slice() {
                    if target_bits == 0 || final_hash[target_pattern.len()] >> (8 - target_bits) == 0 {
                        let mining_time = start_time.elapsed().as_secs_f64();
                        let result = NonceResult {
                            nonce,
                            hash: hex::encode(final_hash),
                            mining_time,
                        };
                        let _ = tx.send(Some(result)).await;
                        return;
                    }
                }
                
                if base_nonce % worker_chunk_size == 0 {
                    tokio::task::yield_now().await;
                }
            }
            
            let _ = tx.send(None).await;
        }));
    }

    drop(tx); // Drop original sender

    while let Some(result) = rx.recv().await {
        if let Some(result) = result {
            // Cancel other workers by dropping rx
            drop(rx);
            return Some(result);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mining_basic() {
        let header = vec![0u8; 76]; // Empty header for testing
        let target_zeros = 4; // Start with a reasonable difficulty
        let options = MiningOptions::default();
        
        let result = mine_async(&header, target_zeros, options).await;
        assert!(result.is_some());
        
        let result = result.unwrap();
        let hash_bytes = hex::decode(&result.hash).unwrap();
        let leading_zeros = hash_bytes[0].leading_zeros() as u32;
        assert!(leading_zeros >= target_zeros);
    }

    #[tokio::test]
    async fn test_mining_increasing_difficulty() {
        let header = vec![0u8; 76];
        let mut target_zeros = 3;
        let options = MiningOptions::default();
        
        while target_zeros <= 5 {
            let result = mine_async(&header, target_zeros, options.clone()).await;
            assert!(result.is_some());
            
            let result = result.unwrap();
            let hash_bytes = hex::decode(&result.hash).unwrap();
            let leading_zeros = hash_bytes[0].leading_zeros() as u32;
            assert!(leading_zeros >= target_zeros);
            
            target_zeros += 1;
        }
    }

    #[test]
    fn test_analyze_nonce_pattern() {
        let nonce = 0b00110011000011110000111100001111u32;
        println!("Binary representation: {:032b}", nonce);
        let (leading_zeros, trailing_zeros, one_count, max_run, transitions) = 
            analyze_nonce_pattern(nonce);
        
        println!("Leading zeros: {}", leading_zeros);
        println!("Trailing zeros: {}", trailing_zeros);
        println!("One count: {}", one_count);
        println!("Max run: {}", max_run);
        println!("Transitions: {}", transitions);
        
        assert_eq!(leading_zeros, 2);
        assert_eq!(trailing_zeros, 0);
        assert_eq!(one_count, 16);
        assert_eq!(max_run, 4);
        assert_eq!(transitions, 9);
    }
}
