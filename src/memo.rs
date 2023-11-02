//! Transaction memo.
use crate::error::{Error, Result};
use crate::xdr;
use crate::xdr::{XDRDeserialize, XDRSerialize};
use xdr_rs_serialize::de::XDRIn;
use xdr_rs_serialize::ser::XDROut;

/// Maximum length of text memo.
pub const MAX_MEMO_TEXT_LEN: usize = 28;

/// Maximum length of hash and return memo.
pub const MAX_HASH_LEN: usize = 32;

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
    pub fn new_none() -> Memo {
        Memo::None
    }

    /// Create new id memo.
    pub fn new_id(id: u64) -> Memo {
        Memo::Id(id)
    }

    /// Create new text memo. `text` must be shorter than 28 bytes.
    pub fn new_text<S: Into<String>>(text: S) -> Result<Memo> {
        let text = text.into();
        if text.len() > MAX_MEMO_TEXT_LEN {
            Err(Error::InvalidMemoText)
        } else {
            Ok(Memo::Text(text))
        }
    }

    /// Create new hash memo.
    pub fn new_hash(hash: &[u8]) -> Result<Memo> {
        if hash.len() > MAX_HASH_LEN {
            Err(Error::InvalidMemoHash)
        } else {
            let mut memo_hash: [u8; 32] = Default::default();
            memo_hash[..hash.len()].copy_from_slice(hash);
            Ok(Memo::Hash(memo_hash))
        }
    }

    /// Creates new return memo.
    pub fn new_return(ret: &[u8]) -> Result<Memo> {
        if ret.len() > MAX_HASH_LEN {
            Err(Error::InvalidMemoReturn)
        } else {
            let mut memo_ret: [u8; 32] = Default::default();
            memo_ret[..ret.len()].copy_from_slice(ret);
            Ok(Memo::Return(memo_ret))
        }
    }

    /// Returns true if memo is None. Returns false otherwise.
    pub fn is_none(&self) -> bool {
        matches!(self, Memo::None)
    }

    /// If the memo is an Id, returns its value. Returns None otherwise.
    pub fn as_id(&self) -> Option<&u64> {
        match *self {
            Memo::Id(ref id) => Some(id),
            _ => None,
        }
    }

    /// If the memo is an Id, returns its mutable value. Returns None otherwise.
    pub fn as_id_mut(&mut self) -> Option<&mut u64> {
        match *self {
            Memo::Id(ref mut id) => Some(id),
            _ => None,
        }
    }

    /// Returns true if memo is Id. Returns false otherwise.
    pub fn is_id(&self) -> bool {
        self.as_id().is_some()
    }

    /// If the memo is a Text, returns its value. Returns None otherwise.
    pub fn as_text(&self) -> Option<&str> {
        match *self {
            Memo::Text(ref text) => Some(text),
            _ => None,
        }
    }

    /// If the memo is a Text, returns its value. Returns None otherwise.
    pub fn as_text_mut(&mut self) -> Option<&mut str> {
        match *self {
            Memo::Text(ref mut text) => Some(text),
            _ => None,
        }
    }

    /// Returns true if memo is Text. Returns false otherwise.
    pub fn is_text(&self) -> bool {
        self.as_text().is_some()
    }

    /// If the memo is a Hash, returns its value. Returns None otherwise.
    pub fn as_hash(&self) -> Option<&[u8; 32]> {
        match *self {
            Memo::Hash(ref hash) => Some(hash),
            _ => None,
        }
    }

    /// If the memo is a Hash, returns its mutable value. Returns None otherwise.
    pub fn as_hash_mut(&mut self) -> Option<&mut [u8; 32]> {
        match *self {
            Memo::Hash(ref mut hash) => Some(hash),
            _ => None,
        }
    }

    /// Returns true if memo is a Hash.
    pub fn is_hash(&self) -> bool {
        self.as_hash().is_some()
    }

    /// If the memo is a Return, returns its value. Returns None otherwise.
    pub fn as_return(&self) -> Option<&[u8; 32]> {
        match *self {
            Memo::Return(ref hash) => Some(hash),
            _ => None,
        }
    }

    /// If the memo is a Return, returns its mutable value. Returns None otherwise.
    pub fn as_return_mut(&mut self) -> Option<&mut [u8; 32]> {
        match *self {
            Memo::Return(ref mut hash) => Some(hash),
            _ => None,
        }
    }

    /// Returns true if memo is a Return.
    pub fn is_return(&self) -> bool {
        self.as_return().is_some()
    }

    /// Returns the memo xdr object.
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

    /// Creates a new memo from the xdr object.
    pub fn from_xdr(x: &xdr::Memo) -> Result<Memo> {
        match x {
            xdr::Memo::MemoNone(()) => Ok(Memo::new_none()),
            xdr::Memo::MemoText(text) => Memo::new_text(text),
            xdr::Memo::MemoId(id) => Ok(Memo::new_id(id.value)),
            xdr::Memo::MemoHash(hash) => Memo::new_hash(&hash.value),
            xdr::Memo::MemoReturn(ret) => Memo::new_return(&ret.value),
        }
    }
}

impl Default for Memo {
    fn default() -> Memo {
        Memo::new_none()
    }
}

