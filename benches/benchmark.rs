// Also look in Cargo.toml how to use a benchmark setup with harness = false


extern crate cobs;

use cobs::{max_encoding_length, encode, decode, encode_vec, decode_vec};

#[macro_use]
extern crate bencher;
extern crate rand;

use bencher::Bencher;
use rand::Rng; 

const LIST_LEN: usize = 1024;


fn provide_rand_vec(len: usize ) -> Vec<u8> {
  let mut rng = rand::thread_rng();
  let mut data = Vec::<u8>::with_capacity(len);

  for _ in 0..len {
      // 0 (inclusive) to 255 (exclusive)
      data.push(rng.gen_range(0, 255));
  }
  
  data
}

fn force_roundtrip(source: Vec<u8>) {
    let encoded = encode_vec(&source);
    let _decoded = decode_vec(&encoded).expect("decode_vec");
}

fn large_00(bench: &mut Bencher) {
  bench.iter(|| {
    force_roundtrip(vec![0u8; LIST_LEN]);
  });
  bench.bytes = LIST_LEN as u64;
}

fn large_06(bench: &mut Bencher) {
  bench.iter(|| {
    force_roundtrip(vec![6u8; 2048]);
  });
  bench.bytes = LIST_LEN as u64;
}



fn large_typical(bench: &mut Bencher) {
  let data = provide_rand_vec(LIST_LEN);
  bench.iter( move || {
    let locz = &data;
    force_roundtrip(locz.to_vec());
  });
  
  bench.bytes = LIST_LEN as u64;
}


benchmark_group!(benches, large_typical, large_06, large_00);
benchmark_main!(benches);