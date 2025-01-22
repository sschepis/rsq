pub mod mining;
pub mod quantum;

// Re-export commonly used items
pub use mining::{QuantumMiner, HashAlgorithm, HashFunction};
pub use quantum::state::PrimeQuantumState;
