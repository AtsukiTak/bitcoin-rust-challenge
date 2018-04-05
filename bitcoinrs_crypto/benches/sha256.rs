#![feature(test)]
extern crate test;
use test::Bencher;

extern crate bitcoinrs_crypto;
use bitcoinrs_crypto::sha256;

#[bench]
fn sha256_with_empty_msg(b: &mut Bencher) {
    let msg = "";
    b.iter(|| sha256(msg.as_bytes()));
}

#[bench]
fn sha256_with_small_msg(b: &mut Bencher) {
    let msg = "hogehogehogeohgeogheohgoehgoehgoehgohge";
    b.iter(|| sha256(msg.as_bytes()));
}

#[bench]
fn sha256_with_large_msg(b: &mut Bencher) {
    let msg: [u8; 10000] = [0; 10000];
    b.iter(|| sha256(&msg[..]));
}
