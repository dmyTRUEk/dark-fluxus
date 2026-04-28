//! utils

pub fn hash_str_to_u64(s: &str) -> u64 {
	// FNV-1a hash
	let mut hash: u64 = 0xcbf29ce484222325; // offset basis
	for b in s.as_bytes() {
		hash ^= *b as u64;
		hash = hash.wrapping_mul(0x100000001b3); // = 2^40 + 2^8 + 0xb3 , which is fast bc mul can be optimized into shifts and adds
	}
	hash
}

pub fn hash_str_to_u64_simple(s: &str) -> u64 {
	let mut hash = 0u64;
	for b in s.as_bytes() {
		hash = hash.wrapping_mul(31).wrapping_add(*b as u64);
	}
	hash
}