impl XDRSerialize for Memo {
    fn write_xdr(&self, out: &mut Vec<u8>) -> Result<u64> {
        let xdr_memo = self.to_xdr()?;
        xdr_memo.write_xdr(out).map_err(Error::XdrError)
    }
}

impl XDRDeserialize for Memo {
    fn from_xdr_bytes(buffer: &[u8]) -> Result<(Self, u64)> {
        let (xdr_memo, bytes_read) = xdr::Memo::read_xdr(buffer).map_err(Error::XdrError)?;
        let res = Memo::from_xdr(&xdr_memo)?;
        Ok((res, bytes_read))
    }
}

#[cfg(test)]
mod tests {
    use super::Memo;
    use crate::xdr::{XDRDeserialize, XDRSerialize};

    #[test]
    fn test_defaults_to_none() {
        let memo: Memo = Default::default();
        assert!(memo.is_none());
    }

    #[test]
    fn test_memo_none() {
        let memo = Memo::new_none();
        assert!(memo.is_none());
        assert!(!memo.is_id());
        assert!(!memo.is_text());
        assert!(!memo.is_hash());
        assert!(!memo.is_return());

        assert_eq!(None, memo.as_id());
        assert_eq!(None, memo.as_text());
        assert_eq!(None, memo.as_hash());
        assert_eq!(None, memo.as_return());
    }

    #[test]
    fn test_memo_id() {
        let mut memo = Memo::new_id(1234);
        assert!(!memo.is_none());
        assert!(memo.is_id());
        assert!(!memo.is_text());
        assert!(!memo.is_hash());
        assert!(!memo.is_return());

        assert_eq!(Some(&1234), memo.as_id());
        assert_eq!(None, memo.as_text());
        assert_eq!(None, memo.as_hash());
        assert_eq!(None, memo.as_return());

        *memo.as_id_mut().unwrap() = 456;
        assert_eq!(Some(&456), memo.as_id());
    }

    #[test]
    fn test_memo_text() {
        let memo = Memo::new_text("Short text memo").unwrap();
        assert!(!memo.is_none());
        assert!(!memo.is_id());
        assert!(memo.is_text());
        assert!(!memo.is_hash());
        assert!(!memo.is_return());

        assert_eq!(None, memo.as_id());
        assert_eq!("Short text memo", memo.as_text().unwrap());
        assert_eq!(None, memo.as_hash());
        assert_eq!(None, memo.as_return());
    }

    #[test]
    fn test_memo_hash() {
        let mut memo = Memo::new_hash(&[1, 2, 3, 4, 5]).unwrap();
        assert!(!memo.is_none());
        assert!(!memo.is_id());
        assert!(!memo.is_text());
        assert!(memo.is_hash());
        assert!(!memo.is_return());

        assert_eq!(None, memo.as_id());
        assert_eq!(None, memo.as_text());
        assert_eq!(
            vec![
                1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ],
            memo.as_hash().unwrap()
        );
        assert_eq!(None, memo.as_return());

        memo.as_hash_mut().unwrap()[super::MAX_HASH_LEN - 1] = 9;

        assert_eq!(
            vec![
                1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 9
            ],
            memo.as_hash().unwrap()
        );
    }

    #[test]
    fn test_memo_return() {
        let mut memo = Memo::new_return(&[1, 2, 3, 4, 5]).unwrap();
        assert!(!memo.is_none());
        assert!(!memo.is_id());
        assert!(!memo.is_text());
        assert!(!memo.is_hash());
        assert!(memo.is_return());

        assert_eq!(None, memo.as_id());
        assert_eq!(None, memo.as_text());
        assert_eq!(None, memo.as_hash());
        assert_eq!(
            vec![
                1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0
            ],
            memo.as_return().unwrap()
        );

        memo.as_return_mut().unwrap()[super::MAX_HASH_LEN - 1] = 9;
        assert_eq!(
            vec![
                1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 9
            ],
            memo.as_return().unwrap()
        );
    }

    #[test]
    fn test_memo_text_too_long() {
        let result =
            Memo::new_text("This is a very long text that will not fit in the memo 100% sure.");
        assert!(result.is_err());
    }

    #[test]
    fn test_memo_hash_too_long() {
        let mut hash = Vec::new();
        hash.resize(33, b'1');
        let result = Memo::new_hash(&hash);
        assert!(result.is_err());
    }

    #[test]
    fn test_memo_return_too_long() {
        let mut hash = Vec::new();
        hash.resize(33, b'1');
        let result = Memo::new_return(&hash);
        assert!(result.is_err());
    }

    #[test]
    fn test_memo_none_xdr_ser_de() {
        let original = Memo::new_none();
        let xdr = original.xdr_base64().unwrap();
        assert_eq!("AAAAAA==", xdr);
        let back = Memo::from_xdr_base64(&xdr).unwrap();
        assert_eq!(original, back);
    }

    #[test]
    fn test_memo_id_xdr_ser_de() {
        let original = Memo::new_id(u64::MAX);
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
        let original = Memo::new_hash(&hash).unwrap();
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
        let original = Memo::new_return(&hash).unwrap();
        let xdr = original.xdr_base64().unwrap();
        assert_eq!("AAAABAABAgMEBQYHCAkKCwwNDg8QERITFBUWFxgZGhscHR4f", xdr);
        let back = Memo::from_xdr_base64(&xdr).unwrap();
        assert_eq!(original, back);
    }
}
