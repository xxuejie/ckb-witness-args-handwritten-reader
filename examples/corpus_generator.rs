use ckb_types::{bytes::Bytes, packed::WitnessArgs, prelude::*};
use rand::{rngs::StdRng, Rng, RngCore, SeedableRng};
use std::time::SystemTime;

const CORPUSES: usize = 200;
const CORPUS_DIR: &str = "corpus";

fn main() {
    let seed: u64 = match std::env::var("SEED") {
        Ok(val) => str::parse(&val).expect("parsing number"),
        Err(_) => SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64,
    };
    println!("Seed: {}", seed);
    let mut rng = StdRng::seed_from_u64(seed);

    std::fs::create_dir_all(CORPUS_DIR).expect("create dir");
    for i in 0..CORPUSES {
        let name = format!(
            "{}/corpus-{}-{}-{}",
            CORPUS_DIR,
            i,
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("time")
                .as_nanos(),
            rng.next_u64(),
        );

        let mut builder = WitnessArgs::new_builder();
        if rng.gen_bool(0.5) {
            let len = rng.gen_range(1..65535);
            let mut data = vec![0; len];
            rng.fill_bytes(&mut data);
            builder = builder.lock(Some(Bytes::from(data)).pack());
        }
        if rng.gen_bool(0.5) {
            let len = rng.gen_range(1..65535);
            let mut data = vec![0; len];
            rng.fill_bytes(&mut data);
            builder = builder.input_type(Some(Bytes::from(data)).pack());
        }
        if rng.gen_bool(0.5) {
            let len = rng.gen_range(1..65535);
            let mut data = vec![0; len];
            rng.fill_bytes(&mut data);
            builder = builder.output_type(Some(Bytes::from(data)).pack());
        }
        let witness = builder.build().as_bytes();

        std::fs::write(&name, &witness).expect("write");
    }
}
