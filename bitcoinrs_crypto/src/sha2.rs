use bitcoinrs_bytes::{WriteBuf, endian::u64_b};

type Word = u32;
type HashValue = [Word; 8];
type MsgBlock = [Word; 16];
type ExpandedMsgBlock = [Word; 64]; // aka message schedule.

pub fn sha256(msg: &[u8]) -> [u8; 32] {
    // Preprocessing
    let vec = get_padded_vec(msg);
    let msg_block_iter = MsgBlockIter::new(vec.as_slice());

    // Computation
    let hash_val = compute_hash(msg_block_iter);

    // Finalize
    parse_into_result(hash_val)
}

/* ===================================== */
/* Preprocessing */
/* ===================================== */

const BYTE_SIZE_PADD_BASE: usize = 512 / 8;
const BYTE_SIZE_DATA_LEN: usize = 64 / 8;

fn get_padded_vec(msg: &[u8]) -> Vec<u8> {
    // Calc after padded size
    let size_zero_padding = size_zero_padding(msg.len());
    let padded_size = msg.len() + 1 + size_zero_padding + 8;

    // Prepare buffer vec
    let mut vec = Vec::with_capacity(padded_size);
    vec.write_bytes(msg);
    vec.write(0b_1000_0000_u8);
    vec.write_zeros(size_zero_padding);
    vec.write(u64_b::new(msg.len() as u64 * 8)); // Length in bits.

    vec
}

fn size_zero_padding(l: usize) -> usize {
    let resv_size = (l + 1 + BYTE_SIZE_DATA_LEN) % BYTE_SIZE_PADD_BASE;
    BYTE_SIZE_PADD_BASE - resv_size
}

struct MsgBlockIter<'a> {
    msg: &'a [u8],
}

impl<'a> MsgBlockIter<'a> {
    pub fn new(msg: &'a [u8]) -> MsgBlockIter<'a> {
        MsgBlockIter { msg: msg }
    }

    fn next_u32(&mut self) -> Option<u32> {
        if self.msg.len() < 4 {
            None
        } else {
            let (n, rmn) = self.msg.split_at(4);
            self.msg = rmn;
            unsafe { Some(u32::from_be(*(n as *const _ as *const u32))) }
        }
    }
}

impl<'a> Iterator for MsgBlockIter<'a> {
    type Item = MsgBlock;

    fn next(&mut self) -> Option<MsgBlock> {
        let mut msg_block = [0; 16];

        for i in 0..16 {
            msg_block[i] = self.next_u32()?;
        }
        Some(msg_block)
    }
}

/* ===================================== */
/* Computation */
/* ===================================== */

fn compute_hash(msg_blocks: MsgBlockIter) -> HashValue {
    msg_blocks.fold(INIT_HASH_VAL, |hash, msg_block| {
        compute_next_hash_val(msg_block, hash)
    })
}

/// Sha-256 initial hash value.
const INIT_HASH_VAL: HashValue = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19
];

const SHA256_CONST_WORDS: [Word; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn compute_next_hash_val(msg_block: MsgBlock, prev_hash: HashValue) -> HashValue {
    use self::word_ops::*;

    // Initialize working variables.
    let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h) = (
        prev_hash[0],
        prev_hash[1],
        prev_hash[2],
        prev_hash[3],
        prev_hash[4],
        prev_hash[5],
        prev_hash[6],
        prev_hash[7],
    );

    // Prepare the expanded message block (aka message schedule).
    let expanded_msg_block = get_expand_msg_block(msg_block);

    // Compute working variables.
    for t in 0..64 {
        let t1 = h.wrapping_add(big_sigma_1(e))
            .wrapping_add(choose(e, f, g))
            .wrapping_add(SHA256_CONST_WORDS[t])
            .wrapping_add(expanded_msg_block[t]);
        let t2 = big_sigma_0(a).wrapping_add(majority(a, b, c));
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }

    // Compute next hash value.
    let mut new_hash = [0; 8];
    new_hash[0] = a.wrapping_add(prev_hash[0]);
    new_hash[1] = b.wrapping_add(prev_hash[1]);
    new_hash[2] = c.wrapping_add(prev_hash[2]);
    new_hash[3] = d.wrapping_add(prev_hash[3]);
    new_hash[4] = e.wrapping_add(prev_hash[4]);
    new_hash[5] = f.wrapping_add(prev_hash[5]);
    new_hash[6] = g.wrapping_add(prev_hash[6]);
    new_hash[7] = h.wrapping_add(prev_hash[7]);
    new_hash
}

fn get_expand_msg_block(msg_block: MsgBlock) -> ExpandedMsgBlock {
    let mut expanded = [0; 64];
    expanded[..16].copy_from_slice(&msg_block);

    use self::word_ops::*;
    for t in 16..64 {
        let w = small_sigma_1(expanded[t - 2])
            .wrapping_add(expanded[t - 7])
            .wrapping_add(small_sigma_0(expanded[t - 15]))
            .wrapping_add(expanded[t - 16]);
        expanded[t] = w;
    }

    expanded
}

mod word_ops {
    use super::Word;

    pub fn choose(x: Word, y: Word, z: Word) -> Word {
        (x & y) ^ (!x & z)
    }

    pub fn majority(x: Word, y: Word, z: Word) -> Word {
        (x & y) ^ (x & z) ^ (y & z)
    }

    /// Represented as large sigma 0 to 256.
    pub fn big_sigma_0(x: Word) -> Word {
        x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
    }

    pub fn big_sigma_1(x: Word) -> Word {
        x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
    }

    pub fn small_sigma_0(x: Word) -> Word {
        x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
    }

    pub fn small_sigma_1(x: Word) -> Word {
        x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
    }
}

/* ===================================== */
/* Computation */
/* ===================================== */

fn parse_into_result(hash_val: HashValue) -> [u8; 32] {
    let mut res = [0; 32];
    for i in 0..8 {
        let bytes = u32_to_bytes(hash_val[i]);
        (&mut res[i * 4..(i + 1) * 4]).copy_from_slice(&bytes);
    }
    res
}

fn u32_to_bytes(n: u32) -> [u8; 4] {
    use std::mem::transmute;

    unsafe { transmute::<u32, [u8; 4]>(n.to_be()) }
}
