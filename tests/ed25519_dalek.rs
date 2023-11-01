//! A custom signer using ed25519-dalek (instead of sodiumoxide)

use ed25519_dalek::Keypair as DalekKeypair;
use rand::rngs::OsRng;
use stellar_base::crypto::KeyPair;

#[test]
pub fn dalek_sign_verify() {
    let mut rng = OsRng {};
    let key_pair = DalekKeypair::generate(&mut rng);
    let signer = key_pair;
    let verifier = signer.public;

    let keys = KeyPair::new(signer, verifier);

    let msg = b"test message";
    let sig = keys.sign(msg);
    assert!(keys.verify(msg, &sig).is_ok());
}
