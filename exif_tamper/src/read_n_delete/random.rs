use rand::seq::{IndexedRandom};

pub fn random() -> i32 {
    let mut rng = rand::rng();
    let nums: Vec<i32>  = (0..1000).collect();

    let num = nums.choose(&mut rng).unwrap();
    num.to_owned()
}