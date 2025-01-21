use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use std::time::Instant;
use rsq::quantum::resonance::PrimeWaveFunction;

const TARGET_DIFFICULTY: u64 = 663511;
const RESONANCE_THRESHOLDS: [(u32, f64); 6] = [
    (6, 0.7),   // 6 zeros - baseline
    (7, 0.78),  // 7 zeros - increased sensitivity
    (8, 0.85),  // 8 zeros - critical phase transition point
    (9, 0.88),  // 9 zeros - post-transition stability
    (10, 0.92), // 10 zeros - high precision zone
    (11, 0.95), // 11 zeros - quantum supremacy threshold
];

// Phase transition parameters
const QUANTUM_BOOST_BASE: f64 = 1.15;  // Base quantum amplification
const STABILITY_THRESHOLD: f64 = 0.82; // Minimum stability for reliable mining
const PHASE_TRANSITION_ZONE: f64 = 0.05; // 5% variance window for transition

fn get_resonance_threshold(zeros: u32) -> f64 {
    RESONANCE_THRESHOLDS
        .iter()
        .find(|(z, _)| *z == zeros)
        .map(|(_, t)| *t)
        .unwrap_or(0.7) // default threshold
}

fn analyze_block_patterns(line: &str, difficulty_range: &std::ops::Range<u64>) -> Option<(u64, Vec<u8>, u64, u64, u128)> {
    // Returns (nonce, header, difficulty, bits, target)
    let fields: Vec<&str> = line.split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    
    if fields.len() < 7 {
        return None;
    }

    // Extract fields from CSV format
    let prev_block = match hex::decode(fields[1].trim_matches('"')) {
        Ok(prev) => prev,
        Err(_) => return None
    };
    
    let merkle_root = match hex::decode(fields[2].trim_matches('"')) {
        Ok(root) => root,
        Err(_) => return None
    };
    
    let timestamp = match fields[3].trim_matches('"').parse::<u64>() {
        Ok(ts) => ts,
        Err(_) => return None
    };
    
    let bits = match u64::from_str_radix(fields[4].trim_matches('"'), 16) {
        Ok(b) => b,
        Err(_) => return None
    };
    
    let nonce = match u64::from_str_radix(fields[5].trim_matches('"'), 16) {
        Ok(n) => n,
        Err(_) => return None
    };
    
    // Construct header
    let mut header = Vec::with_capacity(80);
    header.extend_from_slice(&(2u32).to_le_bytes()); // version, 4 bytes
    header.extend_from_slice(&prev_block); // 32 bytes
    header.extend_from_slice(&merkle_root); // 32 bytes
    header.extend_from_slice(&timestamp.to_le_bytes()); // 8 bytes
    header.extend_from_slice(&(bits as u32).to_le_bytes()); // 4 bytes
    header.extend_from_slice(&nonce.to_le_bytes()); // 8 bytes
    
    // Convert bits to target and calculate difficulty using Bitcoin's compact format
    let exp = ((bits >> 24) & 0xff) as u32;
    let mantissa = (bits & 0x007fffff) as u128;
    
    let difficulty_1_target: u128 = 0x00000000ffff0000;
    
    let target = if exp <= 3 {
        mantissa >> (8 * (3 - exp))
    } else if exp >= 32 {
        return Some((nonce, header, 1, bits, 0));
    } else {
        if 8 * (exp - 3) >= 128 {
            return Some((nonce, header, 1, bits, 0));
        }
        mantissa << (8 * (exp - 3))
    };
    
    let difficulty = if target == 0 {
        1
    } else {
        let diff = (difficulty_1_target as f64 / target as f64).ceil();
        if diff.is_finite() && diff <= u64::MAX as f64 {
            diff as u64
        } else {
            1
        }
    };
    
    // Only output blocks in target range
    if difficulty_range.contains(&difficulty) {
        println!("🎯 Block found: diff={}, nonce={:#x}", difficulty, nonce);
    }
    
    Some((nonce, header, difficulty, bits, target))
}

