use sha2::{Sha256, Digest};
use std::io;

fn main() {
  println!("Enter the SECRET you'd like to encode");
  // Grab the Secret from the user
  let mut secret_input = String::new();
  // Read line the input they created and store it in the secret_type var
  io::stdin().read_line(&mut secret_input).ok().expect("Couldn't read line");
  // Create a new hasher
  let mut hasher = Sha256::new();
  // Write the input to the hasher
  hasher.input(secret_input);
  // Read the hash digest and consume the hasher
  let result = hasher.result();
  // Send the result back to the user
  println!("{:?}", hex::encode(result));
}
