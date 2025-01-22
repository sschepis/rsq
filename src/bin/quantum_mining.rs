use sha2::{Sha256, Digest};
use std::time::{Instant, Duration, SystemTime, UNIX_EPOCH};
use std::collections::{HashMap, VecDeque};
use rand::{Rng, thread_rng};
use rand::seq::IteratorRandom;
use rayon::prelude::*;
use num_complex::Complex64;

// Starting with a very easy target for testing
const INITIAL_TARGET: u64 = 0x00FFFFFFFFFFFFFF;

// Base prime patterns
const PATTERN_625: u64 = 5 * 5 * 5 * 5;           // Powers of 5 pattern
const PATTERN_4648: u64 = 2 * 2 * 2 * 7 * 83;     // Powers of 2 with small primes
const PATTERN_152844: u64 = 2 * 2 * 3 * 47 * 271; // Mixed prime pattern

// Prime patterns that produce good hashes at different difficulty ranges
const PRIME_PATTERNS: [(u64, u64, u64); 4] = [
    (5, 4, 0),            // 5^4 for difficulties 1-16
    (3, 2, 47 * 271),     // 3^2 * (47 * 271) for difficulties 128-4096
    (3, 2, 317263),       // 3^2 * 317263 for difficulties 16384+
    (7, 2, 317263),       // 7^2 * 317263 for higher difficulties
];

// Most successful prime combinations
const PRIME_COMBINATIONS: [(u64, u64); 4] = [
    (3, 317263),      // 3 * 317263
    (7, 317263),      // 7 * 317263
    (47, 271),        // 47 * 271
    (3, 47 * 271),    // 3 * (47 * 271)
];

// Quantum-inspired constants
const PHI: f64 = 1.618033988749895; // Golden ratio
const RIEMANN_ZERO: f64 = 14.134725142; // First Riemann zero
const SIGMA: f64 = 0.45;

#[derive(Clone)]
struct DifficultyPattern {
    difficulty: f64,
    nonce: u64,
    hash: u64,
    prime_factors: Vec<u64>,
    leading_zeros: u32,
}

struct MiningHistory {
    successful_nonces: VecDeque<(u64, u64)>,          // (nonce, hash) pairs
    success_patterns: HashMap<Vec<u64>, u32>,         // prime factorization -> success count
    difficulty_patterns: Vec<DifficultyPattern>,      // Patterns at each difficulty level
    max_history: usize,
}

impl MiningHistory {
    fn new(max_history: usize) -> Self {
        Self {
            successful_nonces: VecDeque::new(),
            success_patterns: HashMap::new(),
            difficulty_patterns: Vec::new(),
            max_history,
        }
    }

    fn add_success(&mut self, nonce: u64, hash: u64, difficulty: f64) {
        let factors = get_prime_factors(nonce);
        *self.success_patterns.entry(factors.clone()).or_insert(0) += 1;
        
        // Count leading zeros in hash
        let leading_zeros = hash.leading_zeros();
        
        // Record difficulty pattern
        self.difficulty_patterns.push(DifficultyPattern {
            difficulty,
            nonce,
            hash,
            prime_factors: factors,
            leading_zeros,
        });
        
        self.successful_nonces.push_back((nonce, hash));
        if self.successful_nonces.len() > self.max_history {
            self.successful_nonces.pop_front();
        }
    }

    fn get_successful_patterns(&self) -> Vec<Vec<u64>> {
        let mut patterns: Vec<_> = self.success_patterns.iter()
            .map(|(k, v)| (k.clone(), *v))
            .collect();
        patterns.sort_by(|a, b| b.1.cmp(&a.1));
        patterns.into_iter()
            .take(5)
            .map(|(k, _)| k)
            .collect()
    }

