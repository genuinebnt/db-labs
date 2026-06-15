// using u64 ensure the size is consistent acroos arch and cpus
pub type HashT = u64;
const PRIME_FACTOR: u64 = 10000019;

/// Safety: Implementing this trait means:
/// 1. The type is Copy
/// 2. The type has no uninitialized memory padding
/// 3. Any bit pattern of `size_of::<Self>()` is a
///    valid representation of this type
pub unsafe trait Pod: Copy {}

/// Primitive types that are safe to hash
unsafe impl Pod for u8 {}
unsafe impl Pod for u16 {}
unsafe impl Pod for u32 {}
unsafe impl Pod for u64 {}
unsafe impl Pod for i8 {}
unsafe impl Pod for i16 {}
unsafe impl Pod for i32 {}
unsafe impl Pod for i64 {}

/// Safe wrapper that converts any valid Pod types
///     to byte array
pub fn bytes_of<T: Pod>(ptr: &T) -> &[u8] {
    let size = std::mem::size_of::<T>();
    let byte_ptr = ptr as *const T as *const u8;

    // Safety: Guarenteed by `Pod` trait bounds on `T`
    // The reference `Ptr` is always a valid and initialized memory representation
    unsafe { std::slice::from_raw_parts(byte_ptr, size) }
}

/// The safe generic hashing function
pub fn hash<T: Pod>(ptr: &T) -> HashT {
    let bytes = bytes_of(ptr);
    hash_bytes(bytes)
}

/// A variant of knuth's multiplicative hashing
/// Reference: https://github.com/vmware-archive/gpos/blob/b53c1acd6285de94044ff91fbee91589543feba1/libgpos/src/utils.cpp#L126
pub fn hash_bytes(bytes: &[u8]) -> HashT {
    // Simple `Salt` using lenght as value
    let mut hash = bytes.len() as HashT;
    for &b in bytes {
        // Standard implementation of bitwise circular shift on 32 bits (5 + 27)
        hash = ((hash << 5) ^ (hash >> 27)) ^ (b as i8 as HashT);
    }
    hash
}

pub fn combine_hashes(l: HashT, r: HashT) -> HashT {
    let mut buf = [0u8; 16];
    buf[..8].copy_from_slice(&l.to_le_bytes());
    buf[8..].copy_from_slice(&r.to_le_bytes());
    hash_bytes(&buf)
}

pub fn sum_hashes(l: HashT, r: HashT) -> HashT {
    (l % PRIME_FACTOR + r % PRIME_FACTOR) % PRIME_FACTOR
}
