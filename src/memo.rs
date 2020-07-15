use crate::error::{Error, Result};
use crate::xdr;
use crate::xdr::{XDRDeserialize, XDRSerialize};
use xdr_rs_serialize::de::XDRIn;
use xdr_rs_serialize::ser::XDROut;

const MAX_MEMO_TEXT_LEN: usize = 28;
const MAX_HASH_LEN: usize = 32;

/// Memo attached to transactions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Memo {
    /// No memo
    None,
    /// Text Memo
    Text(String),
    /// Id Memo
    Id(u64),
    /// Hash Memo
    Hash([u8; 32]),
    /// Return Memo
    Return([u8; 32]),
}

impl Memo {
    /// Create new empty memo.
    pub fn none() -> Memo {
        Memo::None
    }

    /// Create new id memo.
    pub fn id(id: u64) -> Memo {
        Memo::Id(id)
    }

    /// Create new text memo. `text` must be shorter than 28 bytes.
    pub fn text<S: Into<String>>(text: S) -> Result<Memo> {
        let text = text.into();
        if text.len() > MAX_MEMO_TEXT_LEN {
            Err(Error::InvalidMemoText)
        } else {
            Ok(Memo::Text(text))
        }
    }

    /// Create new hash memo.
    pub fn hash(hash: &[u8]) -> Result<Memo> {
        if hash.len() > MAX_HASH_LEN {
            Err(Error::InvalidMemoHash)
        } else {
            let mut memo_hash: [u8; 32] = Default::default();
            memo_hash[..hash.len()].copy_from_slice(&hash);
            Ok(Memo::Hash(memo_hash))
        }
    }

    /// Creates new return memo.
    pub fn return_(ret: &[u8]) -> Result<Memo> {
        if ret.len() > MAX_HASH_LEN {
            Err(Error::InvalidMemoReturn)
        } else {
            let mut memo_ret: [u8; 32] = Default::default();
            memo_ret[..ret.len()].copy_from_slice(&ret);
            Ok(Memo::Return(memo_ret))
        }
    }

    /// Returns `true` if memo is `None`.
    pub fn is_none(&self) -> bool {
        match self {
            Memo::None => true,
            _ => false,
        }
    }