    fn predict_next_patterns(&self, target_difficulty: f64) -> Vec<Vec<u64>> {
        let mut predictions = Vec::new();
        
        // Sort patterns by difficulty
        let mut sorted_patterns = self.difficulty_patterns.clone();
        sorted_patterns.sort_by(|a, b| a.difficulty.partial_cmp(&b.difficulty).unwrap());
        
        // Find patterns from similar difficulties
        let similar_patterns: Vec<_> = sorted_patterns.iter()
            .filter(|p| p.difficulty <= target_difficulty)
            .collect();
        
        if !similar_patterns.is_empty() {
            // Get the most recent pattern
            if let Some(last_pattern) = similar_patterns.last() {
                let mut factors = last_pattern.prime_factors.clone();
                
                // Try scaling up the largest prime factor
                if let Some(largest) = factors.iter_mut().max() {
                    // Find next prime after largest
                    let next_prime = find_next_prime(*largest);
                    *largest = next_prime;
                    predictions.push(factors.clone());
                }
                
                // Try squaring the smallest prime
                if let Some(smallest) = factors.iter_mut().min() {
                    *smallest = *smallest * *smallest;
                    predictions.push(factors.clone());
                }
            }
            
            // Look for progression patterns
            if similar_patterns.len() >= 2 {
                for window in similar_patterns.windows(2) {
                    let prev = &window[0];
                    let curr = &window[1];
                    
                    // If we see a pattern of increasing prime factors
                    if curr.leading_zeros > prev.leading_zeros {
                        let mut factors = curr.prime_factors.clone();
                        // Try continuing the pattern
                        for f in factors.iter_mut() {
                            *f = find_next_prime(*f);
                        }
                        predictions.push(factors);
                    }
                }
            }
        }
        
        predictions
    }
}

fn find_next_prime(n: u64) -> u64 {
    let mut candidate = n + 1;
    while !is_prime(candidate) {
        candidate += 1;
    }
    candidate
}

fn is_prime(n: u64) -> bool {
    if n <= 1 { return false; }
    if n <= 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn format_hash_rate(rate: f64) -> String {
    if rate >= 1_000_000.0 {
        format!("{:.2}M H/s", rate / 1_000_000.0)
    } else if rate >= 1_000.0 {
        format!("{:.2}K H/s", rate / 1_000.0)
    } else {
        format!("{:.2} H/s", rate)
    }
}

fn format_duration(d: Duration) -> String {
    if d.as_millis() < 1000 {
        format!("{}ms", d.as_millis())
    } else {
        format!("{:.2}s", d.as_secs_f64())
    }
}

fn get_prime_factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut d = 2;
    while n > 1 {
        while n % d == 0 {
            factors.push(d);
            n /= d;
        }
        d += if d == 2 { 1 } else { 2 };
        if d * d > n {
            if n > 1 {
                factors.push(n);
            }
            break;
        }
    }
    factors
}

fn generate_random_block() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let mut rng = thread_rng();
    let random_bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    let random_hex: String = random_bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect();
    format!("block_{}_{}", timestamp, random_hex)
}

fn generate_candidate_nonces(_: u64, history: &MiningHistory, difficulty: f64) -> Vec<u64> {
    let mut candidates = Vec::new();
    let mut rng = thread_rng();
    
    // Add predictions for current difficulty
    for pattern in history.predict_next_patterns(difficulty) {
        let mut product = 1u64;
        for &prime in &pattern {
            if let Some(p) = product.checked_mul(prime) {
                product = p;
            } else {
                break;
            }
        }
        if product > 1 {
            candidates.push(product);
        }
    }
    
    // Add variations of successful patterns from history
    for pattern in history.get_successful_patterns() {
        let mut product = 1u64;
        for &prime in &pattern {
            if let Some(p) = product.checked_mul(prime) {
                product = p;
            } else {
                break;
            }
        }
        if product > 1 {
            candidates.push(product);
            
            // Try small variations
            for i in 1..=3 {
                if let Some(var) = product.checked_mul(i) {
                    candidates.push(var);
                }
            }
        }
    }
    
    // Generate nonces based on prime patterns
    for &(base, power, multiplier) in &PRIME_PATTERNS {
        let base_nonce = base.pow(power as u32);
        if multiplier == 0 {
            candidates.push(base_nonce);
        } else {
            if let Some(product) = base_nonce.checked_mul(multiplier) {
                candidates.push(product);
            }
        }
        
        // Try variations around the pattern
        for i in 1..=3 {
            if multiplier == 0 {
                if let Some(product) = base_nonce.checked_mul(i) {
                    candidates.push(product);
                }
            } else {
                if let Some(product) = base_nonce.checked_mul(multiplier) {
                    if let Some(final_product) = product.checked_mul(i) {
                        candidates.push(final_product);
                    }
                }
            }
        }
    }
    
    // Try prime combinations
    for &(prime1, prime2) in &PRIME_COMBINATIONS {
        if let Some(product) = prime1.checked_mul(prime2) {
            candidates.push(product);
            
            // Try squared versions
            if let Some(squared) = prime1.checked_mul(prime1) {
                if let Some(final_product) = squared.checked_mul(prime2) {
                    candidates.push(final_product);
                }
            }
        }
    }
    
    // Add some random prime products based on successful patterns
    for _ in 0..10 {
        if let Some(&(nonce, _)) = history.successful_nonces.iter().choose(&mut rng) {
            let factors = get_prime_factors(nonce);
            if factors.len() >= 2 {
                let idx1 = rng.gen_range(0..factors.len());
                let idx2 = rng.gen_range(0..factors.len());
                if let Some(product) = factors[idx1].checked_mul(factors[idx2]) {
                    candidates.push(product);
                }
            }
        }
    }
    
    candidates.sort();
    candidates.dedup();
    candidates
}

