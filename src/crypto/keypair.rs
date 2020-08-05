use crate::crypto;
use crate::crypto::strkey;
use crate::error::{Error, Result};
use crate::network::Network;
use crate::signature::{DecoratedSignature, Signature, SignatureHint};
use crate::xdr;
use sodiumoxide::crypto::sign::ed25519;

/// The public key of the account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicKey {
    key: ed25519::PublicKey,
}

/// The secret key of the account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecretKey {
    key: ed25519::SecretKey,
    seed: ed25519::Seed,
}

/// The secret and public key pair of the account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyPair {
    public: PublicKey,
    secret: SecretKey,
}

/// A public key together with an id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MuxedEd25519PublicKey {
    key: PublicKey,
    id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MuxedAccount {
    Ed25519(PublicKey),
    MuxedEd25519(MuxedEd25519PublicKey),
}

impl PublicKey {
    /// Create from `account_id`, e.g. `GB3KJPLFUYN5VL6R3GU3EGCGVCKFDSD7BEDX42HWG5BWFKB3KQGJJRMA`.
    pub fn from_account_id(account_id: &str) -> Result<PublicKey> {
        let bytes = strkey::decode_account_id(&account_id)?;
        Self::from_slice(&bytes)
    }

    /// Create from raw bytes.
    pub fn from_slice(data: &[u8]) -> Result<PublicKey> {
        let key = ed25519::PublicKey::from_slice(&data).ok_or(Error::InvalidPublicKey)?;
        Ok(PublicKey { key })
    }

    /// Return the inner key.
    pub fn inner(&self) -> &ed25519::PublicKey {
        &self.key
    }

    pub fn account_id(&self) -> String {
        strkey::encode_account_id(&self.key.0)
    }

    pub fn into_muxed_account(self, id: u64) -> MuxedAccount {
        let inner = MuxedEd25519PublicKey { key: self, id };
        MuxedAccount::MuxedEd25519(inner)
    }

    pub fn to_muxed_account(&self, id: u64) -> MuxedAccount {
        self.clone().into_muxed_account(id)
    }

    pub fn to_xdr_uint256(&self) -> Result<xdr::Uint256> {
        let bytes = self.as_bytes().to_vec();
        Ok(xdr::Uint256::new(bytes))
    }

    pub fn to_xdr_public_key(&self) -> Result<xdr::PublicKey> {
        let uint256 = self.to_xdr_uint256()?;
        Ok(xdr::PublicKey::PublicKeyTypeEd25519(uint256))
    }

    pub fn to_xdr_account_id(&self) -> Result<xdr::AccountId> {
        let public_key = self.to_xdr_public_key()?;
        Ok(xdr::AccountId::new(public_key))
    }

    pub fn from_xdr_public_key(x: &xdr::PublicKey) -> Result<PublicKey> {
        match x {
            xdr::PublicKey::PublicKeyTypeEd25519(inner) => Self::from_slice(&inner.value),
        }
    }

    pub fn from_xdr_account_id(x: &xdr::AccountId) -> Result<PublicKey> {
        Self::from_xdr_public_key(&x.value)
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.key.0
    }

    pub fn to_xdr(&self) -> Result<xdr::MuxedAccount> {
        let uint256 = self.to_xdr_uint256()?;
        Ok(xdr::MuxedAccount::KeyTypeEd25519(uint256))
    }
}

impl SecretKey {
    /// Return the inner key.
    pub fn inner(&self) -> &ed25519::SecretKey {
        &self.key
    }

    /// Return the secret key as String, starting with `S`.
    pub fn secret_seed(&self) -> String {
        strkey::encode_secret_seed(&self.seed.0)
    }
}

impl MuxedEd25519PublicKey {
    pub fn new(key: PublicKey, id: u64) -> MuxedEd25519PublicKey {
        MuxedEd25519PublicKey { key, id }
    }