fn main() -> io::Result<()> {
    println!("🌊 Starting Quantum Resonance Analysis 🏄‍♂️");
    println!("Target difficulty: {} (±10%)", TARGET_DIFFICULTY);
    
    let wave_func = PrimeWaveFunction::new();
    let mut patterns_by_diff: HashMap<u64, Vec<(u64, f64)>> = HashMap::new();
    let mut total_blocks = 0;
    let mut high_resonance_blocks = 0;
    
    let file = File::open("blocks.csv")?;
    let mut reader = BufReader::new(file);
    let start = Instant::now();
    
    let difficulty_range = (TARGET_DIFFICULTY as f64 * 0.9) as u64..(TARGET_DIFFICULTY as f64 * 1.1) as u64;
    
    let mut buffer = Vec::new();
    let mut i = 0;
    
    // Skip header
    reader.read_until(b'\n', &mut buffer)?;
    buffer.clear();
    
    while reader.read_until(b'\n', &mut buffer)? > 0 {
        i += 1;
        
        let result = {
            let line = String::from_utf8_lossy(&buffer);
            analyze_block_patterns(&line, &difficulty_range)
        };
        
        buffer.clear();
        
        if let Some((nonce, header, difficulty, bits, target)) = result {
            total_blocks += 1;
            
            // Calculate leading zeros from difficulty
            let zeros = (*&difficulty as f64).log2().floor() as u32;
            let threshold = get_resonance_threshold(zeros);
            
            // Enhanced resonance calculation with adaptive quantum boost
            let base_resonance = wave_func.evaluate(nonce, Some(&header));
            
            // Calculate phase transition proximity
            let target_zeros = (TARGET_DIFFICULTY as f64).log2().floor();
            let current_zeros = (difficulty as f64).log2().floor();
            let phase_distance = (current_zeros - target_zeros).abs() / target_zeros;
            
            // Adaptive quantum boost based on phase transition proximity
            let quantum_boost = if zeros >= 7 {
                let phase_factor = QUANTUM_BOOST_BASE + (zeros as f64 - 6.0) * 0.18;
                let transition_boost = if phase_distance < PHASE_TRANSITION_ZONE {
                    // Apply additional boost in transition zone
                    1.0 + (PHASE_TRANSITION_ZONE - phase_distance) * 2.0
                } else {
                    1.0
                };
                base_resonance * phase_factor * transition_boost
            } else {
                base_resonance
            };
            
            let is_target_range = difficulty_range.contains(&difficulty);
            if quantum_boost > threshold || is_target_range {
                high_resonance_blocks += 1;
                
                if is_target_range {
                    println!("\n🎯 Target Range Block Analysis:");
                    println!("Resonance: {:.4} | Difficulty: {} | Nonce: {:#x}", 
                        quantum_boost, difficulty, nonce);
                    
                    patterns_by_diff.entry(difficulty)
                        .or_default()
                        .push((nonce, quantum_boost));
                }
            }
            
            patterns_by_diff
                .entry(difficulty)
                .or_default()
                .push((nonce, quantum_boost));
            
            if i % 100000 == 0 {
                print!(".");
            }
        }
    }
    
    println!("\n\n📊 Analysis Summary:");
    println!("Total blocks: {} | High resonance: {} ({:.2}%)", 
        total_blocks, 
        high_resonance_blocks,
        (high_resonance_blocks as f64 / total_blocks as f64) * 100.0);
    
    println!("\n📈 Enhanced Resonance Analysis:");
    println!("Phase transition analysis for 7-8 zero barrier:");
    let mut difficulties: Vec<_> = patterns_by_diff.keys()
        .filter(|&&d| difficulty_range.contains(&d))
        .collect();
    difficulties.sort();
    
    for &diff in &difficulties {
        if let Some(patterns) = patterns_by_diff.get(&diff) {
            let resonances: Vec<f64> = patterns.iter().map(|(_, r)| *r).collect();
            let avg_resonance = resonances.iter().sum::<f64>() / resonances.len() as f64;
            let max_resonance = resonances.iter().fold(0.0f64, |a, &b| a.max(b));
            let std_dev = (resonances.iter()
                .map(|x| (x - avg_resonance).powi(2))
                .sum::<f64>() / resonances.len() as f64)
                .sqrt();
            
            // Calculate quantum stability metric
            let zeros = (*diff as f64).log2().floor() as u32;
            let stability = 1.0 - (std_dev / avg_resonance);
            
            println!("Difficulty {} ({} zeros):", diff, zeros);
            println!("  Blocks: {} | Avg: {:.4} | Max: {:.4}", patterns.len(), avg_resonance, max_resonance);
            println!("  Std Dev: {:.4} | Stability: {:.4}", std_dev, stability);
            
            // Enhanced phase transition analysis
            if zeros >= 7 {
                let transition_risk = 1.0 - stability;
                let is_critical = stability < STABILITY_THRESHOLD;
                let in_transition = phase_distance < PHASE_TRANSITION_ZONE;
                
                if is_critical || in_transition {
                    println!("  ⚠️ Phase Transition Analysis:");
                    println!("    → Stability Risk: {:.2}%", transition_risk * 100.0);
                    println!("    → Transition Zone: {}", if in_transition { "ACTIVE" } else { "inactive" });
                    if is_critical {
                        println!("    💡 Recommendation: Increase quantum boost to {:.3}", 
                            QUANTUM_BOOST_BASE * (1.0 + transition_risk));
                    }
                }
            }
        }
    }
    
    let best_diff = difficulties.into_iter()
        .max_by(|&&a, &&b| {
            let avg_a = patterns_by_diff[&a].iter().map(|(_, r)| *r).sum::<f64>() 
                / patterns_by_diff[&a].len() as f64;
            let avg_b = patterns_by_diff[&b].iter().map(|(_, r)| *r).sum::<f64>() 
                / patterns_by_diff[&b].len() as f64;
            avg_a.partial_cmp(&avg_b).unwrap()
        })
        .unwrap();
    
    println!("\n🏆 Best performing difficulty: {}", best_diff);
    
    Ok(())
}
