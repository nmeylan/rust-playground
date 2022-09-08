#![feature(default_free_fn)]

use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::default::default;
use std::hash::{BuildHasher, Hash, Hasher};

#[derive(Default)]
struct u64Hasher {
    hash: u64
}

impl BuildHasher for u64Hasher {
    type Hasher = u64Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        Default::default()
    }
}

impl Hasher for u64Hasher {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hash =  u64::from_ne_bytes(bytes.try_into().unwrap());
    }
}


fn main() {
    let mut map = HashMap::with_hasher(u64Hasher::default());
    let v = "value";
    let hash = calculate_hash(&v);
    map.insert(hash, v);
}



fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}