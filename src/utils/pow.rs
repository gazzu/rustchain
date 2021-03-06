use crate::rustchain::block::{HASH_BIT_SIZE, Block, Sha256Hash};
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use num_bigint::BigUint;
use num_traits::One;
use crate::utils::util;

// How many leading zeroes the hex hash should have in order to be considered valid.
pub const DEFAULT_DIFFICULTY: usize = 5;
// TODO use exponentiation, 2 ** 32
const MAX_NONCE: u64 = 10_000_000;

// Runs the POW algorithm on a block, i.e. it calculates HASH(header + nonce) until the hash
// fulfills the difficulty requirement.
//
// Returns the nonce.
// TODO only pass headers
pub fn run(block: &Block, difficulty: usize) -> Option<u64> {
    println!("Mining block containing \"{}\"", &block.pretty_data());
    // The target is a number we compare the hash to. It is a HASH_BIT_SIZE bit binary with
    // DEFAULT_DIFFICULTY * 4 leading zeroes, which translated to hex means DEFAULT_DIFFICULTY
    // zeroes.
    let target = BigUint::one() << (HASH_BIT_SIZE - difficulty * 4);

    for nonce in 0..MAX_NONCE {
        let hash = calculate_hash(&block, nonce);
        let hash_int = BigUint::from_bytes_be(&hash);

        if hash_int < target {
            return Some(nonce);
        }
    }

    None

}

pub fn calculate_hash(block: &Block, nonce: u64) -> Sha256Hash {
    let mut headers = block.headers();
    headers.extend_from_slice(&util::convert_u64_to_u8_array(nonce));

    // TODO: removed this for now, as it's not in the struct
    // headers.push(DIFFICULTY_BITS as u8);

    // TODO parallelize
    let mut hasher = Sha256::new();
    hasher.input(&headers);
    let mut hash = Sha256Hash::default();

    hasher.result(&mut hash);

    hash
}