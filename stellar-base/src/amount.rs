use crate::error::{Error, Result};
use crate::xdr;
use crate::xdr::{XDRDeserialize, XDRSerialize};
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::cast::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use std::convert::TryFrom;
use std::ops::Mul;
use std::str::FromStr;
use xdr_rs_serialize::de::XDRIn;
use xdr_rs_serialize::ser::XDROut;

const STELLAR_SCALE: u32 = 7;

/// Amount in XLM.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount {
    inner: Decimal,
}

impl Amount {
    /// Create from amount specified in stroops.
    pub fn from_stroops(stroops: Stroops) -> Result<Amount> {
        let inner = Decimal::new(stroops.0, STELLAR_SCALE);
        Ok(Amount { inner })
    }

    /// Convert to stroops.
    pub fn as_stroops(&self) -> Result<Stroops> {
        self.clone().into_stroops()
    }

    /// Convert into stroops.
    pub fn into_stroops(self) -> Result<Stroops> {
        let scale = self.inner.scale();
        if scale != STELLAR_SCALE {
            return Err(Error::InvalidAmountScale);
        }
        let res = self.inner * Decimal::new(100_000_000, 1);
        match res.to_i64() {
            Some(stroops) => Ok(Stroops::new(stroops)),
            None => Err(Error::InvalidStroopsAmount),
        }
    }
}

impl FromStr for Amount {
    type Err = Error;

    fn from_str(s: &str) -> Result<Amount> {
        let mut inner = Decimal::from_str(&s)?;
        // Check we don't lose precision
        let scale = inner.scale();
        if scale > STELLAR_SCALE {
            Err(Error::InvalidAmountScale)
        } else {
            inner.rescale(STELLAR_SCALE);
            Ok(Amount { inner })
        }
    }
}

/// Amount in stroops.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stroops(pub i64);

impl Stroops {
    /// Create from stroops.
    pub fn new(amount: i64) -> Stroops {
        Stroops(amount)
    }

    pub fn max() -> Stroops {
        Stroops(i64::MAX)
    }

    pub fn to_i64(&self) -> i64 {
        self.0
    }

    pub fn to_xdr_int64(&self) -> Result<xdr::Int64> {
        Ok(xdr::Int64::new(self.0))
    }

    pub fn to_xdr_uint32(&self) -> Result<xdr::Uint32> {
        if self.0 >= 0 {
            Ok(xdr::Uint32::new(self.0 as u32))
        } else {
            Err(Error::NegativeStroops)
        }
    }

    pub fn from_xdr_int64(x: &xdr::Int64) -> Result<Stroops> {
        Ok(Stroops::new(x.value))
    }

    pub fn from_xdr_uint32(x: &xdr::Uint32) -> Result<Stroops> {
        Ok(Stroops::new(x.value as i64))
    }
}

impl Mul<usize> for Stroops {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        Stroops(self.0 * rhs as i64)
    }
}

/// Price in fractional representation.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Price {
    inner: Ratio<i32>,
}

impl Price {
    /// Create from numerator and denominator.
    pub fn new(numerator: i32, denominator: i32) -> Price {
        let inner = Ratio::new_raw(numerator, denominator);
        Price { inner }
    }

    /// Retrievs the price numerator.
    pub fn numerator(&self) -> i32 {
        *self.inner.numer()
    }

    /// Retries the price denominator.
    pub fn denominator(&self) -> i32 {
        *self.inner.denom()
    }

    /// Returns a reduced copy.
    pub fn reduced(&self) -> Price {
        let inner = self.inner.reduced();
        Price { inner }
    }

    pub fn to_xdr(&self) -> Result<xdr::Price> {
        Ok(xdr::Price {
            n: xdr::Int32::new(self.numerator()),
            d: xdr::Int32::new(self.denominator()),
        })
    }

    pub fn from_xdr(x: &xdr::Price) -> Result<Price> {
        Ok(Price::new(x.n.value, x.d.value))
    }
}

impl FromStr for Price {
    type Err = Error;