    pub fn from_account_id(account_id: &str) -> Result<MuxedEd25519PublicKey> {
        let (bytes, id) = strkey::decode_muxed_account(&account_id)?;
        Self::from_slice(&bytes, id)
    }

    /// Create from raw byte and id.
    pub fn from_slice(data: &[u8], id: u64) -> Result<MuxedEd25519PublicKey> {
        let key = PublicKey::from_slice(&data)?;
        Ok(MuxedEd25519PublicKey { key, id })
    }

    /// Return the inner key.
    pub fn public_key(&self) -> &PublicKey {
        &self.key
    }

    pub fn account_id(&self) -> String {
        strkey::encode_muxed_account(&self.key.as_bytes(), self.id)
    }

    pub fn to_xdr(&self) -> Result<xdr::MuxedAccount> {
        let uint64 = xdr::Uint64::new(self.id);
        let uint256 = self.key.to_xdr_uint256()?;
        let muxed = xdr::MuxedAccountMed25519 {
            id: uint64,
            ed25519: uint256,
        };
        Ok(xdr::MuxedAccount::KeyTypeMuxedEd25519(muxed))
    }
}

impl MuxedAccount {
    pub fn account_id(&self) -> String {
        match self {
            MuxedAccount::Ed25519(pk) => pk.account_id(),
            MuxedAccount::MuxedEd25519(mx) => mx.account_id(),
        }
    }

    pub fn to_xdr(&self) -> Result<xdr::MuxedAccount> {
        match self {
            MuxedAccount::Ed25519(pk) => pk.to_xdr(),
            MuxedAccount::MuxedEd25519(mx) => mx.to_xdr(),
        }
    }

    pub fn from_xdr(x: &xdr::MuxedAccount) -> Result<MuxedAccount> {
        match x {
            xdr::MuxedAccount::KeyTypeEd25519(buf) => {
                let inner = PublicKey::from_slice(&buf.value)?;
                Ok(MuxedAccount::Ed25519(inner))
            }
            xdr::MuxedAccount::KeyTypeMuxedEd25519(mx) => {
                let inner = MuxedEd25519PublicKey::from_slice(&mx.ed25519.value, mx.id.value)?;
                Ok(MuxedAccount::MuxedEd25519(inner))
            }
        }
    }
}

impl KeyPair {
    /// Create the key pair from the secret seed, e.g. `SDAKFNYEIAORZKKCYRILFQKLLOCNPL5SWJ3YY5NM3ZH6GJSZGXHZEPQS`.
    pub fn from_secret_seed(data: &str) -> Result<KeyPair> {
        let bytes = strkey::decode_secret_seed(&data)?;
        Self::from_seed_bytes(&bytes)
    }

    /// Create a random key pair.
    pub fn random() -> Result<KeyPair> {
        let seed = crypto::random_bytes(32);
        Self::from_seed_bytes(&seed)
    }

    /// Create a key pair from the `network` passphrase.
    pub fn from_network(network: &Network) -> Result<KeyPair> {
        let bytes = network.network_id();
        Self::from_seed_bytes(&bytes)
    }

    /// Crete a key pair from raw bytes.
    pub fn from_seed_bytes(data: &[u8]) -> Result<KeyPair> {
        let the_seed = ed25519::Seed::from_slice(&data).ok_or(Error::InvalidSeed)?;
        let (pk, sk) = ed25519::keypair_from_seed(&the_seed);
        let public = PublicKey { key: pk };
        let secret = SecretKey {
            key: sk,
            seed: the_seed,
        };
        Ok(KeyPair { public, secret })
    }

    /// Return the public key.
    pub fn public_key(&self) -> &PublicKey {
        &self.public
    }

    /// Return the secret key.
    pub fn secret_key(&self) -> &SecretKey {
        &self.secret
    }

    /// Sign the `message`.
    pub fn sign(&self, message: &[u8]) -> Signature {
        Signature::sign(&self.secret, &message)
    }

