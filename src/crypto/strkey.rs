use crate::error::{Error, Result};
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use crc16::{State, XMODEM};

const ACCOUNT_ID_VERSION_BYTE: u8 = 6 << 3; // G
const MUXED_ACCOUNT_VERSION_BYTE: u8 = 12 << 3; // M
const SECRET_SEED_VERSION_BYTE: u8 = 18 << 3; // S
const PRE_AUTH_TX_VERSION_BYTE: u8 = 19 << 3; // T
const SHA256_HASH_VERSION_BYTE: u8 = 23 << 3; // X

static ALPHABET: base32::Alphabet = base32::Alphabet::RFC4648 { padding: false };

pub fn encode_account_id(data: &[u8]) -> String {
    encode_check(ACCOUNT_ID_VERSION_BYTE, data)
}

pub fn decode_account_id(data: &str) -> Result<Vec<u8>> {
    decode_check(ACCOUNT_ID_VERSION_BYTE, data)
}

pub fn encode_muxed_account(data: &[u8], id: u64) -> String {
    let mut data_to_encode = Vec::new();
    data_to_encode.resize(8 + data.len(), b'0');
    BigEndian::write_u64(&mut data_to_encode[..8], id);
    data_to_encode[8..].copy_from_slice(&data);
    encode_check(MUXED_ACCOUNT_VERSION_BYTE, &data_to_encode)
}

pub fn decode_muxed_account(data: &str) -> Result<(Vec<u8>, u64)> {
    let bytes = decode_check(MUXED_ACCOUNT_VERSION_BYTE, data)?;
    let mut decoded_data = Vec::new();
    decoded_data.resize(bytes.len() - 8, b'0');
    let id = BigEndian::read_u64(&bytes[..8]);
    decoded_data.copy_from_slice(&bytes[8..]);
    Ok((decoded_data, id))
}

pub fn encode_secret_seed(data: &[u8]) -> String {
    encode_check(SECRET_SEED_VERSION_BYTE, data)
}
pub fn decode_secret_seed(data: &str) -> Result<Vec<u8>> {
    decode_check(SECRET_SEED_VERSION_BYTE, data)
}

pub fn encode_pre_auth_tx(data: &[u8]) -> String {
    encode_check(PRE_AUTH_TX_VERSION_BYTE, data)
}
pub fn decode_pre_auth_tx(data: &str) -> Result<Vec<u8>> {
    decode_check(PRE_AUTH_TX_VERSION_BYTE, data)
}

pub fn encode_sha256_hash(data: &[u8]) -> String {
    encode_check(SHA256_HASH_VERSION_BYTE, data)
}
pub fn decode_sha256_hash(data: &str) -> Result<Vec<u8>> {
    decode_check(SHA256_HASH_VERSION_BYTE, data)
}

fn encode_check(version: u8, indata: &[u8]) -> String {
    let mut data = Vec::with_capacity(35);
    data.push(version);
    data.extend_from_slice(&indata);
    let checksum = calculate_checksum(&data);
    let data_end = data.len();
    data.resize(data_end + 2, 0);
    LittleEndian::write_u16(&mut data[data_end..], checksum);
    base32::encode(ALPHABET, &data)
}

fn decode_unchecked(data: &str) -> Result<(u8, Vec<u8>)> {
    let decoded = base32::decode(ALPHABET, &data).ok_or(Error::InvalidStrKey)?;
    let decoded_len = decoded.len();

    if decoded_len == 0 {
        return Err(Error::InvalidStrKey);
    }

    let version_byte = decoded[0];

    if version_byte != MUXED_ACCOUNT_VERSION_BYTE && decoded_len != 35 {
        return Err(Error::InvalidStrKey);
    }

    if version_byte == MUXED_ACCOUNT_VERSION_BYTE && decoded_len != 43 {
        return Err(Error::InvalidStrKey);
    }

    let payload = &decoded[..decoded_len - 2];
    let data = &payload[1..];
    let checksum_bytes = &decoded[decoded_len - 2..];
    let checksum = calculate_checksum(payload);

    if !verify_checksum(checksum, checksum_bytes) {
        return Err(Error::InvalidStrKeyChecksum);
    }
    let key = data.to_vec();
    Ok((version_byte, key))
}

fn decode_check(expected_version: u8, data: &str) -> Result<Vec<u8>> {
    let (version_byte, key) = decode_unchecked(data)?;
    if version_byte != expected_version {
        return Err(Error::InvalidStrKeyVersionByte);
    }
    Ok(key)
}

fn calculate_checksum(payload: &[u8]) -> u16 {
    State::<XMODEM>::calculate(payload)
}

fn verify_checksum(checksum: u16, bytes: &[u8]) -> bool {
    let expected = LittleEndian::read_u16(bytes);
    expected == checksum
}

