extern crate bitcoinrs_crypto;
extern crate openssl;
extern crate rand;

use rand::{OsRng, Rng};

use openssl::sha::sha256 as os_sha256;
use bitcoinrs_crypto::sha256 as btc_sha256;

#[test]
fn sha256_test() {
    assert_hash("hoge".as_bytes());

    let mut rng = OsRng::new().unwrap();
    for _ in 0..1000 {
        let buf = gen_random_vec(&mut rng);
        assert_hash(buf.as_slice());
    }
}

fn gen_random_vec(rng: &mut OsRng) -> Vec<u8> {
    let len = rng.next_u32() % 10000;
    let mut vec = Vec::with_capacity(len as usize);
    vec.resize(10000, 0);
    rng.fill_bytes(vec.as_mut_slice());
    vec
}

fn assert_hash(msg: &[u8]) {
    let btc_hashed = btc_sha256(msg);
    let openssl_hashed = os_sha256(msg);
    assert_eq!(btc_hashed, openssl_hashed);
}
