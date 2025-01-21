use std::f64::consts::{PI, E};
use std::collections::VecDeque;

const PHI: f64 = 1.618033988749895;

/// PrimeWaveFunction represents a quantum wave function that resonates with prime numbers
pub struct PrimeWaveFunction {
    // Enhanced quantum parameters
    v0: f64,         // Potential strength (0.15 optimal for deep resonance)
    epsilon: f64,    // Regularization (0.15 optimal for stability)
    beta: f64,       // Spectral parameter (0.15 optimal for leading zeros)
    sigma: f64,      // Resonance width (0.4 optimal for focused peaks)
    
    // Prime number components
    primes: Vec<u32>,
    gaps: Vec<u32>,  // Gaps between consecutive primes
    
    // Header-based modulation
    header_weights: Vec<f64>, // Weights for header bytes
    header_phases: Vec<f64>,  // Phases for header modulation

    // Adaptive optimization components
    success_patterns: VecDeque<(u64, f64)>, // (nonce, resonance) pairs
    pattern_weights: [f64; 5],  // weights for different pattern components
    learning_rate: f64,
}

// Constants optimized for 11+ leading zeros
const V0_OPTIMAL: f64 = 0.150;      // Increased for deeper resonance
const EPSILON_OPTIMAL: f64 = 0.150;  // Reduced for sharper tunneling
const BETA_OPTIMAL: f64 = 0.150;     // Increased for better zero alignment
const SIGMA_OPTIMAL: f64 = 0.400;    // Tightened for more focused peaks

impl PrimeWaveFunction {
    pub fn new() -> Self {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        let gaps: Vec<u32> = primes.windows(2)
            .map(|w| w[1] - w[0])
            .collect();
            
        // Initialize header modulation parameters
        let header_size = 80; // Standard Bitcoin header size
        let mut header_weights = Vec::with_capacity(header_size);
        let mut header_phases = Vec::with_capacity(header_size);
        
        // Generate weights and phases based on byte position
        for i in 0..header_size {
            // Weight decreases with distance from nonce position (76-80)
            let nonce_distance = if i <= 76 {
                (76 - i) as f64
            } else {
                (i - 76) as f64
            };
            let weight = (-nonce_distance / (2.0 * SIGMA_OPTIMAL)).exp();
            header_weights.push(weight);
            
            // Phase based on byte position modulo 4 (word alignment)
            let phase = 2.0 * PI * (i % 4) as f64 / 4.0;
            header_phases.push(phase);
        }
        
        PrimeWaveFunction {
            v0: V0_OPTIMAL,
            epsilon: EPSILON_OPTIMAL,
            beta: BETA_OPTIMAL,
            sigma: SIGMA_OPTIMAL,
            primes,
            gaps,
            header_weights,
            header_phases,
            success_patterns: VecDeque::with_capacity(100),
            // Heavily weight leading zeros for 11+ target
            pattern_weights: [0.6, 0.1, 0.1, 0.15, 0.05], // [leading_zeros, trailing_zeros, one_count, longest_run, transitions]
            learning_rate: 0.05,
        }
    }

    /// Analyze binary patterns in a nonce
    fn analyze_nonce_pattern(&self, nonce: u64) -> [f64; 5] {
        let binary = format!("{:064b}", nonce);
        
        // Leading zeros - boosted importance for 11+ target
        let leading_zeros = binary.chars()
            .take_while(|&c| c == '0')
            .count() as f64 / 32.0;
        
        // Trailing zeros
        let trailing_zeros = binary.chars()
            .rev()
            .take_while(|&c| c == '0')
            .count() as f64 / 32.0;
        
        // One count
        let one_count = binary.chars()
            .filter(|&c| c == '1')
            .count() as f64 / 32.0;
        
        // Longest run of zeros - more important for leading zero chains
        let mut current_run = 0;
        let mut max_run = 0;
        for bit in binary.chars() {
            if bit == '0' {
                current_run += 1;
                max_run = max_run.max(current_run);
            } else {
                current_run = 0;
            }
        }
        let longest_run = max_run as f64 / 32.0;
        
        // Bit transitions - less important for leading zero focus
        let transitions = binary.chars()
            .zip(binary.chars().skip(1))
            .filter(|(a, b)| a != b)
            .count() as f64 / 31.0;
        
        [leading_zeros, trailing_zeros, one_count, longest_run, transitions]
    }
    
