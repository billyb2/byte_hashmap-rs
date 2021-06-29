//!Essentially, this crate is just a hashmap, except the hashing function is literally just writing the value being hashed. The advantage to this is that the HashMap itself is very, very, very fast. I haven't done any benchmarks, but I'd guess that its probably hundreds of times faster than the standard library version, as well as literally any other implementation that isn't doing something extremely similar to this. At the very least, it definetly is **faster**.
//!
//!The massive disadvantage, however, is that this sucks for literally any other input that isn't a singular byte. It definetly won't protect against collisions, and I'm unsure if it's vulnerable to any DoS attacks. Because of this, I added a little debug assertion that forces the HashMap to be used on single byte inputs, and nothing else.
//!
//!I made this in like a half hour for another project I'm working on (for speed improvements), and thought that I might as well share this. Please don't use this crate if you're worried about security or having hashmaps with key values that are over a byte in size, since the only thing this crate is useful for is being fast.
//!
//!The features of this crate just enable some features that are in [HashBrown](https://github.com/rust-lang/hashbrown), since it's a faster implementation of the standard library's HashMap. The `faster_hashmap` feature just uses HashBrown instead of the std HashMap, and is enabled by default.
//!
//!## Usage
//!Using this crate is almost exactly the same to the std HashMap (the only difference being you need to specify the Hasher)
//!
//!```rust
//!use single_byte_hashmap::*;
//!
//!let player_score: HashMap<u8, u16> = HashMap::with_capacity_and_hasher(256, BuildHasher::default());
//!player_score.insert(0, 2000);
//!player_score.insert(1, 3000);
//!
//!for (player_id, score) in player_score.iter() {
//!   println!("Player {} has a score of {}", player_id, score);
//!
//!}
//!
//!```

use std::default::Default;
use std::hash::{Hasher as OtherHasher, BuildHasherDefault};

#[cfg(feature = "faster_hashmap")]
use hashbrown::HashMap as OtherHashMap;
#[cfg(not(feature = "faster_hashmap"))]
use std::collections::HashMap as OtherHashMap;

#[allow(missing_copy_implementations)]
pub struct Hasher(u64);

impl Default for Hasher {
    #[inline]
    fn default() -> Hasher {
        Hasher(0)
    }
}

impl OtherHasher for Hasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }

    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        // This hasher is ONLY safe for comparing singular bytes, and nothing else
        debug_assert!(bytes.len() == 1);

        *self = Hasher(*bytes.first().unwrap() as u64);
    }
}

pub type BuildHasher = BuildHasherDefault<Hasher>;

pub type HashMap<K, V> = OtherHashMap<K, V, BuildHasher>;

