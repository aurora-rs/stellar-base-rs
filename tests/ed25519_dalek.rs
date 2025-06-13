//! A custom signer using ed25519-dalek (without relying on the "dalek" feature)

use rand::rngs::OsRng;
use stellar_base::crypto::KeyPair;

#[test]
pub fn dalek_sign_verify() {
    let mut rng = OsRng {};
    let signer = ed25519_dalek::SigningKey::generate(&mut rng);
    let verifier = signer.verifying_key();

    let keys = KeyPair::new(signer, verifier);

    let msg = b"test message";
    let sig = keys.sign(msg);
    assert!(keys.verify(msg, &sig).is_ok());
}