    fn from_str(s: &str) -> Result<Price> {
        if s.is_empty() {
            return Err(Error::ParsePriceError);
        }
        let max_i32 = Decimal::new(i32::MAX as i64, 0);
        let mut number = Decimal::from_str(s).map_err(|_| Error::ParsePriceError)?;
        let zero = Decimal::new(0, 0);
        let one = Decimal::new(1, 0);

        let mut fractions = vec![(zero, one), (one, zero)];
        let mut i = 2;
        loop {
            if number > max_i32 {
                break;
            }

            let whole = number.floor();
            let fract = number - whole;
            let h = whole * fractions[i - 1].0 + fractions[i - 2].0;
            let k = whole * fractions[i - 1].1 + fractions[i - 2].1;
            if (k >= max_i32) || (h >= max_i32) {
                break;
            }
            fractions.push((h, k));
            if fract == zero {
                break;
            }
            number = one / fract;
            i += 1;
        }
        match fractions.last() {
            None => Err(Error::ParsePriceError),
            Some((num, den)) => {
                let num = num.to_i32();
                let den = den.to_i32();
                match (num, den) {
                    (Some(0), _) => Err(Error::ParsePriceError),
                    (_, Some(0)) => Err(Error::ParsePriceError),
                    (Some(num), Some(den)) => Ok(Price::new(num, den)),
                    _ => Err(Error::ParsePriceError),
                }
            }
        }
    }
}

impl TryFrom<Amount> for Stroops {
    type Error = Error;

    fn try_from(amount: Amount) -> std::result::Result<Self, Self::Error> {
        amount.as_stroops()
    }
}

impl XDRSerialize for Price {
    fn write_xdr(&self, mut out: &mut Vec<u8>) -> Result<u64> {
        let xdr_price = self.to_xdr()?;
        xdr_price.write_xdr(&mut out).map_err(Error::XdrError)
    }
}

impl XDRDeserialize for Price {
    fn from_xdr_bytes(buffer: &[u8]) -> Result<(Self, u64)> {
        let (xdr_price, bytes_read) = xdr::Price::read_xdr(&buffer).map_err(Error::XdrError)?;
        let res = Price::from_xdr(&xdr_price)?;
        Ok((res, bytes_read))
    }
}

#[cfg(test)]
mod tests {
    use super::{Amount, Price, Stroops};
    use crate::xdr::{XDRDeserialize, XDRSerialize};
    use std::str;
    use std::str::FromStr;

    #[test]
    fn test_amount_from_str() {
        let amount1 = str::parse::<Amount>("123.4567891").unwrap();
        let amount2 = str::parse::<Amount>("123.4567891").unwrap();
        let amount3 = str::parse::<Amount>("123.4567890").unwrap();

        assert_eq!(amount1, amount2);
        assert_ne!(amount1, amount3);
        assert!(amount3 < amount1);
    }

    #[test]
    fn test_error_too_many_decimals() {
        let res = str::parse::<Amount>("123.45678901");
        assert!(res.is_err());
    }

    #[test]
    fn test_amount_as_stroops() {
        let amount = str::parse::<Amount>("123.45678").unwrap();
        let stroops = amount.as_stroops().unwrap();
        assert_eq!(stroops, Stroops::new(1234567800));
    }

    #[test]
    fn test_price_from_str() {
        let one_22 = "1".repeat(22);
        let one_big = "1".repeat(1000000);
        let zero_one = "0.".to_string() + &"1".repeat(1000);
        let test_cases = vec![
            ("0.1", Some((1, 10))),
            ("0.01", Some((1, 100))),
            ("0.001", Some((1, 1000))),
            ("543.017930", Some((54301793, 100000))),
            ("319.69983", Some((31969983, 100000))),
            ("0.93", Some((93, 100))),
            ("0.5", Some((1, 2))),
            ("1.730", Some((173, 100))),
            ("0.85334384", Some((5333399, 6250000))),
            ("5.5", Some((11, 2))),
            ("2.72783", Some((272783, 100000))),
            ("638082.0", Some((638082, 1))),
            ("58.04", Some((1451, 25))),
            ("41.265", Some((8253, 200))),
            ("5.1476", Some((12869, 2500))),
            ("95.14", Some((4757, 50))),
            ("0.74580", Some((3729, 5000))),
            ("4119.0", Some((4119, 1))),
            // Expensive imputs
            (&one_22, None),
            (&one_big, None),
            // (&zero_one, None),
            ("1E9223372036854775807", None),
            ("1e9223372036854775807", None),
        ];

        for (test_str, expected_res) in test_cases {
            let res = Price::from_str(test_str);
            match expected_res {
                None => {
                    assert!(res.is_err());
                }
                Some((num, den)) => {
                    let price = res.unwrap();
                    assert_eq!(num, price.numerator());
                    assert_eq!(den, price.denominator());
                }
            }
        }
    }

    #[test]
    fn test_price_xdr_ser() {
        let price = Price::new(123, 456);
        let xdr = price.xdr_base64().unwrap();
        assert_eq!("AAAAewAAAcg=", xdr);
    }

    #[test]
    fn test_price_xdr_de() {
        let expected = Price::new(123, 456);
        let price = Price::from_xdr_base64("AAAAewAAAcg=").unwrap();
        assert_eq!(expected, price);
    }
}
