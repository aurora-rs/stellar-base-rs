//! Represent when a transaction is valid.
use crate::error::{Error, Result};
use crate::xdr;
use crate::xdr::{XDRDeserialize, XDRSerialize};
use chrono::{DateTime, Duration, TimeZone, Utc};
use xdr_rs_serialize::de::XDRIn;
use xdr_rs_serialize::ser::XDROut;

/// The time window in which a transaction is considered valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeBounds {
    lower: Option<DateTime<Utc>>,
    upper: Option<DateTime<Utc>>,
}

impl TimeBounds {
    /// Returns time bounds with the upper bounds set to `duration` in the future.
    pub fn valid_for(duration: Duration) -> TimeBounds {
        let lower = Utc::now();
        let upper = lower + duration;
        TimeBounds {
            lower: None,
            upper: Some(upper),
        }
    }

    /// Returns time bounds such that the transaction is always valid.
    pub fn always_valid() -> TimeBounds {
        TimeBounds {
            lower: None,
            upper: None,
        }
    }

    /// Makes a new time bounds with the lower bound changed.
    pub fn with_lower(&self, lower: DateTime<Utc>) -> Result<TimeBounds> {
        ensure_valid_timestamp(&lower)?;
        match self.upper {
            Some(upper) if upper < lower => Err(Error::InvalidTimeBounds),
            Some(upper) => Ok(TimeBounds {
                lower: Some(lower),
                upper: Some(upper),
            }),
            None => Ok(TimeBounds {
                lower: Some(lower),
                upper: None,
            }),
        }
    }

    /// Makes a new time bounds with the upper bound changed.
    pub fn with_upper(&self, upper: DateTime<Utc>) -> Result<TimeBounds> {
        ensure_valid_timestamp(&upper)?;
        match self.lower {
            Some(lower) if upper < lower => Err(Error::InvalidTimeBounds),
            Some(lower) => Ok(TimeBounds {
                lower: Some(lower),
                upper: Some(upper),
            }),
            None => Ok(TimeBounds {
                lower: None,
                upper: Some(upper),
            }),
        }
    }

    /// Retrieves the time bounds lower bound.
    pub fn lower(&self) -> &Option<DateTime<Utc>> {
        &self.lower
    }

    /// Retrieves a mutable reference to the time bounds lower bound.
    pub fn lower_mut(&mut self) -> &mut Option<DateTime<Utc>> {
        &mut self.lower
    }

    /// Retrieves the time bounds lower bound.
    pub fn upper(&self) -> &Option<DateTime<Utc>> {
        &self.upper
    }

    /// Retrieves a mutable reference to the time bounds lower bound.
    pub fn upper_mut(&mut self) -> &mut Option<DateTime<Utc>> {
        &mut self.upper
    }

    /// Returns the xdr object.
    pub fn to_xdr(&self) -> Result<xdr::TimeBounds> {
        let min_time = match self.lower {
            None => xdr::Uint64::new(0),
            Some(t) => xdr::Uint64::new(t.timestamp() as u64),
        };
        let min_time = xdr::TimePoint::new(min_time);
        let max_time = match self.upper {
            None => xdr::Uint64::new(0),
            Some(t) => xdr::Uint64::new(t.timestamp() as u64),
        };
        let max_time = xdr::TimePoint::new(max_time);
        Ok(xdr::TimeBounds { min_time, max_time })
    }

    /// Creates from the xdr object.
    pub fn from_xdr(x: &xdr::TimeBounds) -> Result<TimeBounds> {
        let min_time_epoch = x.min_time.value.value as i64;
        let max_time_epoch = x.max_time.value.value as i64;

        let mut res = TimeBounds::always_valid();

        if min_time_epoch != 0 {
            res = res.with_lower(Utc.timestamp(min_time_epoch, 0))?;
        }
        if max_time_epoch != 0 {
            res = res.with_upper(Utc.timestamp(max_time_epoch, 0))?;
        }

        Ok(res)
    }
}

impl XDRSerialize for TimeBounds {
    fn write_xdr(&self, out: &mut Vec<u8>) -> Result<u64> {
        let xdr = self.to_xdr()?;
        xdr.write_xdr(out).map_err(Error::XdrError)
    }
}