fn analyze_nonce_patterns(nonce: u64, target: u64, hash: u64, pattern_history: &mut HashMap<u64, Vec<u64>>) -> String {
    let nonce_hex = format!("{:016x}", nonce);
    let target_hex = format!("{:016x}", target);
    let hash_hex = format!("{:016x}", hash);
    
    // Get prime factors
    let nonce_factors = get_prime_factors(nonce);
    let hash_factors = get_prime_factors(hash);
    
    // Analyze remainders
    let remainder_625 = nonce % PATTERN_625;
    let remainder_4648 = nonce % PATTERN_4648;
    let remainder_152844 = nonce % PATTERN_152844;
    
    // Record pattern history
    pattern_history.entry(remainder_625).or_default().push(nonce);
    pattern_history.entry(remainder_4648).or_default().push(nonce);
    pattern_history.entry(remainder_152844).or_default().push(nonce);
    
    format!(
        "Nonce Analysis:\n\
        Hex Values:\n\
        Nonce:  {}\n\
        Target: {}\n\
        Hash:   {}\n\n\
        Prime Factorization:\n\
        Nonce factors: {:?}\n\
        Hash factors:  {:?}\n\n\
        Pattern Analysis:\n\
        Remainder mod 625: {} (5^4)\n\
        Remainder mod 4648: {} (2^3 * 7 * 83)\n\
        Remainder mod 152844: {} (2^2 * 3 * 47 * 271)",
        nonce_hex, target_hex, hash_hex,
        nonce_factors,
        hash_factors,
        remainder_625,
        remainder_4648,
        remainder_152844
    )
}

fn mine_block_chunk(block_data: &str, start_nonce: u64, end_nonce: u64, target: u64) -> Option<(u64, u64)> {
    (start_nonce..end_nonce).into_par_iter().find_map(|nonce| {
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}", block_data, nonce));
        let result1 = hasher.finalize();
        
        let mut hasher = Sha256::new();
        hasher.update(result1);
        let final_hash = hasher.finalize();
        
        let hash_val = u64::from_be_bytes(final_hash[0..8].try_into().unwrap());
        
        if hash_val <= target {
            Some((nonce, hash_val))
        } else {
            None
        }
    })
}

