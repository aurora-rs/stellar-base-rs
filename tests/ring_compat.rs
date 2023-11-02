//! A custom signer using ring-compat (instead of sodiumoxide)

use rand::rngs::OsRng;
use rand::RngCore;
use ring_compat::signature::ed25519::SigningKey;
use stellar_base::crypto::KeyPair;

#[test]
pub fn ring_compat_sign_verify() {
    let mut rng = OsRng {};
    let mut ed25519_seed = [0u8; 32];
    rng.fill_bytes(&mut ed25519_seed);

    let signing_key = SigningKey::from_seed(&ed25519_seed).unwrap();
    let verify_key = signing_key.verifying_key();

    let keys = KeyPair::new(signing_key, verify_key);

    let msg = b"test message";
    let sig = keys.sign(msg);
    assert!(keys.verify(msg, &sig).is_ok());
}