impl XDRDeserialize for TimeBounds {
    fn from_xdr_bytes(buffer: &[u8]) -> Result<(Self, u64)> {
        let (xdr_timebounds, bytes_read) =
            xdr::TimeBounds::read_xdr(buffer).map_err(Error::XdrError)?;
        let res = TimeBounds::from_xdr(&xdr_timebounds)?;
        Ok((res, bytes_read))
    }
}

fn ensure_valid_timestamp(dt: &DateTime<Utc>) -> Result<()> {
    let ts = dt.timestamp();
    if ts >= 0 {
        Ok(())
    } else {
        Err(Error::InvalidTimeBounds)
    }
}

#[cfg(test)]
mod tests {
    use super::TimeBounds;
    use crate::xdr::{XDRDeserialize, XDRSerialize};
    use chrono::{DateTime, Datelike, Duration, NaiveDateTime, Utc};

    #[test]
    fn test_valid_for() {
        let five_min = Duration::minutes(5);
        let tb = TimeBounds::valid_for(five_min);
        assert_eq!(None, *tb.lower());
        assert_ne!(None, *tb.upper());
    }

    #[test]
    fn test_always_valid() {
        let tb = TimeBounds::always_valid();
        assert_eq!(None, *tb.lower());
        assert_eq!(None, *tb.upper());
    }

    #[test]
    fn test_with_upper_success() {
        let tb = TimeBounds::always_valid().with_upper(Utc::now()).unwrap();
        assert_eq!(None, *tb.lower());
        assert_ne!(None, *tb.upper());
    }

    #[test]
    fn test_with_lower_success() {
        let tb = TimeBounds::always_valid().with_lower(Utc::now()).unwrap();
        assert_ne!(None, *tb.lower());
        assert_eq!(None, *tb.upper());
    }

    #[test]
    fn test_with_both_success() {
        let now = Utc::now();
        let before_now = now - Duration::minutes(1);
        let tb = TimeBounds::always_valid()
            .with_lower(before_now)
            .unwrap()
            .with_upper(now)
            .unwrap();
        assert_ne!(None, *tb.lower());
        assert_ne!(None, *tb.upper());
    }

    #[test]
    fn test_with_upper_before_the_seventies() {
        let res = TimeBounds::always_valid().with_upper(Utc::now().with_year(1960).unwrap());
        assert!(res.is_err());
    }

    #[test]
    fn test_with_lower_before_the_seventies() {
        let res = TimeBounds::always_valid().with_lower(Utc::now().with_year(1960).unwrap());
        assert!(res.is_err());
    }

    #[test]
    fn test_with_upper_before_lower() {
        let now = Utc::now();
        let before_now = now - Duration::minutes(1);
        let res = TimeBounds::always_valid()
            .with_lower(now)
            .unwrap()
            .with_upper(before_now);
        assert!(res.is_err());
    }

    #[test]
    fn test_with_lower_after_upper() {
        let now = Utc::now();
        let before_now = now - Duration::minutes(1);
        let res = TimeBounds::always_valid()
            .with_upper(before_now)
            .unwrap()
            .with_lower(now);
        assert!(res.is_err());
    }

    #[test]
    fn test_serialize_always_valid() {
        let tb = TimeBounds::always_valid();
        let xdr = tb.xdr_base64().unwrap();
        assert_eq!("AAAAAAAAAAAAAAAAAAAAAA==", xdr);
    }

    #[test]
    fn test_serialize_with_bounds() {
        let now = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1594305941, 0), Utc);
        let before_now = now - Duration::minutes(1);
        let tb = TimeBounds::always_valid()
            .with_lower(before_now)
            .unwrap()
            .with_upper(now)
            .unwrap();
        let xdr = tb.xdr_base64().unwrap();
        assert_eq!("AAAAAF8HLVkAAAAAXwctlQ==", xdr);
    }

    #[test]
    fn test_deserialize_always_valid() {
        let expected = TimeBounds::always_valid();
        let tb = TimeBounds::from_xdr_base64("AAAAAAAAAAAAAAAAAAAAAA==").unwrap();
        assert_eq!(expected, tb);
    }

    #[test]
    fn test_deserialize_with_bounds() {
        let now = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1594305941, 0), Utc);
        let before_now = now - Duration::minutes(1);
        let expected = TimeBounds::always_valid()
            .with_lower(before_now)
            .unwrap()
            .with_upper(now)
            .unwrap();
        let tb = TimeBounds::from_xdr_base64("AAAAAF8HLVkAAAAAXwctlQ==").unwrap();
        assert_eq!(expected, tb);
    }
}
