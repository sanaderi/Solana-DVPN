use sha2::{Digest, Sha256};

fn main() {
    let instruction_name = "global:create_plan"; // Replace with your instruction name
    let mut hasher = Sha256::new();
    hasher.update(instruction_name.as_bytes());
    let result = hasher.finalize();
    let discriminator = &result[..8]; // First 8 bytes
    println!(
        "Discriminator for '{}': {:?}",
        instruction_name, discriminator
    );
}
