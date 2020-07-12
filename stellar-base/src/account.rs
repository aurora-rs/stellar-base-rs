use crate::xdr;

bitflags! {
    pub struct AccountFlags: u32 {
        const AUTH_REQUIRED = xdr::AccountFlags::AuthRequiredFlag as u32;
        const AUTH_REVOCABLE = xdr::AccountFlags::AuthRevocableFlag as u32;
        const AUTH_IMMUTABLE = xdr::AccountFlags::AuthImmutableFlag as u32;
    }
}