    /// Returns `true` if memo is `Id`.
    pub fn is_id(&self) -> bool {
        match self {
            Memo::Id(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if memo is `Text`.
    pub fn is_text(&self) -> bool {
        match self {
            Memo::Text(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if memo is `Hash`.
    pub fn is_hash(&self) -> bool {
        match self {
            Memo::Hash(_) => true,
            _ => false,
        }
    }

    /// Returns `true` if memo is `Return`.
    pub fn is_return(&self) -> bool {
        match self {
            Memo::Return(_) => true,
            _ => false,
        }
    }

    /// Retrieves memo value if the memo is of type `Id`, returning `None` if not an `Id` memo.
    pub fn id_value(&self) -> Option<&u64> {
        match self {
            Memo::Id(id) => Some(id),
            _ => None,
        }
    }

    /// Retrieves memo value if the memo is of type `Text`, returning `None` if not a `Text` memo.
    pub fn text_value(&self) -> Option<&str> {
        match self {
            Memo::Text(text) => Some(text),
            _ => None,
        }
    }

    /// Retrieves memo value if the memo is of type `Hash`, returning `None` if not a `Hash` memo.
    pub fn hash_value(&self) -> Option<&[u8; 32]> {
        match self {
            Memo::Hash(hash) => Some(hash),
            _ => None,
        }
    }

    /// Retrieves memo value if the memo is of type `Return`, returning `None` if not a `Return` memo.
    pub fn return_value(&self) -> Option<&[u8; 32]> {
        match self {
            Memo::Return(ret) => Some(ret),
            _ => None,
        }
    }

    pub fn to_xdr(&self) -> Result<xdr::Memo> {
        match self {
            Memo::None => Ok(xdr::Memo::MemoNone(())),
            Memo::Text(text) => Ok(xdr::Memo::MemoText(text.clone())),
            Memo::Id(id) => Ok(xdr::Memo::MemoId(xdr::Uint64::new(*id))),
            Memo::Hash(hash) => {
                let hash = xdr::Hash::new(hash.to_vec());
                Ok(xdr::Memo::MemoHash(hash))
            }
            Memo::Return(ret) => {
                let ret = xdr::Hash::new(ret.to_vec());
                Ok(xdr::Memo::MemoReturn(ret))
            }
        }
    }

    pub fn from_xdr(x: &xdr::Memo) -> Result<Memo> {
        match x {
            xdr::Memo::MemoNone(()) => Ok(Memo::none()),
            xdr::Memo::MemoText(text) => Memo::text(text),
            xdr::Memo::MemoId(id) => Ok(Memo::id(id.value)),
            xdr::Memo::MemoHash(hash) => Memo::hash(&hash.value),
            xdr::Memo::MemoReturn(ret) => Memo::return_(&ret.value),
        }
    }
}

impl Default for Memo {
    fn default() -> Memo {
        Memo::none()
    }
}

impl XDRSerialize for Memo {
    fn write_xdr(&self, mut out: &mut Vec<u8>) -> Result<u64> {
        let xdr_memo = self.to_xdr()?;
        xdr_memo.write_xdr(&mut out).map_err(Error::XdrError)
    }
}

impl XDRDeserialize for Memo {
    fn from_xdr_bytes(buffer: &[u8]) -> Result<(Self, u64)> {
        let (xdr_memo, bytes_read) = xdr::Memo::read_xdr(&buffer).map_err(Error::XdrError)?;
        let res = Memo::from_xdr(&xdr_memo)?;
        Ok((res, bytes_read))
    }
}

#[cfg(test)]
mod tests {
    use super::Memo;
    use crate::error::Error;
    use crate::xdr::{XDRDeserialize, XDRSerialize};

    #[test]
    fn test_defaults_to_none() {
        let memo: Memo = Default::default();
        assert!(memo.is_none());
    }

    #[test]
    fn test_memo_none() {
        let memo = Memo::none();
        assert!(memo.is_none());
        assert!(!memo.is_id());
        assert!(!memo.is_text());
        assert!(!memo.is_hash());
        assert!(!memo.is_return());

        assert_eq!(None, memo.id_value());
        assert_eq!(None, memo.text_value());
        assert_eq!(None, memo.hash_value());
        assert_eq!(None, memo.return_value());
    }

    #[test]
    fn test_memo_id() {
        let memo = Memo::id(1234);
        assert!(!memo.is_none());
        assert!(memo.is_id());
        assert!(!memo.is_text());
        assert!(!memo.is_hash());
        assert!(!memo.is_return());

        assert_eq!(Some(&1234), memo.id_value());
        assert_eq!(None, memo.text_value());
        assert_eq!(None, memo.hash_value());
        assert_eq!(None, memo.return_value());
    }

    #[test]
    fn test_memo_text() {
        let memo = Memo::text("Short text memo").unwrap();
        assert!(!memo.is_none());
        assert!(!memo.is_id());
        assert!(memo.is_text());
        assert!(!memo.is_hash());
        assert!(!memo.is_return());

        assert_eq!(None, memo.id_value());
        assert_eq!("Short text memo", memo.text_value().unwrap());
        assert_eq!(None, memo.hash_value());
        assert_eq!(None, memo.return_value());
    }

    #[test]
    fn test_memo_hash() {
        let memo = Memo::hash(&vec![1, 2, 3, 4, 5]).unwrap();
        assert!(!memo.is_none());
        assert!(!memo.is_id());
        assert!(!memo.is_text());
        assert!(memo.is_hash());
        assert!(!memo.is_return());

        assert_eq!(None, memo.id_value());
        assert_eq!(None, memo.text_value());
        assert_eq!(
            vec![
                1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ],
            memo.hash_value().unwrap()
        );
        assert_eq!(None, memo.return_value());
    }

    #[test]
    fn test_memo_return() {
        let memo = Memo::return_(&vec![1, 2, 3, 4, 5]).unwrap();
        assert!(!memo.is_none());
        assert!(!memo.is_id());
        assert!(!memo.is_text());
        assert!(!memo.is_hash());
        assert!(memo.is_return());

        assert_eq!(None, memo.id_value());
        assert_eq!(None, memo.text_value());
        assert_eq!(None, memo.hash_value());
        assert_eq!(
            vec![
                1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ],
            memo.return_value().unwrap()
        );
    }

    #[test]
    fn test_memo_text_too_long() {
        let result =
            Memo::text("This is a very long text that will not fit in the memo 100% sure.");
        assert!(result.is_err());
    }

    #[test]
    fn test_memo_hash_too_long() {
        let mut hash = Vec::new();
        hash.resize(33, b'1');
        let result = Memo::hash(&hash);
        assert!(result.is_err());
    }

    #[test]
    fn test_memo_return_too_long() {
        let mut hash = Vec::new();
        hash.resize(33, b'1');
        let result = Memo::return_(&hash);
        assert!(result.is_err());
    }

    #[test]
    fn test_memo_none_xdr_ser_de() {
        let original = Memo::none();
        let xdr = original.xdr_base64().unwrap();
        assert_eq!("AAAAAA==", xdr);
        let back = Memo::from_xdr_base64(&xdr).unwrap();
        assert_eq!(original, back);
    }

    #[test]
    fn test_memo_id_xdr_ser_de() {
        let original = Memo::id(u64::MAX);
        let xdr = original.xdr_base64().unwrap();
        assert_eq!("AAAAAv//////////", xdr);
        let back = Memo::from_xdr_base64(&xdr).unwrap();
        assert_eq!(original, back);
    }

    #[test]
    fn test_memo_hash_xdr_ser_de() {
        let mut hash = Vec::new();
        for i in 0..32 {
            hash.push(i as u8);
        }
        let original = Memo::hash(&hash).unwrap();
        let xdr = original.xdr_base64().unwrap();
        assert_eq!("AAAAAwABAgMEBQYHCAkKCwwNDg8QERITFBUWFxgZGhscHR4f", xdr);
        let back = Memo::from_xdr_base64(&xdr).unwrap();
        assert_eq!(original, back);
    }

    #[test]
    fn test_return_hash_xdr_ser_de() {
        let mut hash = Vec::new();
        for i in 0..32 {
            hash.push(i as u8);
        }
        let original = Memo::return_(&hash).unwrap();
        let xdr = original.xdr_base64().unwrap();
        assert_eq!("AAAABAABAgMEBQYHCAkKCwwNDg8QERITFBUWFxgZGhscHR4f", xdr);
        let back = Memo::from_xdr_base64(&xdr).unwrap();
        assert_eq!(original, back);
    }
}
