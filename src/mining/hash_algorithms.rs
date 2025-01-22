use sha2::{Sha256, Digest};
use scrypt::{Params as ScryptParams, scrypt};
use std::io;

#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    Sha256,
    Equihash,
    Scrypt,
}

impl HashAlgorithm {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sha256" => Some(HashAlgorithm::Sha256),
            "equihash" => Some(HashAlgorithm::Equihash),
            "scrypt" => Some(HashAlgorithm::Scrypt),
            _ => None,
        }
    }
}

pub trait HashFunction: std::any::Any + Send + Sync {
    fn hash(&self, data: &[u8]) -> Vec<u8>;
    fn verify(&self, data: &[u8], target: &[u8]) -> bool;
}

pub struct Sha256Hash;
impl HashFunction for Sha256Hash {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let hash1 = Sha256::digest(data);
        let final_hash = Sha256::digest(&hash1);
        final_hash.to_vec()
    }

    fn verify(&self, data: &[u8], target: &[u8]) -> bool {
        let hash = self.hash(data);
        // Compare hash to target - hash must be less than target for valid solution
        for (h, t) in hash.iter().zip(target.iter()) {
            if h < t { return true; }
            if h > t { return false; }
        }
        true // Equal is valid
    }
}


#[derive(Clone)]
pub struct ScryptHash {
    params: ScryptParams,
}

impl ScryptHash {
    pub fn new(n: u32, r: u32, p: u32) -> io::Result<Self> {
        let log_n = n.ilog2().try_into().map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidInput, "Invalid n parameter")
        })?;
        let params = ScryptParams::new(log_n, r, p, 32)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;
        Ok(Self { params })
    }
}

impl HashFunction for ScryptHash {
    fn hash(&self, data: &[u8]) -> Vec<u8> {
        let mut output = vec![0u8; 32];
        scrypt(data, &[], &self.params, &mut output).unwrap_or_default();
        output
    }

    fn verify(&self, data: &[u8], target: &[u8]) -> bool {
        let hash = self.hash(data);
        &hash[..target.len()] == target
    }
}

pub fn create_hash_function(algorithm: HashAlgorithm) -> Box<dyn HashFunction> {
    match algorithm {
        HashAlgorithm::Sha256 => Box::new(Sha256Hash),
        HashAlgorithm::Equihash => unimplemented!("Equihash support temporarily disabled"),
        HashAlgorithm::Scrypt => Box::new(ScryptHash::new(1024, 1, 1).expect("Failed to create ScryptHash")), // N=1024, r=1, p=1 are common Scrypt parameters
    }
}
