use ring::{agreement, rand};

fn main() {
    let rng = rand::SystemRandom::new();
    let my_private_key = agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng);
    println!("Hello, world!");
}