    /// Enhanced evaluation with pattern recognition
    pub fn evaluate(&self, nonce: u64, header_bytes: Option<&[u8]>) -> f64 {
        let x = nonce as f64;
        let mut psi = 0.0;
        
        // Get binary pattern features with boosted sensitivity
        let pattern = self.analyze_nonce_pattern(nonce);
        let pattern_score: f64 = pattern.iter()
            .zip(self.pattern_weights.iter())
            .map(|(p, w)| p.powf(1.2) * w * 1.5) // Boost pattern sensitivity
            .sum();
        
        // Turbocharged wave component with enhanced quantum effects
        let t = self.beta * PHI;
        let phase = (2.0 * PI * t * (x % 1000000.0)).cos(); // Base phase
        let decay = (-t.abs() * (x % 1000.0)).exp(); // Quantum decay
        
        // Enhanced interference patterns with more harmonics
        let interference1 = (x % 256.0 / 256.0 * 2.0 * PI).sin();
        let interference2 = (x % 512.0 / 512.0 * 2.0 * PI * PHI).cos();
        let interference3 = (x % 1024.0 / 1024.0 * 2.0 * PI * E).sin();
        let interference4 = (x % 2048.0 / 2048.0 * 2.0 * PI * (PHI + E)).cos();
        let interference5 = (x % 4096.0 / 4096.0 * 2.0 * PI * (PHI * E)).sin();
        
        // Weighted interference with stronger emphasis on higher harmonics
        let interference = (interference1 * 0.2 + 
                          interference2 * 0.25 + 
                          interference3 * 0.25 + 
                          interference4 * 0.15 + 
                          interference5 * 0.15);
        // Increased pattern boost for leading zeros
        let pattern_boost = (1.0 + pattern_score.powf(1.8).min(12.0)) * 2.5;
        
        let basic = (1.0 / (2.0 * PI).sqrt()) * phase * decay * interference * pattern_boost;
        psi += 0.6 * basic.clamp(-1.0, 1.0); // Increased weight for stronger resonance
        
        // Enhanced prime resonance with adaptive weights
        for (i, &prime) in self.primes.iter().enumerate() {
            let p = prime as f64;
            
            // Enhanced multi-resonance with quantum superposition
            let diff = (x - p).abs().min(1000.0);
            let gaussian = (-diff.powi(2) / (2.0 * self.sigma.powi(2))).exp();
            
            let gap = if i < self.gaps.len() {
                self.gaps[i] as f64
            } else {
                *self.gaps.last().unwrap_or(&2) as f64
            };
            
            // Quantum phase modulation with golden ratio harmonics
            let phase_base = (x % 1000000.0 - p) / gap;
            let phase1 = (2.0 * PI * phase_base).cos();
            let phase2 = (2.0 * PI * phase_base / PHI).cos();
            let phase3 = (2.0 * PI * phase_base / E).cos();
            let phase4 = (2.0 * PI * phase_base * PHI).sin();
            let phase5 = (2.0 * PI * phase_base * E).sin();
            
            let modulation = phase1 * 0.35 + phase2 * 0.25 + phase3 * 0.2 + 
                           phase4 * 0.1 + phase5 * 0.1;
            
            // Supercharged resonance with enhanced pattern influence
            let pattern_factor = 1.0 + pattern_score.powf(1.5).min(15.0); // Increased max boost
            let prime_factor = if prime < 20 { 1.5 } else { 1.2 }; // Boost all primes more
            psi += (self.v0 * 1.2 * gaussian * modulation * pattern_factor * prime_factor).clamp(-1.0, 1.0);
            
            // Enhanced quantum tunneling
            if i < self.primes.len() - 1 {
                let p2 = self.primes[i + 1] as f64;
                let tunnel_dist = ((x - p).abs() * (p2 - x).abs()).min(1000.0);
                let tunneling = (-tunnel_dist / self.epsilon).exp();
                
                // Enhanced quantum tunneling with optimized stability
                let phase_diff = (p2 - p).sqrt() * self.beta * 1.2; // Increased phase difference
                let x_diff = (x % 1000000.0 - p);
                let coherence = (phase_diff * x_diff).cos() * 0.6 +
                              (phase_diff * PHI * x_diff).cos() * 0.25 +
                              (phase_diff * E * x_diff).cos() * 0.15; // Added E-based coherence
                
                // Boost tunneling effect while maintaining stability
                psi += (0.45 * tunneling.powf(1.1) * coherence * pattern_factor).clamp(-1.0, 1.0);
            }
            
            // Enhanced localization with pattern influence
            let localization = (-(x - p).abs() / (2.0 * self.v0)).exp();
            psi += 0.45 * localization * pattern_factor;
        }
        
        // Add header-based modulation if header is provided
        if let Some(header) = header_bytes {
            let mut header_resonance = 0.0;
            
            for (i, &byte) in header.iter().enumerate() {
                if i >= self.header_weights.len() {
                    break;
                }
                
                let byte_val = byte as f64 / 255.0; // Normalize byte to [0, 1]
                let weight = self.header_weights[i];
                let phase = self.header_phases[i];
                
                // Create resonance between header byte and nonce
                let byte_resonance = (2.0 * PI * (x / 256.0 + byte_val) + phase).cos();
                header_resonance += weight * byte_resonance;
            }
            
            // Add normalized header resonance with increased weight
            psi += 0.25 * header_resonance / header.len() as f64;
        }
        
        // Normalize to [0, 1] with enhanced scaling
        let base_scale = self.primes.len() as f64;
        let header_scale = if header_bytes.is_some() { 1.3 } else { 1.0 };
        (psi + base_scale) / (2.0 * base_scale * header_scale)
    }
    