#[cfg(test)]
mod tests {
    use super::{decode_account_id, encode_account_id};
    use super::{decode_muxed_account, encode_muxed_account};
    use super::{decode_pre_auth_tx, encode_pre_auth_tx};
    use super::{decode_secret_seed, encode_secret_seed};
    use super::{decode_sha256_hash, encode_sha256_hash};
    use crate::crypto::SodiumKeyPair;
    use crate::network::Network;

    #[test]
    fn test_encode_decode_secret_seed() {
        let seed = "SDJHRQF4GCMIIKAAAQ6IHY42X73FQFLHUULAPSKKD4DFDM7UXWWCRHBE";
        let secret = decode_secret_seed(&seed).unwrap();
        let encoded = encode_secret_seed(&secret);
        assert_eq!(seed, &encoded);
    }

    #[test]
    fn test_encode_decode_account_id() {
        let addr = "GCZHXL5HXQX5ABDM26LHYRCQZ5OJFHLOPLZX47WEBP3V2PF5AVFK2A5D";
        let accountid = decode_account_id(&addr).unwrap();
        let encoded = encode_account_id(&accountid);
        assert_eq!(addr, &encoded);
    }

    #[test]
    fn test_invalid_version() {
        let addr = "GCZHXL5HXQX5ABDM26LHYRCQZ5OJFHLOPLZX47WEBP3V2PF5AVFK2A5D";
        let result = decode_secret_seed(&addr);
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_decode_muxed_account() {
        let addr = "MAAAAAAAAAAAAAB7BQ2L7E5NBWMXDUCMZSIPOBKRDSBYVLMXGSSKF6YNPIB7Y77ITLVL6";
        let (key, id) = decode_muxed_account(addr).unwrap();
        assert_eq!(0, id);
        let public_addr = encode_account_id(&key);
        assert_eq!(
            "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ",
            public_addr
        );
        let back = encode_muxed_account(&key, 0);
        assert_eq!(addr, back);
    }

    #[test]
    fn test_encode_decode_muxed_account_with_large_id() {
        let addr = "MCAAAAAAAAAAAAB7BQ2L7E5NBWMXDUCMZSIPOBKRDSBYVLMXGSSKF6YNPIB7Y77ITKNOG";
        let (key, id) = decode_muxed_account(addr).unwrap();
        assert_eq!(9223372036854775808, id);
        let public_addr = encode_account_id(&key);
        assert_eq!(
            "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ",
            public_addr
        );
        let back = encode_muxed_account(&key, id);
        assert_eq!(addr, back);
    }

    #[test]
    fn test_invalid_account_id() {
        let addresses = vec![
            "SAA6NXOBOXP3RXGAXBW6PGFI5BPK4ODVAWITS4VDOMN5C2M4B66ZML",
            "MAAAAAAAAAAAAAB7BQ2L7E5NBWMXDUCMZSIPOBKRDSBYVLMXGSSKF6YNPIB7Y77ITLVL6",
            "GAAAAAAAACGC6",
            "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJUACUSI",
            "G47QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVP2I",
            "",
        ];
        for addr in addresses {
            let result = decode_account_id(&addr);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_invalid_muxed_account() {
        let addresses = vec![
            "SAA6NXOBOXP3RXGAXBW6PGFI5BPK4ODVAWITS4VDOMN5C2M4B66ZML",
            "GA7QYNF7SOWQ3GLR2BGMZEHXAVIRZA4KVWLTJJFC7MGXUA74P7UJVSGZ",
            "MAAAAAAAAAAAAAB7BQ2L7E5NBWMXDUCMZSIPOBKRDSBYVLMXGSSKF6YNPIB7Y77ITIADJPA",
            "M4AAAAAAAAAAAAB7BQ2L7E5NBWMXDUCMZSIPOBKRDSBYVLMXGSSKF6YNPIB7Y77ITIU2K",
            "MAAAAAAAAAAAAAB7BQ2L7E5NBWMXDUCMZSIPOBKRDSBYVLMXGSSKF6YNPIB7Y77ITLVL4",
            "",
        ];

        for addr in addresses {
            let result = decode_muxed_account(&addr);
            assert!(result.is_err());
        }
    }

    #[test]
    fn test_pre_auth_tx() {
        let keypair = SodiumKeyPair::from_network(&Network::new_test()).unwrap();
        let pk = keypair.public_key();
        let encoded = encode_pre_auth_tx(pk.as_bytes());
        assert_eq!('T', encoded.chars().next().unwrap());
        let decoded = decode_pre_auth_tx(&encoded).unwrap();
        assert_eq!(pk.as_bytes(), &decoded[..]);
    }

    #[test]
    fn test_sha256_hash() {
        let keypair = SodiumKeyPair::from_network(&Network::new_test()).unwrap();
        let pk = keypair.public_key();
        let encoded = encode_sha256_hash(&pk.as_bytes());
        assert_eq!('X', encoded.chars().next().unwrap());
        let decoded = decode_sha256_hash(&encoded).unwrap();
        assert_eq!(pk.as_bytes(), &decoded[..]);
    }
}
