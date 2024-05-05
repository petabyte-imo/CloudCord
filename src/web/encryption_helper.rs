use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use rand::{thread_rng, Rng};

fn generate_random_bytes() -> [u8; 32] {
    let mut rng = thread_rng();
    let mut bytes = [0u8; 32];
    rng.fill(&mut bytes);
    bytes
}
pub fn encrypt(key: &[u8], nonce: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new_from_slice(key).unwrap();
    let nonce = Nonce::from_slice(nonce); // 96-bits; unique per message
    cipher.encrypt(nonce, plaintext).unwrap()
}
pub fn decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Vec<u8> {
    let cipher = ChaCha20Poly1305::new_from_slice(key).unwrap();
    let nonce = Nonce::from_slice(nonce); // 96-bits; unique per message
    cipher.decrypt(nonce, ciphertext).unwrap()
}
pub fn read_file(filecontents: &Vec<u8>) -> &[u8] {
    let to_return = &filecontents.as_slice();
    to_return
}
pub fn string_to_bytes(input: &str) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    let input_bytes = input.as_bytes();

    for (i, &byte) in input_bytes.iter().enumerate() {
        bytes[i] = byte;
    }

    bytes
}
pub fn encrypt_file(key: &[u8], nonce: &[u8], filename: &str) -> Vec<u8> {
    let filecontents = std::fs::read(filename).unwrap();
    let plaintext = read_file(&filecontents);
    let cipher = ChaCha20Poly1305::new_from_slice(key).unwrap();
    let nonce = Nonce::from_slice(nonce); // 96-bits; unique per message
    cipher.encrypt(nonce, plaintext).unwrap()
}
pub fn decrypt_file(key: &[u8], nonce: &[u8], filename: &str) -> Vec<u8> {
    println!("Decrypting file: {}", filename);
    let filecontents = std::fs::read(filename).unwrap();

    let plaintext = read_file(&filecontents);
    let cipher = ChaCha20Poly1305::new_from_slice(key).unwrap();
    let nonce = Nonce::from_slice(nonce); // 96-bits; unique per message
    cipher.decrypt(nonce, plaintext).unwrap()
}
