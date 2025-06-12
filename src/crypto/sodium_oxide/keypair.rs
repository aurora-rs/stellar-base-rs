use std::convert::TryInto;

use crate::crypto::sodium_oxide::random_bytes;
use crate::crypto::{strkey, KeyPair};
use crate::crypto::{Ed25519Signer, Signature};
use crate::error::{Error, Result};
use crate::network::Network;
use crate::PublicKey;
use sodiumoxide::crypto::sign::ed25519 as sodium;

/// The secret key of the account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretKey {
    key: sodium::SecretKey,
    seed: sodium::Seed,
}

impl Ed25519Signer<Signature> for SecretKey {
    fn try_sign(&self, msg: &[u8]) -> std::result::Result<ed25519::Signature, ed25519::Error> {
        self.key.try_sign(msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SodiumKeyPair(KeyPair<SecretKey, sodium::PublicKey>);

impl SecretKey {
    /// Return the inner key.
    pub fn inner(&self) -> &sodium::SecretKey {
        &self.key
    }

    /// Return the secret key as String, starting with `S`.
    pub fn secret_seed(&self) -> String {
        strkey::encode_secret_seed(&self.seed.0)
    }
}

impl AsRef<KeyPair<SecretKey, sodium::PublicKey>> for SodiumKeyPair {
    fn as_ref(&self) -> &KeyPair<SecretKey, sodium::PublicKey> {
        &self.0
    }
}

impl SodiumKeyPair {
    /// Create the key pair from the secret seed, e.g. `SDAKFNYEIAORZKKCYRILFQKLLOCNPL5SWJ3YY5NM3ZH6GJSZGXHZEPQS`.
    pub fn from_secret_seed(data: &str) -> Result<SodiumKeyPair> {
        let bytes = strkey::decode_secret_seed(data)?;
        Self::from_seed_bytes(&bytes)
    }

    /// Create a random key pair.
    pub fn random() -> Result<SodiumKeyPair> {
        let seed = random_bytes(32);
        Self::from_seed_bytes(&seed)
    }

    /// Create a key pair from the `network` passphrase.
    pub fn from_network(network: &Network) -> Result<SodiumKeyPair> {
        let bytes = network.network_id();
        Self::from_seed_bytes(&bytes)
    }

    /// Create a key pair from raw bytes.
    pub fn from_seed_bytes(data: &[u8]) -> Result<SodiumKeyPair> {
        let the_seed = sodium::Seed::from_slice(data).ok_or(Error::InvalidSeed)?;
        let (public, sk) = sodium::keypair_from_seed(&the_seed);
        let secret = SecretKey {
            key: sk,
            seed: the_seed,
        };
        Ok(SodiumKeyPair(KeyPair::new(secret, public)))
    }

    /// Return the secret key.
    pub fn secret_key(&self) -> &SecretKey {
        &self.0.signer.signing_key
    }

    /// Return the public key
    pub fn public_key(&self) -> PublicKey {
        PublicKey(
            self.0
                .verifier
                .verify_key
                .as_ref()
                .try_into()
                .expect("Ed22519 public key is not 32 bytes"),
        )
    }

    pub fn sign(&self, msg: &[u8]) -> Signature {
        self.0.sign(msg)
    }

    pub fn verify(&self, sig: &Signature, msg: &[u8]) -> bool {
        self.0.verify(msg, sig).map_or(false, |_| true)
    }
}

impl std::str::FromStr for SodiumKeyPair {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let sk = SodiumKeyPair::from_secret_seed(s)?;
        Ok(sk)
    }
}

#[cfg(test)]
mod tests {
    use super::{PublicKey, SodiumKeyPair};
    use crate::network::Network;
    use std::str::FromStr;

    #[test]
    fn test_from_secret_seed() {
        let keypairs = [
            (
                "SCRG6SFG64YDEVGWDWTZBE6BWEW25WICOGOUTODVICCA3L3FOQIG26E6",
                "GDRZ6UR4VI3DWATUKAXCGCGAUXGF3IJYG5T3CTJDUJ674TFLN4AR6RV4",
            ),
            (
                "SCOHYGOOQBONAMHLFFTS3OHG2V45GHRVU6Q5HI7VZ2T4C3WRSVCUAWRN",
                "GB5YW3UGJQ635LBGCXJVU3D2TXJIQRX7LRRWFMD2PKZLQ3G2OIZY4ZWO",
            ),
            (
                "SD4IDUZJZKPWU2T2AJWB35AN42L3NADBRCXHL5HY4WXPWO43MQTMCIIF",
                "GCEAKB6W342KSAQ6SVJYROF5W5FJTPZDDOSIOT3Y6CNQ3U2ZBAH7AQN3",
            ),
            (
                "SBB3NC6JM6IT6RXXQKMWH5XBGWWVWXL2MDBVOUPWWKP52QRFB2ISF2IR",
                "GCG5C6WXDSN55I25M7C6734EYVJ562KR4CKWEKK7RISUISNO36LX5J7G",
            ),
            (
                "SDGLIKRUQX5WJCLKGJUSAG3XK7GW262H7SQMLCKV4K5RWXNOWFD5VFG6",
                "GAGKPXZNQ7QDYKRMKE34IB7S5SUKPK4AWETZRBG6CYK3TNANV4HQHJGV",
            ),
            (
                "SD3AXQQRD4HXWHWVX7R56UHPT7ZDJPADTYTC3UTGRYHZ2AR37O66OZQB",
                "GD6KUX4TY4BFKCQBNAFK577IWNB3PNPASBG2JEBIJK6X4CSXNRWCINHU",
            ),
            (
                "SBHRV6BPRPIMTY7EYWFOLUNILQDU3UP5KWQA43UPW7R6WBTWTUGCY446",
                "GARHX6B4IAM2E3WUISLBWYY6TVWWQIWZPUYWXHSQ5BSSV5NKQX5CT4MC",
            ),
            (
                "SA7M4FOPTIT3CNW3B44CQMQRA4A7ZJCAOVHRUEHAMGFVWWCLIA3UXULE",
                "GASQUYLK6GRHBE6Z4A4HH5NHQPAHMHWAUSQP5N7D465QYYBEHDOPFXVP",
            ),
            (
                "SBKNQCZGK2X2VDOIAZWWZ7H3J5XWSU23UGUU62IXVTMWC6AGVN4N3OX7",
                "GD7EMEPGLBYDHUEDSJMAI64BS6HX46SAIPGIF3E5HI4CJ53VQ4OWEEBE",
            ),
            (
                "SA7UGZLM6Q6RR4BC7252IMUAHCZJLDHGBJFKVENIE2HKQ66T7L4T4ZMF",
                "GC22QEDRADRMZ2RPSBGY5N3RMGGSBKCMBQBFCJH3M4RZQ4KQQXUVQNID",
            ),
        ];

        for &(secret, address) in keypairs.iter() {
            let keypair = SodiumKeyPair::from_secret_seed(secret).unwrap();
            let keypair2 = SodiumKeyPair::from_str(secret).unwrap();
            assert_eq!(&keypair2.secret_key().secret_seed(), secret);
            let account_id = keypair.public_key().account_id();
            assert_eq!(&account_id, address);
            let parsed = PublicKey::from_str(&account_id).unwrap();
            assert_eq!(account_id, parsed.to_string());
        }
    }

    #[test]
    fn test_from_network() {
        let network = Network::new_public();
        let kp = SodiumKeyPair::from_network(&network).unwrap();
        let public = kp.public_key().account_id();
        assert_eq!(
            public,
            "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7"
        );
    }

    #[test]
    fn test_sign_and_verify() {
        let the_secret = "SD7X7LEHBNMUIKQGKPARG5TDJNBHKC346OUARHGZL5ITC6IJPXHILY36";
        let kp = SodiumKeyPair::from_secret_seed(the_secret).unwrap();
        let message = "test post please ignore".as_bytes();
        let sign = kp.sign(message);
        let expected_sign = [
            0x19, 0xDB, 0xFD, 0xAF, 0x0A, 0xA8, 0x4D, 0xF9, 0xA7, 0xFF, 0x6F, 0xE3, 0xC1, 0x0E,
            0xBC, 0x1F, 0xE2, 0x14, 0xC5, 0x10, 0xB9, 0x5D, 0xB0, 0xD6, 0x33, 0xBE, 0xD9, 0x3D,
            0xF9, 0x25, 0x6B, 0xA9, 0x92, 0xEF, 0x7D, 0x94, 0xB2, 0xA6, 0xE4, 0x54, 0xDE, 0x8F,
            0x21, 0x9, 0x28, 0xCA, 0x96, 0x11, 0x39, 0x03, 0x29, 0xC8, 0x40, 0xC8, 0xE5, 0x64,
            0xE7, 0xA0, 0x72, 0x16, 0x02, 0x7A, 0xB4, 0xA,
        ];
        assert_eq!(sign.to_bytes(), expected_sign[..]);
        assert!(kp.verify(&sign, message));
    }

    #[test]
    fn test_sign_decorated() {
        let the_secret = "SD7X7LEHBNMUIKQGKPARG5TDJNBHKC346OUARHGZL5ITC6IJPXHILY36";
        let kp = SodiumKeyPair::from_secret_seed(the_secret).unwrap();
        let message = "test post please ignore".as_bytes();
        let sign = kp.as_ref().sign_decorated(message);
        assert_eq!(sign.hint().to_vec(), vec![0x0B, 0xFA, 0xD1, 0x34]);
    }
}
