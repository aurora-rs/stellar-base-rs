use crate::xdr;

bitflags! {
    pub struct AccountFlags: u32 {
        const AUTH_REQUIRED = xdr::AccountFlags::AuthRequiredFlag as u32;
        const AUTH_REVOCABLE = xdr::AccountFlags::AuthRevocableFlag as u32;
        const AUTH_IMMUTABLE = xdr::AccountFlags::AuthImmutableFlag as u32;
    }
}

bitflags! {
    pub struct TrustLineFlags: u32 {
        const AUTHORIZED = xdr::TrustLineFlags::AuthorizedFlag as u32;
        const AUTHORIZED_TO_MAINTAIN_LIABILITIES = xdr::TrustLineFlags::AuthorizedToMaintainLiabilitiesFlag as u32;
    }
}