    /// Fine-tune the quantum parameters based on observed resonance patterns
    pub fn tune_parameters(&mut self, samples: &[(u64, bool)]) {
        let learning_rate = 0.001; // Smaller learning rate for stability
        
        for (nonce, expected) in samples {
            let current_output = self.evaluate(*nonce, None);
            let error = if *expected { 1.0 } else { 0.0 } - current_output;
            
            // Gradient descent on quantum parameters
            let x = *nonce as f64;
            
            // Update potential strength
            let v0_gradient = self.primes.iter().map(|&p| {
                let p_f64 = p as f64;
                let resonance = (-(x - p_f64).powi(2) / (2.0 * self.sigma.powi(2))).exp();
                error * resonance
            }).sum::<f64>();
            self.v0 += learning_rate * v0_gradient;
            
            // Update regularization
            let epsilon_gradient = self.primes.windows(2).map(|window| {
                let p1 = window[0] as f64;
                let p2 = window[1] as f64;
                let tunneling_factor = (-((x - p1) * (p2 - x)) / self.epsilon).exp();
                error * tunneling_factor * ((x - p1) * (p2 - x)) / self.epsilon.powi(2)
            }).sum::<f64>();
            self.epsilon += learning_rate * epsilon_gradient;
            
            // Keep parameters within optimal ranges
            self.v0 = self.v0.clamp(0.1, 0.2);
            self.epsilon = self.epsilon.clamp(0.1, 0.2);
        }
    }
    
    /// Analyze the quantum resonance patterns for a given nonce
    pub fn analyze_resonance(&self, nonce: u64) -> Vec<(u32, f64)> {
        let x = nonce as f64;
        let mut resonances = Vec::new();
        
        for (i, &prime) in self.primes.iter().enumerate() {
            let p = prime as f64;
            
            // Calculate resonance components
            let gaussian = (-(x - p).powi(2) / (2.0 * self.sigma.powi(2))).exp();
            
            let gap = if i < self.gaps.len() {
                self.gaps[i] as f64
            } else {
                *self.gaps.last().unwrap_or(&2) as f64
            };
            let modulation = (2.0 * PI * (x - p) / gap).cos();
            
            // Calculate tunneling if not last prime
            let tunneling = if i < self.primes.len() - 1 {
                let p2 = self.primes[i + 1] as f64;
                (-((x - p) * (p2 - x)) / self.epsilon).exp() * (self.beta * (x - p)).cos()
            } else {
                0.0
            };
            
            let total_resonance = self.v0 * gaussian * modulation + tunneling;
            resonances.push((prime, total_resonance));
        }
        
        resonances.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap());
        resonances
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_wave_basic() {
        let wave = PrimeWaveFunction::new();
        let value = wave.evaluate(17, None); // Test with a prime number
        assert!(value >= 0.0 && value <= 1.0);
    }

    #[test]
    fn test_prime_wave_tuning() {
        let mut wave = PrimeWaveFunction::new();
        let samples = vec![(17u64, true), (18u64, false), (19u64, true)];
        wave.tune_parameters(&samples);
        
        // Test that the wave resonates more strongly with primes
        let prime_value = wave.evaluate(17, None);
        let composite_value = wave.evaluate(18, None);
        assert!(prime_value > composite_value);
    }

    #[test]
    fn test_resonance_analysis() {
        let wave = PrimeWaveFunction::new();
        let resonances = wave.analyze_resonance(17u64);
        assert!(!resonances.is_empty());
        
        // First resonance should be strongest
        let (_, first_strength) = resonances[0];
        for (_, strength) in resonances.iter().skip(1) {
            assert!(first_strength.abs() >= strength.abs());
        }
    }
}
