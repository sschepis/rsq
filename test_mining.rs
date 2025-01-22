use sha2::{Sha256, Digest};
use std::time::{Instant, Duration};
use colored::*;

// Starting with a very easy target for testing
const INITIAL_TARGET: u64 = 0x00FFFFFFFFFFFFFF;

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

fn main() {
    let mut current_target = INITIAL_TARGET;
    let mut results = Vec::new();
    
    println!("\n{}", "Mining Difficulty Test".bright_green().bold());
    println!("{:<20} {:<12} {:<12} {:<15} {}", 
        "Target".cyan(), 
        "Time".cyan(), 
        "Difficulty".cyan(), 
        "Hash Rate".cyan(), 
        "Attempts".cyan());
    println!("{}", "-".repeat(70).dimmed());

    loop {
        match mine_block(current_target, Duration::from_secs(60)) {
            Some((time_taken, attempts)) => {
                // Calculate difficulty (max_target / current_target)
                let difficulty = (INITIAL_TARGET as f64) / (current_target as f64);
                
                // Calculate hash rate
                let hash_rate = attempts as f64 / time_taken.as_secs_f64();
                
                // Record result
                results.push((current_target, time_taken, difficulty, attempts));
                
                // Print result with colors
                println!("{} {} {} {} {}", 
                    format!("0x{:016x}", current_target).yellow(),
                    format!("{:<12}", format_duration(time_taken)).green(),
                    format!("{:<12.2}", difficulty).cyan(),
                    format!("{:<15}", format_hash_rate(hash_rate)).blue(),
                    attempts.to_string().white()
                );
                
                // Increase difficulty by decreasing target by ~50%
                current_target = (current_target as f64 * 0.5) as u64;
            }
            None => {
                println!("\n{}", format!("Reached timeout at target 0x{:016x}", current_target).red());
                break;
            }
        }
    }
    
    // Calculate and display statistics
    if !results.is_empty() {
        println!("\n{}", "Results Analysis:".bright_green().bold());
        println!("{}", "-".repeat(70).dimmed());
        
        // Find the highest difficulty we achieved
        let max_difficulty = results.iter()
            .map(|(_, _, diff, _)| diff)
            .fold(0f64, |a, b| a.max(*b));
            
        // Calculate average hash rate
        let total_hashes: u64 = results.iter().map(|(_, _, _, attempts)| attempts).sum();
        let total_time: f64 = results.iter()
            .map(|(_, time, _, _)| time.as_secs_f64())
            .sum();
        let avg_hash_rate = total_hashes as f64 / total_time;
            
        // Calculate expected time for pool difficulty
        if let Some((_, time, diff, _)) = results.last() {
            let pool_difficulty = 1_000_000f64; // Example pool difficulty
            let expected_time = time.as_secs_f64() * (pool_difficulty / diff);
            
            println!("{}: {}", "Maximum difficulty achieved".cyan(), 
                format!("{:.2}", max_difficulty).yellow());
            println!("{}: {}", "Average hash rate".cyan(), 
                format_hash_rate(avg_hash_rate).green());
            println!("{}: {}", "Total hashes".cyan(), 
                total_hashes.to_string().yellow());
            println!("{}: {}", "Total time".cyan(), 
                format_duration(Duration::from_secs_f64(total_time)).green());
            println!("{} {}: {}", 
                "Expected time for pool difficulty".cyan(),
                pool_difficulty.to_string().yellow(),
                format_duration(Duration::from_secs_f64(expected_time)).green());
        }
    }
}

fn mine_block(target: u64, timeout: Duration) -> Option<(Duration, u64)> {
    let start_time = Instant::now();
    let mut nonce: u64 = 0;
    let mut last_status = Instant::now();
    let status_interval = Duration::from_secs(1);
    
    // Create some mock data for the block
    let mock_block = "test block data";
    
    while start_time.elapsed() < timeout {
        // Show mining progress every second
        if last_status.elapsed() >= status_interval {
            let elapsed = start_time.elapsed().as_secs_f64();
            let hash_rate = nonce as f64 / elapsed;
            print!("\r{} {} {} {}", 
                "Mining...".blue(),
                format_hash_rate(hash_rate).yellow(),
                "Attempts:".blue(),
                nonce.to_string().yellow());
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            last_status = Instant::now();
        }

        let mut hasher = Sha256::new();
        
        // Double SHA256 as per Bitcoin protocol
        hasher.update(format!("{}{}", mock_block, nonce));
        let result1 = hasher.finalize();
        
        let mut hasher = Sha256::new();
        hasher.update(result1);
        let final_hash = hasher.finalize();
        
        // Convert first 8 bytes of hash to u64 for comparison
        let hash_val = u64::from_be_bytes(final_hash[0..8].try_into().unwrap());
        
        if hash_val <= target {
            print!("\r{}{}\n", 
                " ".repeat(70), // Clear the progress line
                format!("Found valid nonce after {} attempts!", nonce).green());
            return Some((start_time.elapsed(), nonce));
        }
        
        nonce += 1;
    }
    
    print!("\r{}{}\n",
        " ".repeat(70), // Clear the progress line
        format!("Timeout reached after {} attempts", nonce).red());
    None
}