    /// Sign the `message` together with the signature hint.
    pub fn sign_decorated(&self, message: &[u8]) -> DecoratedSignature {
        let hint = self.signature_hint();
        let signature = self.sign(message);
        DecoratedSignature::new(hint, signature)
    }

    /// Verify the `signature` against the `message`.
    pub fn verify(&self, message: &[u8], signature: &Signature) -> bool {
        signature.verify(&self.public, &message)
    }

    /// Return the signature hint, that is the last 4 bytes of the public key.
    pub fn signature_hint(&self) -> SignatureHint {
        SignatureHint::from_public_key(&self.public)
    }
}

impl From<PublicKey> for MuxedAccount {
    fn from(pk: PublicKey) -> Self {
        MuxedAccount::Ed25519(pk)
    }
}

impl From<MuxedEd25519PublicKey> for MuxedAccount {
    fn from(muxed: MuxedEd25519PublicKey) -> Self {
        MuxedAccount::MuxedEd25519(muxed)
    }
}

impl std::str::FromStr for PublicKey {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let pk = PublicKey::from_account_id(s)?;
        Ok(pk)
    }
}

impl std::fmt::Display for PublicKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.account_id())
    }
}

impl std::str::FromStr for KeyPair {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let sk = KeyPair::from_secret_seed(&s)?;
        Ok(sk)
    }
}

#[cfg(test)]
mod tests {
    use super::{KeyPair, PublicKey};
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
            let keypair = KeyPair::from_secret_seed(&secret).unwrap();
            let keypair2 = KeyPair::from_str(&secret).unwrap();
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
        let kp = KeyPair::from_network(&network).unwrap();
        let public = kp.public_key().account_id();
        assert_eq!(
            public,
            "GAAZI4TCR3TY5OJHCTJC2A4QSY6CJWJH5IAJTGKIN2ER7LBNVKOCCWN7"
        );
    }

    #[test]
    fn test_sign_and_verify() {
        let the_secret = "SD7X7LEHBNMUIKQGKPARG5TDJNBHKC346OUARHGZL5ITC6IJPXHILY36";
        let kp = KeyPair::from_secret_seed(&the_secret).unwrap();
        let message = "test post please ignore".as_bytes();
        let sign = kp.sign(&message);
        let expected_sign = vec![
            0x19, 0xDB, 0xFD, 0xAF, 0x0A, 0xA8, 0x4D, 0xF9, 0xA7, 0xFF, 0x6F, 0xE3, 0xC1, 0x0E,
            0xBC, 0x1F, 0xE2, 0x14, 0xC5, 0x10, 0xB9, 0x5D, 0xB0, 0xD6, 0x33, 0xBE, 0xD9, 0x3D,
            0xF9, 0x25, 0x6B, 0xA9, 0x92, 0xEF, 0x7D, 0x94, 0xB2, 0xA6, 0xE4, 0x54, 0xDE, 0x8F,
            0x21, 0x9, 0x28, 0xCA, 0x96, 0x11, 0x39, 0x03, 0x29, 0xC8, 0x40, 0xC8, 0xE5, 0x64,
            0xE7, 0xA0, 0x72, 0x16, 0x02, 0x7A, 0xB4, 0xA,
        ];
        assert_eq!(sign.to_vec(), expected_sign);
        assert!(kp.verify(&message, &sign));
    }

    #[test]
    fn test_sign_decorated() {
        let the_secret = "SD7X7LEHBNMUIKQGKPARG5TDJNBHKC346OUARHGZL5ITC6IJPXHILY36";
        let kp = KeyPair::from_secret_seed(&the_secret).unwrap();
        let message = "test post please ignore".as_bytes();
        let sign = kp.sign_decorated(&message);
        assert_eq!(sign.hint().to_vec(), vec![0x0B, 0xFA, 0xD1, 0x34]);
    }
}