fn main() {
    let mut current_target = INITIAL_TARGET;
    let mut results = Vec::new();
    let mut pattern_history = HashMap::new();
    let mut mining_history = MiningHistory::new(10);
    
    println!("Starting mining difficulty test with quantum pattern prediction...");
    println!("{:<20} {:<12} {:<12} {:<15} {:<15} {}", 
        "Target", "Time", "Difficulty", "Hash Rate", "Nonce", "Attempts");
    println!("{:-<90}", "");

    loop {
        let difficulty = (INITIAL_TARGET as f64) / (current_target as f64);
        let block_data = generate_random_block();
        let start_time = Instant::now();
        
        // Try candidate nonces first
        let candidates = generate_candidate_nonces(current_target, &mining_history, difficulty);
        let mut found = false;
        
        for nonce in candidates {
            let mut hasher = Sha256::new();
            hasher.update(format!("{}{}", block_data, nonce));
            let result1 = hasher.finalize();
            
            let mut hasher = Sha256::new();
            hasher.update(result1);
            let final_hash = hasher.finalize();
            
            let hash_val = u64::from_be_bytes(final_hash[0..8].try_into().unwrap());
            
            if hash_val <= current_target {
                mining_history.add_success(nonce, hash_val, difficulty);
                let time_taken = start_time.elapsed();
                
                // Record result
                results.push((current_target, time_taken, difficulty, nonce, 1, hash_val));
                
                // Print result with nonce analysis
                println!("0x{:016x} {:<12} {:<12.2} {:<15} 0x{:016x} {}", 
                    current_target,
                    format_duration(time_taken),
                    difficulty,
                    format_hash_rate(1.0 / time_taken.as_secs_f64()),
                    nonce,
                    1
                );
                println!("{}\n", analyze_nonce_patterns(nonce, current_target, hash_val, &mut pattern_history));
                
                found = true;
                break;
            }
        }
        
        if !found {
            // If no candidates work, try parallel mining
            let chunk_size = 500_000;
            let mut attempts = 0;
            let mut last_status = Instant::now();
            let status_interval = Duration::from_secs(1);
            
            while start_time.elapsed() < Duration::from_secs(60) {
                let chunks: Vec<_> = (0..16).map(|i| {
                    let start = i as u64 * chunk_size;
                    let end = start + chunk_size;
                    (block_data.clone(), start, end, current_target)
                }).collect();
                
                if let Some((nonce, hash_val)) = chunks.par_iter()
                    .find_map(|(data, start, end, target)| {
                        mine_block_chunk(data, *start, *end, *target)
                    }) {
                    let time_taken = start_time.elapsed();
                    attempts += nonce - (nonce / chunk_size * chunk_size);
                    
                    mining_history.add_success(nonce, hash_val, difficulty);
                    
                    // Record result
                    results.push((current_target, time_taken, difficulty, nonce, attempts, hash_val));
                    
                    // Print result with nonce analysis
                    println!("0x{:016x} {:<12} {:<12.2} {:<15} 0x{:016x} {}", 
                        current_target,
                        format_duration(time_taken),
                        difficulty,
                        format_hash_rate(attempts as f64 / time_taken.as_secs_f64()),
                        nonce,
                        attempts
                    );
                    println!("{}\n", analyze_nonce_patterns(nonce, current_target, hash_val, &mut pattern_history));
                    
                    found = true;
                    break;
                }
                
                attempts += chunk_size * 16;
                
                if last_status.elapsed() >= status_interval {
                    let elapsed = start_time.elapsed().as_secs_f64();
                    let hash_rate = attempts as f64 / elapsed;
                    print!("\rMining... {} Attempts: {}", 
                        format_hash_rate(hash_rate), attempts);
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                    last_status = Instant::now();
                }
            }
            
            if !found {
                println!("\nReached timeout at target 0x{:016x}", current_target);
                break;
            }
        }
        
        // Increase difficulty by decreasing target by ~50%
        current_target = (current_target as f64 * 0.5) as u64;
    }
    
    // Analyze patterns across all results
    if results.len() > 1 {
        println!("\nPattern Analysis Across All Results:");
        println!("{:-<90}", "");
        
        // Analyze remainder patterns
        let mut remainder_success = HashMap::new();
        for (_, _, _, nonce, _, _) in &results {
            let r625 = nonce % PATTERN_625;
            let r4648 = nonce % PATTERN_4648;
            let r152844 = nonce % PATTERN_152844;
            
            *remainder_success.entry(r625).or_insert(0) += 1;
            *remainder_success.entry(r4648).or_insert(0) += 1;
            *remainder_success.entry(r152844).or_insert(0) += 1;
        }
        
        // Find most successful remainders
        let mut remainder_vec: Vec<_> = remainder_success.iter().collect();
        remainder_vec.sort_by(|a, b| b.1.cmp(&a.1));
        
        println!("Most Common Successful Remainders:");
        for (remainder, count) in remainder_vec.iter().take(5) {
            println!("Remainder {}: appeared {} times", remainder, count);
        }
        
        // Analyze nonce differences and their prime factors
        println!("\nNonce Pattern Analysis:");
        for window in results.windows(2) {
            if let [(_, _, _, nonce1, _, hash1), (_, _, _, nonce2, _, hash2)] = window {
                let diff = nonce2.wrapping_sub(*nonce1);
                println!("\nNonce change: 0x{:016x} -> 0x{:016x}", nonce1, nonce2);
                println!("Hash change:  0x{:016x} -> 0x{:016x}", hash1, hash2);
                println!("Difference: {} (0x{:x})", diff, diff);
                println!("Prime factors of diff: {:?}", get_prime_factors(diff));
                println!("Remainder patterns:");
                println!("  mod 625: {}", diff % PATTERN_625);
                println!("  mod 4648: {}", diff % PATTERN_4648);
                println!("  mod 152844: {}", diff % PATTERN_152844);
            }
        }
        
        // Print pattern progression analysis
        println!("\nPattern Progression Analysis:");
        for pattern in mining_history.difficulty_patterns {
            println!("Difficulty {:.2}:", pattern.difficulty);
            println!("  Nonce factors: {:?}", pattern.prime_factors);
            println!("  Leading zeros: {}", pattern.leading_zeros);
        }
    }
}
