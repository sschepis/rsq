use crate::quantum::core::complex::Complex;
use std::sync::OnceLock;
use std::time::{Instant, Duration};

// Enhanced prime generation strategy
static PRIME_CACHE: OnceLock<Vec<u32>> = OnceLock::new();

const _MAX_PRIMES: usize = 2048; // Increased cache size
const _ADVANCED_TUNNELING_STRENGTH: f64 = 0.7; // More aggressive tunneling
const _ADVANCED_CORRELATION_STRENGTH: f64 = 0.3; // Enhanced correlation
const _PHASE_ALIGNMENT_ADVANCED_ROUNDS: usize = 6; // More alignment iterations

#[derive(Debug)]
pub struct QuantumStatePerformance {
    initialization_time: Duration,
    resonance_computation_time: Duration,
    total_computations: usize,
}

impl Default for QuantumStatePerformance {
    fn default() -> Self {
        Self {
            initialization_time: Duration::new(0, 0),
            resonance_computation_time: Duration::new(0, 0),
            total_computations: 0,
        }
    }
}

pub struct PrimeQuantumState {
    state: Vec<Complex>,
    resolution: usize,
    amplitude_boost: Complex,
    interference_weights: Vec<f64>,
    _phase_cache: Vec<Complex>,
    performance_profile: QuantumStatePerformance,
    adaptive_resolution: bool,
}

impl PrimeQuantumState {
    pub fn new(resolution: usize) -> Self {
        let start_time = Instant::now();
        
        let mut state = Vec::with_capacity(resolution);
        let mut interference_weights = Vec::with_capacity(resolution);
        let mut _phase_cache = Vec::with_capacity(resolution);
        
        // Advanced phase initialization with more sophisticated weighting
        for i in 0..resolution {
            let phase = 2.0 * std::f64::consts::PI * (i as f64).powf(1.2) / (resolution as f64);
            let phase_factor = Complex::new(phase.cos(), phase.sin());
            _phase_cache.push(phase_factor);
            state.push(phase_factor);
            
            // Non-linear interference weight calculation
            interference_weights.push(1.0 / (i + 1) as f64 * (1.0 + (i as f64).ln()));
        }
        
        let initialization_time = start_time.elapsed();
        
        PrimeQuantumState {
            state,
            resolution,
            amplitude_boost: Complex::new(1.0, 0.0),
            interference_weights,
            _phase_cache,
            performance_profile: QuantumStatePerformance {
                initialization_time,
                ..Default::default()
            },
            adaptive_resolution: false,
        }
    }

    pub fn initialize_with_primes(&mut self, target_zeros: u32) {
        let start_time = Instant::now();
        
        // Dynamic prime generation with segmented sieve
        let primes = PRIME_CACHE.get_or_init(|| {
            let mut primes = Vec::with_capacity(_MAX_PRIMES);
            let mut sieve = vec![true; _MAX_PRIMES * 10];
            sieve[0] = false;
            sieve[1] = false;
            
            for i in 2..sieve.len() {
                if sieve[i] {
                    primes.push(i as u32);
                    for j in (i * i..sieve.len()).step_by(i) {
                        sieve[j] = false;
                    }
                    
                    if primes.len() >= _MAX_PRIMES {
                        break;
                    }
                }
            }
            primes
        });
        
        // Adaptive resolution based on target difficulty
        self.adaptive_resolution = target_zeros > 3;
        let effective_resolution = if self.adaptive_resolution {
            self.resolution * (1 + target_zeros as usize)
        } else {
            self.resolution
        };
        
        // Enhanced quantum state initialization
        for i in 0..effective_resolution {
            let prime = primes[i % primes.len()] as f64;
            let phase = 2.0 * std::f64::consts::PI * prime.powf(1.3) / (effective_resolution as f64);
            
            // Advanced phase generation with quantum interference
            let quantum_phase = Complex::new(
                (phase * 1.5).cos(), 
                (phase * 1.5).sin()
            );
            
            if i < self.state.len() {
                self.state[i] = quantum_phase;
            } else {
                self.state.push(quantum_phase);
            }
        }
        
        // Dynamic amplitude boost with non-linear scaling
        let boost = 1.0 + (target_zeros as f64).powf(1.5) * 0.2;
        self.amplitude_boost = Complex::new(boost, 0.0);
        
        let initialization_time = start_time.elapsed();
        self.performance_profile.initialization_time = initialization_time;
    }

    pub fn apply_prime_resonance(&mut self, t: &[f64]) -> Vec<Complex> {
        let start_time = Instant::now();
        
        let mut resonance = vec![Complex::new(0.0, 0.0); t.len()];
        let _resolution_f64 = self.resolution as f64;
        
        for (i, &time) in t.iter().enumerate() {
            let mut quantum_pattern = Complex::new(0.0, 0.0);
            
            // Advanced quantum interference calculation
            for (j, &state) in self.state.iter().enumerate() {
                let weight = self.interference_weights[j];
                let interference_factor = Complex::new(
                    weight * _ADVANCED_CORRELATION_STRENGTH * time.sin(), 
                    0.0
                );
                quantum_pattern = quantum_pattern + state * interference_factor;
            }
            
            resonance[i] = quantum_pattern * self.amplitude_boost;
        }
        
        let resonance_time = start_time.elapsed();
        self.performance_profile.resonance_computation_time = resonance_time;
        self.performance_profile.total_computations += 1;
        
        resonance
    }

    pub fn get_performance_profile(&self) -> &QuantumStatePerformance {
        &self.performance_profile
    }
}

// Retained existing prime checking function
fn _is_prime(n: u32) -> bool {
    if n <= 1 { return false; }
    if n <= 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }
    
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 { return false; }
        i += 6;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advanced_prime_resonance() {
        let mut state = PrimeQuantumState::new(32);
        state.initialize_with_primes(4);
        let t = vec![0.1, 0.2, 0.3];
        let resonance = state.apply_prime_resonance(&t);
        
        assert_eq!(resonance.len(), t.len());
        
        // Performance profile checks
        let profile = state.get_performance_profile();
        assert!(profile.initialization_time.as_nanos() > 0);
        assert!(profile.resonance_computation_time.as_nanos() > 0);
        assert_eq!(profile.total_computations, 1);
    }

    #[test]
    fn test_adaptive_initialization() {
        let mut state = PrimeQuantumState::new(32);
        state.initialize_with_primes(5);
        
        // Check that state adapted to higher difficulty
        assert!(state.state.len() > 32);
    }
}
