// Module  is generated from:
//
//  stellar-proto/Stellar-ledger-entries.x
//  stellar-proto/Stellar-ledger.x
//  stellar-proto/Stellar-overlay.x
//  stellar-proto/Stellar-SCP.x
//  stellar-proto/Stellar-transaction.x
//  stellar-proto/Stellar-types.x
//
// DO NOT EDIT or your changes may be overwritten
//! Stellar XDR types
#![allow(dead_code)]
use std::io::Write;
#[allow(unused_imports)]
use xdr_rs_serialize::de::{
    read_fixed_array, read_fixed_array_json, read_fixed_opaque, read_fixed_opaque_json,
    read_var_array, read_var_array_json, read_var_opaque, read_var_opaque_json, read_var_string,
    read_var_string_json, XDRIn,
};
use xdr_rs_serialize::error::Error;
#[allow(unused_imports)]
use xdr_rs_serialize::ser::{
    write_fixed_array, write_fixed_array_json, write_fixed_opaque, write_fixed_opaque_json,
    write_var_array, write_var_array_json, write_var_opaque, write_var_opaque_json,
    write_var_string, write_var_string_json, XDROut,
};

// AccountId is an XDR Typedef defines as:
//
//   typedef PublicKey AccountID;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct AccountId {
    // TODO
    pub value: PublicKey,
}

impl AccountId {
    pub fn new(value: PublicKey) -> AccountId {
        AccountId { value }
    }
}

// Thresholds is an XDR Typedef defines as:
//
//   typedef opaque Thresholds[4];
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Thresholds {
    #[array(fixed = 4)]
    pub value: Vec<u8>,
}

impl Thresholds {
    pub fn new(value: Vec<u8>) -> Thresholds {
        Thresholds { value }
    }
}

// String32 is an XDR Typedef defines as:
//
//   typedef string string32<32>;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct String32 {
    #[array(var = 32)]
    pub value: String,
}

impl String32 {
    pub fn new(value: String) -> String32 {
        String32 { value }
    }
}

// String64 is an XDR Typedef defines as:
//
//   typedef string string64<64>;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct String64 {
    #[array(var = 64)]
    pub value: String,
}

impl String64 {
    pub fn new(value: String) -> String64 {
        String64 { value }
    }
}

// SequenceNumber is an XDR Typedef defines as:
//
//   typedef int64 SequenceNumber;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct SequenceNumber {
    // TODO
    pub value: Int64,
}

impl SequenceNumber {
    pub fn new(value: Int64) -> SequenceNumber {
        SequenceNumber { value }
    }
}

// TimePoint is an XDR Typedef defines as:
//
//   typedef uint64 TimePoint;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TimePoint {
    // TODO
    pub value: Uint64,
}

impl TimePoint {
    pub fn new(value: Uint64) -> TimePoint {
        TimePoint { value }
    }
}

// DataValue is an XDR Typedef defines as:
//
//   typedef opaque DataValue<64>;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct DataValue {
    #[array(var = 64)]
    pub value: Vec<u8>,
}

impl DataValue {
    pub fn new(value: Vec<u8>) -> DataValue {
        DataValue { value }
    }
}

// PoolId is an XDR Typedef defines as:
//
//   typedef Hash PoolID;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct PoolId {
    // TODO
    pub value: Hash,
}

impl PoolId {
    pub fn new(value: Hash) -> PoolId {
        PoolId { value }
    }
}

// AssetCode4 is an XDR Typedef defines as:
//
//   typedef opaque AssetCode4[4];
//
#[derive(Debug, XDROut, XDRIn)]
pub struct AssetCode4 {
    #[array(fixed = 4)]
    pub value: Vec<u8>,
}

impl AssetCode4 {
    pub fn new(value: Vec<u8>) -> AssetCode4 {
        AssetCode4 { value }
    }
}

// AssetCode12 is an XDR Typedef defines as:
//
//   typedef opaque AssetCode12[12];
//
#[derive(Debug, XDROut, XDRIn)]
pub struct AssetCode12 {
    #[array(fixed = 12)]
    pub value: Vec<u8>,
}

impl AssetCode12 {
    pub fn new(value: Vec<u8>) -> AssetCode12 {
        AssetCode12 { value }
    }
}

// AssetType is an XDR Enum defines as:
//
//   enum AssetType
//    {
//        ASSET_TYPE_NATIVE = 0,
//        ASSET_TYPE_CREDIT_ALPHANUM4 = 1,
//        ASSET_TYPE_CREDIT_ALPHANUM12 = 2,
//        ASSET_TYPE_POOL_SHARE = 3
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum AssetType {
    AssetTypeNative = 0,
    AssetTypeCreditAlphanum4 = 1,
    AssetTypeCreditAlphanum12 = 2,
    AssetTypePoolShare = 3,
}

// AssetCode is an XDR Union defines as:
//
//   union AssetCode switch (AssetType type)
//    {
//    case ASSET_TYPE_CREDIT_ALPHANUM4:
//        AssetCode4 assetCode4;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM12:
//        AssetCode12 assetCode12;
//
//        // add other asset types here in the future
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum AssetCode {
    // IDEN ASSET_TYPE_CREDIT_ALPHANUM4
    #[discriminant(value = "1")]
    AssetTypeCreditAlphanum4(AssetCode4),
    // IDEN ASSET_TYPE_CREDIT_ALPHANUM12
    #[discriminant(value = "2")]
    AssetTypeCreditAlphanum12(AssetCode12),
}

// AlphaNum4 is an XDR Struct defines as:
//
//   struct AlphaNum4
//    {
//        AssetCode4 assetCode;
//        AccountID issuer;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct AlphaNum4 {
    // TODO
    pub asset_code: AssetCode4,
    // TODO
    pub issuer: AccountId,
}

// AlphaNum12 is an XDR Struct defines as:
//
//   struct AlphaNum12
//    {
//        AssetCode12 assetCode;
//        AccountID issuer;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct AlphaNum12 {
    // TODO
    pub asset_code: AssetCode12,
    // TODO
    pub issuer: AccountId,
}

// Asset is an XDR Union defines as:
//
//   union Asset switch (AssetType type)
//    {
//    case ASSET_TYPE_NATIVE: // Not credit
//        void;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM4:
//        AlphaNum4 alphaNum4;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM12:
//        AlphaNum12 alphaNum12;
//
//        // add other asset types here in the future
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum Asset {
    // IDEN ASSET_TYPE_NATIVE
    #[discriminant(value = "0")]
    AssetTypeNative(()),
    // IDEN ASSET_TYPE_CREDIT_ALPHANUM4
    #[discriminant(value = "1")]
    AssetTypeCreditAlphanum4(AlphaNum4),
    // IDEN ASSET_TYPE_CREDIT_ALPHANUM12
    #[discriminant(value = "2")]
    AssetTypeCreditAlphanum12(AlphaNum12),
}

// Price is an XDR Struct defines as:
//
//   struct Price
//    {
//        int32 n; // numerator
//        int32 d; // denominator
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Price {
    // TODO
    pub n: Int32,
    // TODO
    pub d: Int32,
}

// Liabilities is an XDR Struct defines as:
//
//   struct Liabilities
//    {
//        int64 buying;
//        int64 selling;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Liabilities {
    // TODO
    pub buying: Int64,
    // TODO
    pub selling: Int64,
}

// ThresholdIndexes is an XDR Enum defines as:
//
//   enum ThresholdIndexes
//    {
//        THRESHOLD_MASTER_WEIGHT = 0,
//        THRESHOLD_LOW = 1,
//        THRESHOLD_MED = 2,
//        THRESHOLD_HIGH = 3
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ThresholdIndexes {
    ThresholdMasterWeight = 0,
    ThresholdLow = 1,
    ThresholdMed = 2,
    ThresholdHigh = 3,
}

// LedgerEntryType is an XDR Enum defines as:
//
//   enum LedgerEntryType
//    {
//        ACCOUNT = 0,
//        TRUSTLINE = 1,
//        OFFER = 2,
//        DATA = 3,
//        CLAIMABLE_BALANCE = 4,
//        LIQUIDITY_POOL = 5
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerEntryType {
    Account = 0,
    Trustline = 1,
    Offer = 2,
    Data = 3,
    ClaimableBalance = 4,
    LiquidityPool = 5,
}

// Signer is an XDR Struct defines as:
//
//   struct Signer
//    {
//        SignerKey key;
//        uint32 weight; // really only need 1 byte
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Signer {
    // TODO
    pub key: SignerKey,
    // TODO
    pub weight: Uint32,
}

// AccountFlags is an XDR Enum defines as:
//
//   enum AccountFlags
//    { // masks for each flag
//
//        // Flags set on issuer accounts
//        // TrustLines are created with authorized set to "false" requiring
//        // the issuer to set it for each TrustLine
//        AUTH_REQUIRED_FLAG = 0x1,
//        // If set, the authorized flag in TrustLines can be cleared
//        // otherwise, authorization cannot be revoked
//        AUTH_REVOCABLE_FLAG = 0x2,
//        // Once set, causes all AUTH_* flags to be read-only
//        AUTH_IMMUTABLE_FLAG = 0x4,
//        // Trustlines are created with clawback enabled set to "true",
//        // and claimable balances created from those trustlines are created
//        // with clawback enabled set to "true"
//        AUTH_CLAWBACK_ENABLED_FLAG = 0x8
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum AccountFlags {
    AuthRequiredFlag = 1,
    AuthRevocableFlag = 2,
    AuthImmutableFlag = 4,
    AuthClawbackEnabledFlag = 8,
}

// MaskAccountFlags is an XDR Const defines as:
//
//   const MASK_ACCOUNT_FLAGS = 0x7;
//
const MASK_ACCOUNT_FLAGS: u64 = 0x7;

// MaskAccountFlagsV17 is an XDR Const defines as:
//
//   const MASK_ACCOUNT_FLAGS_V17 = 0xF;
//
const MASK_ACCOUNT_FLAGS_V17: u64 = 0xF;

// MaxSigners is an XDR Const defines as:
//
//   const MAX_SIGNERS = 20;
//
const MAX_SIGNERS: u64 = 20;

// SponsorshipDescriptor is an XDR Typedef defines as:
//
//   typedef AccountID* SponsorshipDescriptor;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct SponsorshipDescriptor {
    // TODO
    pub value: Option<AccountId>,
}

impl SponsorshipDescriptor {
    pub fn new(value: Option<AccountId>) -> SponsorshipDescriptor {
        SponsorshipDescriptor { value }
    }
}

// AccountEntryExtensionV2Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum AccountEntryExtensionV2Ext {
    // NO IDEN 0
    V0(()),
}

// AccountEntryExtensionV2 is an XDR Struct defines as:
//
//   struct AccountEntryExtensionV2
//    {
//        uint32 numSponsored;
//        uint32 numSponsoring;
//        SponsorshipDescriptor signerSponsoringIDs<MAX_SIGNERS>;
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct AccountEntryExtensionV2 {
    // TODO
    pub num_sponsored: Uint32,
    // TODO
    pub num_sponsoring: Uint32,
    // TODO
    pub signer_sponsoring_i_ds: Vec<SponsorshipDescriptor>,
    // TODO
    pub ext: AccountEntryExtensionV2Ext,
}

// AccountEntryExtensionV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 2:
//            AccountEntryExtensionV2 v2;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum AccountEntryExtensionV1Ext {
    // NO IDEN 0
    V0(()),
    // NO IDEN 2
    V2(AccountEntryExtensionV2),
}

// AccountEntryExtensionV1 is an XDR Struct defines as:
//
//   struct AccountEntryExtensionV1
//    {
//        Liabilities liabilities;
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 2:
//            AccountEntryExtensionV2 v2;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct AccountEntryExtensionV1 {
    // TODO
    pub liabilities: Liabilities,
    // TODO
    pub ext: AccountEntryExtensionV1Ext,
}

// AccountEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            AccountEntryExtensionV1 v1;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum AccountEntryExt {
    // NO IDEN 0
    V0(()),
    // NO IDEN 1
    V1(AccountEntryExtensionV1),
}

// AccountEntry is an XDR Struct defines as:
//
//   struct AccountEntry
//    {
//        AccountID accountID;      // master public key for this account
//        int64 balance;            // in stroops
//        SequenceNumber seqNum;    // last sequence number used for this account
//        uint32 numSubEntries;     // number of sub-entries this account has
//                                  // drives the reserve
//        AccountID* inflationDest; // Account to vote for during inflation
//        uint32 flags;             // see AccountFlags
//
//        string32 homeDomain; // can be used for reverse federation and memo lookup
//
//        // fields used for signatures
//        // thresholds stores unsigned bytes: [weight of master|low|medium|high]
//        Thresholds thresholds;
//
//        Signer signers<MAX_SIGNERS>; // possible signers for this account
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            AccountEntryExtensionV1 v1;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct AccountEntry {
    // TODO
    pub account_id: AccountId,
    // TODO
    pub balance: Int64,
    // TODO
    pub seq_num: SequenceNumber,
    // TODO
    pub num_sub_entries: Uint32,
    // TODO
    pub inflation_dest: Option<AccountId>,
    // TODO
    pub flags: Uint32,
    // TODO
    pub home_domain: String32,
    // TODO
    pub thresholds: Thresholds,
    // TODO
    pub signers: Vec<Signer>,
    // TODO
    pub ext: AccountEntryExt,
}

// TrustLineFlags is an XDR Enum defines as:
//
//   enum TrustLineFlags
//    {
//        // issuer has authorized account to perform transactions with its credit
//        AUTHORIZED_FLAG = 1,
//        // issuer has authorized account to maintain and reduce liabilities for its
//        // credit
//        AUTHORIZED_TO_MAINTAIN_LIABILITIES_FLAG = 2,
//        // issuer has specified that it may clawback its credit, and that claimable
//        // balances created with its credit may also be clawed back
//        TRUSTLINE_CLAWBACK_ENABLED_FLAG = 4
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum TrustLineFlags {
    AuthorizedFlag = 1,
    AuthorizedToMaintainLiabilitiesFlag = 2,
    TrustlineClawbackEnabledFlag = 4,
}

// MaskTrustlineFlags is an XDR Const defines as:
//
//   const MASK_TRUSTLINE_FLAGS = 1;
//
const MASK_TRUSTLINE_FLAGS: u64 = 1;

// MaskTrustlineFlagsV13 is an XDR Const defines as:
//
//   const MASK_TRUSTLINE_FLAGS_V13 = 3;
//
const MASK_TRUSTLINE_FLAGS_V13: u64 = 3;

// MaskTrustlineFlagsV17 is an XDR Const defines as:
//
//   const MASK_TRUSTLINE_FLAGS_V17 = 7;
//
const MASK_TRUSTLINE_FLAGS_V17: u64 = 7;

// LiquidityPoolType is an XDR Enum defines as:
//
//   enum LiquidityPoolType
//    {
//        LIQUIDITY_POOL_CONSTANT_PRODUCT = 0
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum LiquidityPoolType {
    LiquidityPoolConstantProduct = 0,
}

// TrustLineAsset is an XDR Union defines as:
//
//   union TrustLineAsset switch (AssetType type)
//    {
//    case ASSET_TYPE_NATIVE: // Not credit
//        void;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM4:
//        AlphaNum4 alphaNum4;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM12:
//        AlphaNum12 alphaNum12;
//
//    case ASSET_TYPE_POOL_SHARE:
//        PoolID liquidityPoolID;
//
//        // add other asset types here in the future
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum TrustLineAsset {
    // IDEN ASSET_TYPE_NATIVE
    #[discriminant(value = "0")]
    AssetTypeNative(()),
    // IDEN ASSET_TYPE_CREDIT_ALPHANUM4
    #[discriminant(value = "1")]
    AssetTypeCreditAlphanum4(AlphaNum4),
    // IDEN ASSET_TYPE_CREDIT_ALPHANUM12
    #[discriminant(value = "2")]
    AssetTypeCreditAlphanum12(AlphaNum12),
    // IDEN ASSET_TYPE_POOL_SHARE
    #[discriminant(value = "3")]
    AssetTypePoolShare(PoolId),
}

// TrustLineEntryExtensionV2Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum TrustLineEntryExtensionV2Ext {
    // NO IDEN 0
    V0(()),
}

// TrustLineEntryExtensionV2 is an XDR Struct defines as:
//
//   struct TrustLineEntryExtensionV2
//    {
//        int32 liquidityPoolUseCount;
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TrustLineEntryExtensionV2 {
    // TODO
    pub liquidity_pool_use_count: Int32,
    // TODO
    pub ext: TrustLineEntryExtensionV2Ext,
}

// TrustLineEntryV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//                {
//                case 0:
//                    void;
//                case 2:
//                    TrustLineEntryExtensionV2 v2;
//                }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum TrustLineEntryV1Ext {
    // NO IDEN 0
    #[discriminant(value = "0")]
    V0(()),
    // NO IDEN 2
    #[discriminant(value = "2")]
    V2(TrustLineEntryExtensionV2),
}

// TrustLineEntryV1 is an XDR NestedStruct defines as:
//
//   struct
//            {
//                Liabilities liabilities;
//
//                union switch (int v)
//                {
//                case 0:
//                    void;
//                case 2:
//                    TrustLineEntryExtensionV2 v2;
//                }
//                ext;
//            }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TrustLineEntryV1 {
    // TODO
    pub liabilities: Liabilities,
    // TODO
    pub ext: TrustLineEntryV1Ext,
}

// TrustLineEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            struct
//            {
//                Liabilities liabilities;
//
//                union switch (int v)
//                {
//                case 0:
//                    void;
//                case 2:
//                    TrustLineEntryExtensionV2 v2;
//                }
//                ext;
//            } v1;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum TrustLineEntryExt {
    // NO IDEN 0
    #[discriminant(value = "0")]
    V0(()),
    // NO IDEN 1
    #[discriminant(value = "1")]
    V1(TrustLineEntryV1),
}

// TrustLineEntry is an XDR Struct defines as:
//
//   struct TrustLineEntry
//    {
//        AccountID accountID;  // account this trustline belongs to
//        TrustLineAsset asset; // type of asset (with issuer)
//        int64 balance;        // how much of this asset the user has.
//                              // Asset defines the unit for this;
//
//        int64 limit;  // balance cannot be above this
//        uint32 flags; // see TrustLineFlags
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            struct
//            {
//                Liabilities liabilities;
//
//                union switch (int v)
//                {
//                case 0:
//                    void;
//                case 2:
//                    TrustLineEntryExtensionV2 v2;
//                }
//                ext;
//            } v1;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TrustLineEntry {
    // TODO
    pub account_id: AccountId,
    // TODO
    pub asset: TrustLineAsset,
    // TODO
    pub balance: Int64,
    // TODO
    pub limit: Int64,
    // TODO
    pub flags: Uint32,
    // TODO
    pub ext: TrustLineEntryExt,
}

// OfferEntryFlags is an XDR Enum defines as:
//
//   enum OfferEntryFlags
//    {
//        // issuer has authorized account to perform transactions with its credit
//        PASSIVE_FLAG = 1
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum OfferEntryFlags {
    PassiveFlag = 1,
}

// MaskOfferentryFlags is an XDR Const defines as:
//
//   const MASK_OFFERENTRY_FLAGS = 1;
//
const MASK_OFFERENTRY_FLAGS: u64 = 1;

// OfferEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum OfferEntryExt {
    // NO IDEN 0
    V0(()),
}

// OfferEntry is an XDR Struct defines as:
//
//   struct OfferEntry
//    {
//        AccountID sellerID;
//        int64 offerID;
//        Asset selling; // A
//        Asset buying;  // B
//        int64 amount;  // amount of A
//
//        /* price for this offer:
//            price of A in terms of B
//            price=AmountB/AmountA=priceNumerator/priceDenominator
//            price is after fees
//        */
//        Price price;
//        uint32 flags; // see OfferEntryFlags
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct OfferEntry {
    // TODO
    pub seller_id: AccountId,
    // TODO
    pub offer_id: Int64,
    // TODO
    pub selling: Asset,
    // TODO
    pub buying: Asset,
    // TODO
    pub amount: Int64,
    // TODO
    pub price: Price,
    // TODO
    pub flags: Uint32,
    // TODO
    pub ext: OfferEntryExt,
}

// DataEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum DataEntryExt {
    // NO IDEN 0
    V0(()),
}

// DataEntry is an XDR Struct defines as:
//
//   struct DataEntry
//    {
//        AccountID accountID; // account this data belongs to
//        string64 dataName;
//        DataValue dataValue;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct DataEntry {
    // TODO
    pub account_id: AccountId,
    // TODO
    pub data_name: String64,
    // TODO
    pub data_value: DataValue,
    // TODO
    pub ext: DataEntryExt,
}

// ClaimPredicateType is an XDR Enum defines as:
//
//   enum ClaimPredicateType
//    {
//        CLAIM_PREDICATE_UNCONDITIONAL = 0,
//        CLAIM_PREDICATE_AND = 1,
//        CLAIM_PREDICATE_OR = 2,
//        CLAIM_PREDICATE_NOT = 3,
//        CLAIM_PREDICATE_BEFORE_ABSOLUTE_TIME = 4,
//        CLAIM_PREDICATE_BEFORE_RELATIVE_TIME = 5
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ClaimPredicateType {
    ClaimPredicateUnconditional = 0,
    ClaimPredicateAnd = 1,
    ClaimPredicateOr = 2,
    ClaimPredicateNot = 3,
    ClaimPredicateBeforeAbsoluteTime = 4,
    ClaimPredicateBeforeRelativeTime = 5,
}

// ClaimPredicate is an XDR Union defines as:
//
//   union ClaimPredicate switch (ClaimPredicateType type)
//    {
//    case CLAIM_PREDICATE_UNCONDITIONAL:
//        void;
//    case CLAIM_PREDICATE_AND:
//        ClaimPredicate andPredicates<2>;
//    case CLAIM_PREDICATE_OR:
//        ClaimPredicate orPredicates<2>;
//    case CLAIM_PREDICATE_NOT:
//        ClaimPredicate* notPredicate;
//    case CLAIM_PREDICATE_BEFORE_ABSOLUTE_TIME:
//        int64 absBefore; // Predicate will be true if closeTime < absBefore
//    case CLAIM_PREDICATE_BEFORE_RELATIVE_TIME:
//        int64 relBefore; // Seconds since closeTime of the ledger in which the
//                         // ClaimableBalanceEntry was created
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ClaimPredicate {
    // IDEN CLAIM_PREDICATE_UNCONDITIONAL
    ClaimPredicateUnconditional(()),
    // IDEN CLAIM_PREDICATE_AND
    ClaimPredicateAnd(Vec<Box<ClaimPredicate>>),
    // IDEN CLAIM_PREDICATE_OR
    ClaimPredicateOr(Vec<Box<ClaimPredicate>>),
    // IDEN CLAIM_PREDICATE_NOT
    ClaimPredicateNot(Option<Box<ClaimPredicate>>),
    // IDEN CLAIM_PREDICATE_BEFORE_ABSOLUTE_TIME
    ClaimPredicateBeforeAbsoluteTime(Int64),
    // IDEN CLAIM_PREDICATE_BEFORE_RELATIVE_TIME
    ClaimPredicateBeforeRelativeTime(Int64),
}

// ClaimantType is an XDR Enum defines as:
//
//   enum ClaimantType
//    {
//        CLAIMANT_TYPE_V0 = 0
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ClaimantType {
    ClaimantTypeV0 = 0,
}

// ClaimantV0 is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID destination;    // The account that can use this condition
//            ClaimPredicate predicate; // Claimable if predicate is true
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ClaimantV0 {
    // TODO
    pub destination: AccountId,
    // TODO
    pub predicate: ClaimPredicate,
}

// Claimant is an XDR Union defines as:
//
//   union Claimant switch (ClaimantType type)
//    {
//    case CLAIMANT_TYPE_V0:
//        struct
//        {
//            AccountID destination;    // The account that can use this condition
//            ClaimPredicate predicate; // Claimable if predicate is true
//        } v0;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum Claimant {
    // IDEN CLAIMANT_TYPE_V0
    ClaimantTypeV0(ClaimantV0),
}

// ClaimableBalanceIdType is an XDR Enum defines as:
//
//   enum ClaimableBalanceIDType
//    {
//        CLAIMABLE_BALANCE_ID_TYPE_V0 = 0
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ClaimableBalanceIdType {
    ClaimableBalanceIdTypeV0 = 0,
}

// ClaimableBalanceId is an XDR Union defines as:
//
//   union ClaimableBalanceID switch (ClaimableBalanceIDType type)
//    {
//    case CLAIMABLE_BALANCE_ID_TYPE_V0:
//        Hash v0;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ClaimableBalanceId {
    // IDEN CLAIMABLE_BALANCE_ID_TYPE_V0
    ClaimableBalanceIdTypeV0(Hash),
}

// ClaimableBalanceFlags is an XDR Enum defines as:
//
//   enum ClaimableBalanceFlags
//    {
//        // If set, the issuer account of the asset held by the claimable balance may
//        // clawback the claimable balance
//        CLAIMABLE_BALANCE_CLAWBACK_ENABLED_FLAG = 0x1
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ClaimableBalanceFlags {
    ClaimableBalanceClawbackEnabledFlag = 1,
}

// MaskClaimableBalanceFlags is an XDR Const defines as:
//
//   const MASK_CLAIMABLE_BALANCE_FLAGS = 0x1;
//
const MASK_CLAIMABLE_BALANCE_FLAGS: u64 = 0x1;

// ClaimableBalanceEntryExtensionV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ClaimableBalanceEntryExtensionV1Ext {
    // NO IDEN 0
    V0(()),
}

// ClaimableBalanceEntryExtensionV1 is an XDR Struct defines as:
//
//   struct ClaimableBalanceEntryExtensionV1
//    {
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//
//        uint32 flags; // see ClaimableBalanceFlags
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ClaimableBalanceEntryExtensionV1 {
    // TODO
    pub ext: ClaimableBalanceEntryExtensionV1Ext,
    // TODO
    pub flags: Uint32,
}

// ClaimableBalanceEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            ClaimableBalanceEntryExtensionV1 v1;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ClaimableBalanceEntryExt {
    // NO IDEN 0
    V0(()),
    // NO IDEN 1
    V1(ClaimableBalanceEntryExtensionV1),
}

// ClaimableBalanceEntry is an XDR Struct defines as:
//
//   struct ClaimableBalanceEntry
//    {
//        // Unique identifier for this ClaimableBalanceEntry
//        ClaimableBalanceID balanceID;
//
//        // List of claimants with associated predicate
//        Claimant claimants<10>;
//
//        // Any asset including native
//        Asset asset;
//
//        // Amount of asset
//        int64 amount;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            ClaimableBalanceEntryExtensionV1 v1;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ClaimableBalanceEntry {
    // TODO
    pub balance_id: ClaimableBalanceId,
    // TODO
    pub claimants: Vec<Claimant>,
    // TODO
    pub asset: Asset,
    // TODO
    pub amount: Int64,
    // TODO
    pub ext: ClaimableBalanceEntryExt,
}

// LiquidityPoolConstantProductParameters is an XDR Struct defines as:
//
//   struct LiquidityPoolConstantProductParameters
//    {
//        Asset assetA; // assetA < assetB
//        Asset assetB;
//        int32 fee;    // Fee is in basis points, so the actual rate is (fee/100)%
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LiquidityPoolConstantProductParameters {
    // TODO
    pub asset_a: Asset,
    // TODO
    pub asset_b: Asset,
    // TODO
    pub fee: Int32,
}

// LiquidityPoolEntryConstantProduct is an XDR NestedStruct defines as:
//
//   struct
//            {
//                LiquidityPoolConstantProductParameters params;
//
//                int64 reserveA;        // amount of A in the pool
//                int64 reserveB;        // amount of B in the pool
//                int64 totalPoolShares; // total number of pool shares issued
//                int64 poolSharesTrustLineCount; // number of trust lines for the associated pool shares
//            }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LiquidityPoolEntryConstantProduct {
    // TODO
    pub params: LiquidityPoolConstantProductParameters,
    // TODO
    pub reserve_a: Int64,
    // TODO
    pub reserve_b: Int64,
    // TODO
    pub total_pool_shares: Int64,
    // TODO
    pub pool_shares_trust_line_count: Int64,
}

// LiquidityPoolEntryBody is an XDR NestedUnion defines as:
//
//   union switch (LiquidityPoolType type)
//        {
//        case LIQUIDITY_POOL_CONSTANT_PRODUCT:
//            struct
//            {
//                LiquidityPoolConstantProductParameters params;
//
//                int64 reserveA;        // amount of A in the pool
//                int64 reserveB;        // amount of B in the pool
//                int64 totalPoolShares; // total number of pool shares issued
//                int64 poolSharesTrustLineCount; // number of trust lines for the associated pool shares
//            } constantProduct;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LiquidityPoolEntryBody {
    // IDEN LIQUIDITY_POOL_CONSTANT_PRODUCT
    LiquidityPoolConstantProduct(LiquidityPoolEntryConstantProduct),
}

// LiquidityPoolEntry is an XDR Struct defines as:
//
//   struct LiquidityPoolEntry
//    {
//        PoolID liquidityPoolID;
//
//        union switch (LiquidityPoolType type)
//        {
//        case LIQUIDITY_POOL_CONSTANT_PRODUCT:
//            struct
//            {
//                LiquidityPoolConstantProductParameters params;
//
//                int64 reserveA;        // amount of A in the pool
//                int64 reserveB;        // amount of B in the pool
//                int64 totalPoolShares; // total number of pool shares issued
//                int64 poolSharesTrustLineCount; // number of trust lines for the associated pool shares
//            } constantProduct;
//        }
//        body;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LiquidityPoolEntry {
    // TODO
    pub liquidity_pool_id: PoolId,
    // TODO
    pub body: LiquidityPoolEntryBody,
}

// LedgerEntryExtensionV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerEntryExtensionV1Ext {
    // NO IDEN 0
    V0(()),
}

// LedgerEntryExtensionV1 is an XDR Struct defines as:
//
//   struct LedgerEntryExtensionV1
//    {
//        SponsorshipDescriptor sponsoringID;
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerEntryExtensionV1 {
    // TODO
    pub sponsoring_id: SponsorshipDescriptor,
    // TODO
    pub ext: LedgerEntryExtensionV1Ext,
}

// LedgerEntryData is an XDR NestedUnion defines as:
//
//   union switch (LedgerEntryType type)
//        {
//        case ACCOUNT:
//            AccountEntry account;
//        case TRUSTLINE:
//            TrustLineEntry trustLine;
//        case OFFER:
//            OfferEntry offer;
//        case DATA:
//            DataEntry data;
//        case CLAIMABLE_BALANCE:
//            ClaimableBalanceEntry claimableBalance;
//        case LIQUIDITY_POOL:
//            LiquidityPoolEntry liquidityPool;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerEntryData {
    // IDEN ACCOUNT
    Account(AccountEntry),
    // IDEN TRUSTLINE
    Trustline(TrustLineEntry),
    // IDEN OFFER
    Offer(OfferEntry),
    // IDEN DATA
    Data(DataEntry),
    // IDEN CLAIMABLE_BALANCE
    ClaimableBalance(ClaimableBalanceEntry),
    // IDEN LIQUIDITY_POOL
    LiquidityPool(LiquidityPoolEntry),
}

// LedgerEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            LedgerEntryExtensionV1 v1;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerEntryExt {
    // NO IDEN 0
    V0(()),
    // NO IDEN 1
    V1(LedgerEntryExtensionV1),
}

// LedgerEntry is an XDR Struct defines as:
//
//   struct LedgerEntry
//    {
//        uint32 lastModifiedLedgerSeq; // ledger the LedgerEntry was last changed
//
//        union switch (LedgerEntryType type)
//        {
//        case ACCOUNT:
//            AccountEntry account;
//        case TRUSTLINE:
//            TrustLineEntry trustLine;
//        case OFFER:
//            OfferEntry offer;
//        case DATA:
//            DataEntry data;
//        case CLAIMABLE_BALANCE:
//            ClaimableBalanceEntry claimableBalance;
//        case LIQUIDITY_POOL:
//            LiquidityPoolEntry liquidityPool;
//        }
//        data;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            LedgerEntryExtensionV1 v1;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerEntry {
    // TODO
    pub last_modified_ledger_seq: Uint32,
    // TODO
    pub data: LedgerEntryData,
    // TODO
    pub ext: LedgerEntryExt,
}

// LedgerKeyAccount is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID accountID;
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerKeyAccount {
    // TODO
    pub account_id: AccountId,
}

// LedgerKeyTrustLine is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID accountID;
//            TrustLineAsset asset;
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerKeyTrustLine {
    // TODO
    pub account_id: AccountId,
    // TODO
    pub asset: TrustLineAsset,
}

// LedgerKeyOffer is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID sellerID;
//            int64 offerID;
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerKeyOffer {
    // TODO
    pub seller_id: AccountId,
    // TODO
    pub offer_id: Int64,
}

// LedgerKeyData is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID accountID;
//            string64 dataName;
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerKeyData {
    // TODO
    pub account_id: AccountId,
    // TODO
    pub data_name: String64,
}

// LedgerKeyClaimableBalance is an XDR NestedStruct defines as:
//
//   struct
//        {
//            ClaimableBalanceID balanceID;
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerKeyClaimableBalance {
    // TODO
    pub balance_id: ClaimableBalanceId,
}

// LedgerKeyLiquidityPool is an XDR NestedStruct defines as:
//
//   struct
//        {
//            PoolID liquidityPoolID;
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerKeyLiquidityPool {
    // TODO
    pub liquidity_pool_id: PoolId,
}

// LedgerKey is an XDR Union defines as:
//
//   union LedgerKey switch (LedgerEntryType type)
//    {
//    case ACCOUNT:
//        struct
//        {
//            AccountID accountID;
//        } account;
//
//    case TRUSTLINE:
//        struct
//        {
//            AccountID accountID;
//            TrustLineAsset asset;
//        } trustLine;
//
//    case OFFER:
//        struct
//        {
//            AccountID sellerID;
//            int64 offerID;
//        } offer;
//
//    case DATA:
//        struct
//        {
//            AccountID accountID;
//            string64 dataName;
//        } data;
//
//    case CLAIMABLE_BALANCE:
//        struct
//        {
//            ClaimableBalanceID balanceID;
//        } claimableBalance;
//
//    case LIQUIDITY_POOL:
//        struct
//        {
//            PoolID liquidityPoolID;
//        } liquidityPool;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerKey {
    // IDEN ACCOUNT
    Account(LedgerKeyAccount),
    // IDEN TRUSTLINE
    Trustline(LedgerKeyTrustLine),
    // IDEN OFFER
    Offer(LedgerKeyOffer),
    // IDEN DATA
    Data(LedgerKeyData),
    // IDEN CLAIMABLE_BALANCE
    ClaimableBalance(LedgerKeyClaimableBalance),
    // IDEN LIQUIDITY_POOL
    LiquidityPool(LedgerKeyLiquidityPool),
}

// EnvelopeType is an XDR Enum defines as:
//
//   enum EnvelopeType
//    {
//        ENVELOPE_TYPE_TX_V0 = 0,
//        ENVELOPE_TYPE_SCP = 1,
//        ENVELOPE_TYPE_TX = 2,
//        ENVELOPE_TYPE_AUTH = 3,
//        ENVELOPE_TYPE_SCPVALUE = 4,
//        ENVELOPE_TYPE_TX_FEE_BUMP = 5,
//        ENVELOPE_TYPE_OP_ID = 6,
//        ENVELOPE_TYPE_POOL_REVOKE_OP_ID = 7
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum EnvelopeType {
    EnvelopeTypeTxV0 = 0,
    EnvelopeTypeScp = 1,
    EnvelopeTypeTx = 2,
    EnvelopeTypeAuth = 3,
    EnvelopeTypeScpvalue = 4,
    EnvelopeTypeTxFeeBump = 5,
    EnvelopeTypeOpId = 6,
    EnvelopeTypePoolRevokeOpId = 7,
}

// UpgradeType is an XDR Typedef defines as:
//
//   typedef opaque UpgradeType<128>;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct UpgradeType {
    #[array(var = 128)]
    pub value: Vec<u8>,
}

impl UpgradeType {
    pub fn new(value: Vec<u8>) -> UpgradeType {
        UpgradeType { value }
    }
}

// StellarValueType is an XDR Enum defines as:
//
//   enum StellarValueType
//    {
//        STELLAR_VALUE_BASIC = 0,
//        STELLAR_VALUE_SIGNED = 1
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum StellarValueType {
    StellarValueBasic = 0,
    StellarValueSigned = 1,
}

// LedgerCloseValueSignature is an XDR Struct defines as:
//
//   struct LedgerCloseValueSignature
//    {
//        NodeID nodeID;       // which node introduced the value
//        Signature signature; // nodeID's signature
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerCloseValueSignature {
    // TODO
    pub node_id: NodeId,
    // TODO
    pub signature: Signature,
}

// StellarValueExt is an XDR NestedUnion defines as:
//
//   union switch (StellarValueType v)
//        {
//        case STELLAR_VALUE_BASIC:
//            void;
//        case STELLAR_VALUE_SIGNED:
//            LedgerCloseValueSignature lcValueSignature;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum StellarValueExt {
    // IDEN STELLAR_VALUE_BASIC
    StellarValueBasic(()),
    // IDEN STELLAR_VALUE_SIGNED
    StellarValueSigned(LedgerCloseValueSignature),
}

// StellarValue is an XDR Struct defines as:
//
//   struct StellarValue
//    {
//        Hash txSetHash;      // transaction set to apply to previous ledger
//        TimePoint closeTime; // network close time
//
//        // upgrades to apply to the previous ledger (usually empty)
//        // this is a vector of encoded 'LedgerUpgrade' so that nodes can drop
//        // unknown steps during consensus if needed.
//        // see notes below on 'LedgerUpgrade' for more detail
//        // max size is dictated by number of upgrade types (+ room for future)
//        UpgradeType upgrades<6>;
//
//        // reserved for future use
//        union switch (StellarValueType v)
//        {
//        case STELLAR_VALUE_BASIC:
//            void;
//        case STELLAR_VALUE_SIGNED:
//            LedgerCloseValueSignature lcValueSignature;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct StellarValue {
    // TODO
    pub tx_set_hash: Hash,
    // TODO
    pub close_time: TimePoint,
    // TODO
    pub upgrades: Vec<UpgradeType>,
    // TODO
    pub ext: StellarValueExt,
}

// MaskLedgerHeaderFlags is an XDR Const defines as:
//
//   const MASK_LEDGER_HEADER_FLAGS = 0x7;
//
const MASK_LEDGER_HEADER_FLAGS: u64 = 0x7;

// LedgerHeaderFlags is an XDR Enum defines as:
//
//   enum LedgerHeaderFlags
//    {
//        DISABLE_LIQUIDITY_POOL_TRADING_FLAG = 0x1,
//        DISABLE_LIQUIDITY_POOL_DEPOSIT_FLAG = 0x2,
//        DISABLE_LIQUIDITY_POOL_WITHDRAWAL_FLAG = 0x4
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerHeaderFlags {
    DisableLiquidityPoolTradingFlag = 1,
    DisableLiquidityPoolDepositFlag = 2,
    DisableLiquidityPoolWithdrawalFlag = 4,
}

// LedgerHeaderExtensionV1Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerHeaderExtensionV1Ext {
    // NO IDEN 0
    V0(()),
}

// LedgerHeaderExtensionV1 is an XDR Struct defines as:
//
//   struct LedgerHeaderExtensionV1
//    {
//        uint32 flags; // LedgerHeaderFlags
//
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerHeaderExtensionV1 {
    // TODO
    pub flags: Uint32,
    // TODO
    pub ext: LedgerHeaderExtensionV1Ext,
}

// LedgerHeaderExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            LedgerHeaderExtensionV1 v1;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerHeaderExt {
    // NO IDEN 0
    V0(()),
    // NO IDEN 1
    V1(LedgerHeaderExtensionV1),
}

// LedgerHeader is an XDR Struct defines as:
//
//   struct LedgerHeader
//    {
//        uint32 ledgerVersion;    // the protocol version of the ledger
//        Hash previousLedgerHash; // hash of the previous ledger header
//        StellarValue scpValue;   // what consensus agreed to
//        Hash txSetResultHash;    // the TransactionResultSet that led to this ledger
//        Hash bucketListHash;     // hash of the ledger state
//
//        uint32 ledgerSeq; // sequence number of this ledger
//
//        int64 totalCoins; // total number of stroops in existence.
//                          // 10,000,000 stroops in 1 XLM
//
//        int64 feePool;       // fees burned since last inflation run
//        uint32 inflationSeq; // inflation sequence number
//
//        uint64 idPool; // last used global ID, used for generating objects
//
//        uint32 baseFee;     // base fee per operation in stroops
//        uint32 baseReserve; // account base reserve in stroops
//
//        uint32 maxTxSetSize; // maximum size a transaction set can be
//
//        Hash skipList[4]; // hashes of ledgers in the past. allows you to jump back
//                          // in time without walking the chain back ledger by ledger
//                          // each slot contains the oldest ledger that is mod of
//                          // either 50  5000  50000 or 500000 depending on index
//                          // skipList[0] mod(50), skipList[1] mod(5000), etc
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        case 1:
//            LedgerHeaderExtensionV1 v1;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerHeader {
    // TODO
    pub ledger_version: Uint32,
    // TODO
    pub previous_ledger_hash: Hash,
    // TODO
    pub scp_value: StellarValue,
    // TODO
    pub tx_set_result_hash: Hash,
    // TODO
    pub bucket_list_hash: Hash,
    // TODO
    pub ledger_seq: Uint32,
    // TODO
    pub total_coins: Int64,
    // TODO
    pub fee_pool: Int64,
    // TODO
    pub inflation_seq: Uint32,
    // TODO
    pub id_pool: Uint64,
    // TODO
    pub base_fee: Uint32,
    // TODO
    pub base_reserve: Uint32,
    // TODO
    pub max_tx_set_size: Uint32,
    // TODO
    pub skip_list: Vec<Hash>,
    // TODO
    pub ext: LedgerHeaderExt,
}

// LedgerUpgradeType is an XDR Enum defines as:
//
//   enum LedgerUpgradeType
//    {
//        LEDGER_UPGRADE_VERSION = 1,
//        LEDGER_UPGRADE_BASE_FEE = 2,
//        LEDGER_UPGRADE_MAX_TX_SET_SIZE = 3,
//        LEDGER_UPGRADE_BASE_RESERVE = 4,
//        LEDGER_UPGRADE_FLAGS = 5
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerUpgradeType {
    LedgerUpgradeVersion = 1,
    LedgerUpgradeBaseFee = 2,
    LedgerUpgradeMaxTxSetSize = 3,
    LedgerUpgradeBaseReserve = 4,
    LedgerUpgradeFlags = 5,
}

// LedgerUpgrade is an XDR Union defines as:
//
//   union LedgerUpgrade switch (LedgerUpgradeType type)
//    {
//    case LEDGER_UPGRADE_VERSION:
//        uint32 newLedgerVersion; // update ledgerVersion
//    case LEDGER_UPGRADE_BASE_FEE:
//        uint32 newBaseFee; // update baseFee
//    case LEDGER_UPGRADE_MAX_TX_SET_SIZE:
//        uint32 newMaxTxSetSize; // update maxTxSetSize
//    case LEDGER_UPGRADE_BASE_RESERVE:
//        uint32 newBaseReserve; // update baseReserve
//    case LEDGER_UPGRADE_FLAGS:
//        uint32 newFlags; // update flags
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerUpgrade {
    // IDEN LEDGER_UPGRADE_VERSION
    #[discriminant(value = "1")]
    LedgerUpgradeVersion(Uint32),
    // IDEN LEDGER_UPGRADE_BASE_FEE
    #[discriminant(value = "2")]
    LedgerUpgradeBaseFee(Uint32),
    // IDEN LEDGER_UPGRADE_MAX_TX_SET_SIZE
    #[discriminant(value = "3")]
    LedgerUpgradeMaxTxSetSize(Uint32),
    // IDEN LEDGER_UPGRADE_BASE_RESERVE
    #[discriminant(value = "4")]
    LedgerUpgradeBaseReserve(Uint32),
    // IDEN LEDGER_UPGRADE_FLAGS
    #[discriminant(value = "5")]
    LedgerUpgradeFlags(Uint32),
}

// BucketEntryType is an XDR Enum defines as:
//
//   enum BucketEntryType
//    {
//        METAENTRY =
//            -1, // At-and-after protocol 11: bucket metadata, should come first.
//        LIVEENTRY = 0, // Before protocol 11: created-or-updated;
//                       // At-and-after protocol 11: only updated.
//        DEADENTRY = 1,
//        INITENTRY = 2 // At-and-after protocol 11: only created.
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum BucketEntryType {
    Metaentry = -1,
    Liveentry = 0,
    Deadentry = 1,
    Initentry = 2,
}

// BucketMetadataExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum BucketMetadataExt {
    // NO IDEN 0
    V0(()),
}

// BucketMetadata is an XDR Struct defines as:
//
//   struct BucketMetadata
//    {
//        // Indicates the protocol version used to create / merge this bucket.
//        uint32 ledgerVersion;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct BucketMetadata {
    // TODO
    pub ledger_version: Uint32,
    // TODO
    pub ext: BucketMetadataExt,
}

// BucketEntry is an XDR Union defines as:
//
//   union BucketEntry switch (BucketEntryType type)
//    {
//    case LIVEENTRY:
//    case INITENTRY:
//        LedgerEntry liveEntry;
//
//    case DEADENTRY:
//        LedgerKey deadEntry;
//    case METAENTRY:
//        BucketMetadata metaEntry;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum BucketEntry {
    // IDEN LIVEENTRY
    #[discriminant(value = "0")]
    Liveentry(LedgerEntry),
    // IDEN INITENTRY
    #[discriminant(value = "1")]
    Initentry(LedgerEntry),
    // IDEN DEADENTRY
    #[discriminant(value = "2")]
    Deadentry(LedgerKey),
    // IDEN METAENTRY
    #[discriminant(value = "-1")]
    Metaentry(BucketMetadata),
}

// TransactionSet is an XDR Struct defines as:
//
//   struct TransactionSet
//    {
//        Hash previousLedgerHash;
//        TransactionEnvelope txs<>;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TransactionSet {
    // TODO
    pub previous_ledger_hash: Hash,
    // TODO
    pub txs: Vec<TransactionEnvelope>,
}

// TransactionResultPair is an XDR Struct defines as:
//
//   struct TransactionResultPair
//    {
//        Hash transactionHash;
//        TransactionResult result; // result for the transaction
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TransactionResultPair {
    // TODO
    pub transaction_hash: Hash,
    // TODO
    pub result: TransactionResult,
}

// TransactionResultSet is an XDR Struct defines as:
//
//   struct TransactionResultSet
//    {
//        TransactionResultPair results<>;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TransactionResultSet {
    // TODO
    pub results: Vec<TransactionResultPair>,
}

// TransactionHistoryEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum TransactionHistoryEntryExt {
    // NO IDEN 0
    V0(()),
}

// TransactionHistoryEntry is an XDR Struct defines as:
//
//   struct TransactionHistoryEntry
//    {
//        uint32 ledgerSeq;
//        TransactionSet txSet;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TransactionHistoryEntry {
    // TODO
    pub ledger_seq: Uint32,
    // TODO
    pub tx_set: TransactionSet,
    // TODO
    pub ext: TransactionHistoryEntryExt,
}

// TransactionHistoryResultEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum TransactionHistoryResultEntryExt {
    // NO IDEN 0
    V0(()),
}

// TransactionHistoryResultEntry is an XDR Struct defines as:
//
//   struct TransactionHistoryResultEntry
//    {
//        uint32 ledgerSeq;
//        TransactionResultSet txResultSet;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TransactionHistoryResultEntry {
    // TODO
    pub ledger_seq: Uint32,
    // TODO
    pub tx_result_set: TransactionResultSet,
    // TODO
    pub ext: TransactionHistoryResultEntryExt,
}

// LedgerHeaderHistoryEntryExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerHeaderHistoryEntryExt {
    // NO IDEN 0
    V0(()),
}

// LedgerHeaderHistoryEntry is an XDR Struct defines as:
//
//   struct LedgerHeaderHistoryEntry
//    {
//        Hash hash;
//        LedgerHeader header;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerHeaderHistoryEntry {
    // TODO
    pub hash: Hash,
    // TODO
    pub header: LedgerHeader,
    // TODO
    pub ext: LedgerHeaderHistoryEntryExt,
}

// LedgerScpMessages is an XDR Struct defines as:
//
//   struct LedgerSCPMessages
//    {
//        uint32 ledgerSeq;
//        SCPEnvelope messages<>;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerScpMessages {
    // TODO
    pub ledger_seq: Uint32,
    // TODO
    pub messages: Vec<ScpEnvelope>,
}

// ScpHistoryEntryV0 is an XDR Struct defines as:
//
//   struct SCPHistoryEntryV0
//    {
//        SCPQuorumSet quorumSets<>; // additional quorum sets used by ledgerMessages
//        LedgerSCPMessages ledgerMessages;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ScpHistoryEntryV0 {
    // TODO
    pub quorum_sets: Vec<ScpQuorumSet>,
    // TODO
    pub ledger_messages: LedgerScpMessages,
}

// ScpHistoryEntry is an XDR Union defines as:
//
//   union SCPHistoryEntry switch (int v)
//    {
//    case 0:
//        SCPHistoryEntryV0 v0;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ScpHistoryEntry {
    // NO IDEN 0
    V0(ScpHistoryEntryV0),
}

// LedgerEntryChangeType is an XDR Enum defines as:
//
//   enum LedgerEntryChangeType
//    {
//        LEDGER_ENTRY_CREATED = 0, // entry was added to the ledger
//        LEDGER_ENTRY_UPDATED = 1, // entry was modified in the ledger
//        LEDGER_ENTRY_REMOVED = 2, // entry was removed from the ledger
//        LEDGER_ENTRY_STATE = 3    // value of the entry
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerEntryChangeType {
    LedgerEntryCreated = 0,
    LedgerEntryUpdated = 1,
    LedgerEntryRemoved = 2,
    LedgerEntryState = 3,
}

// LedgerEntryChange is an XDR Union defines as:
//
//   union LedgerEntryChange switch (LedgerEntryChangeType type)
//    {
//    case LEDGER_ENTRY_CREATED:
//        LedgerEntry created;
//    case LEDGER_ENTRY_UPDATED:
//        LedgerEntry updated;
//    case LEDGER_ENTRY_REMOVED:
//        LedgerKey removed;
//    case LEDGER_ENTRY_STATE:
//        LedgerEntry state;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerEntryChange {
    // IDEN LEDGER_ENTRY_CREATED
    #[discriminant(value = "0")]
    LedgerEntryCreated(LedgerEntry),
    // IDEN LEDGER_ENTRY_UPDATED
    #[discriminant(value = "1")]
    LedgerEntryUpdated(LedgerEntry),
    // IDEN LEDGER_ENTRY_REMOVED
    #[discriminant(value = "2")]
    LedgerEntryRemoved(LedgerKey),
    // IDEN LEDGER_ENTRY_STATE
    #[discriminant(value = "3")]
    LedgerEntryState(LedgerEntry),
}

// LedgerEntryChanges is an XDR Typedef defines as:
//
//   typedef LedgerEntryChange LedgerEntryChanges<>;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerEntryChanges {
    // TODO
    pub value: Vec<LedgerEntryChange>,
}

impl LedgerEntryChanges {
    pub fn new(value: Vec<LedgerEntryChange>) -> LedgerEntryChanges {
        LedgerEntryChanges { value }
    }
}

// OperationMeta is an XDR Struct defines as:
//
//   struct OperationMeta
//    {
//        LedgerEntryChanges changes;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct OperationMeta {
    // TODO
    pub changes: LedgerEntryChanges,
}

// TransactionMetaV1 is an XDR Struct defines as:
//
//   struct TransactionMetaV1
//    {
//        LedgerEntryChanges txChanges; // tx level changes if any
//        OperationMeta operations<>;   // meta for each operation
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TransactionMetaV1 {
    // TODO
    pub tx_changes: LedgerEntryChanges,
    // TODO
    pub operations: Vec<OperationMeta>,
}

// TransactionMetaV2 is an XDR Struct defines as:
//
//   struct TransactionMetaV2
//    {
//        LedgerEntryChanges txChangesBefore; // tx level changes before operations
//                                            // are applied if any
//        OperationMeta operations<>;         // meta for each operation
//        LedgerEntryChanges txChangesAfter;  // tx level changes after operations are
//                                            // applied if any
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TransactionMetaV2 {
    // TODO
    pub tx_changes_before: LedgerEntryChanges,
    // TODO
    pub operations: Vec<OperationMeta>,
    // TODO
    pub tx_changes_after: LedgerEntryChanges,
}

// TransactionMeta is an XDR Union defines as:
//
//   union TransactionMeta switch (int v)
//    {
//    case 0:
//        OperationMeta operations<>;
//    case 1:
//        TransactionMetaV1 v1;
//    case 2:
//        TransactionMetaV2 v2;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum TransactionMeta {
    // NO IDEN 0
    #[discriminant(value = "0")]
    V0(Vec<OperationMeta>),
    // NO IDEN 1
    #[discriminant(value = "1")]
    V1(TransactionMetaV1),
    // NO IDEN 2
    #[discriminant(value = "2")]
    V2(TransactionMetaV2),
}

// TransactionResultMeta is an XDR Struct defines as:
//
//   struct TransactionResultMeta
//    {
//        TransactionResultPair result;
//        LedgerEntryChanges feeProcessing;
//        TransactionMeta txApplyProcessing;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TransactionResultMeta {
    // TODO
    pub result: TransactionResultPair,
    // TODO
    pub fee_processing: LedgerEntryChanges,
    // TODO
    pub tx_apply_processing: TransactionMeta,
}

// UpgradeEntryMeta is an XDR Struct defines as:
//
//   struct UpgradeEntryMeta
//    {
//        LedgerUpgrade upgrade;
//        LedgerEntryChanges changes;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct UpgradeEntryMeta {
    // TODO
    pub upgrade: LedgerUpgrade,
    // TODO
    pub changes: LedgerEntryChanges,
}

// LedgerCloseMetaV0 is an XDR Struct defines as:
//
//   struct LedgerCloseMetaV0
//    {
//        LedgerHeaderHistoryEntry ledgerHeader;
//        // NB: txSet is sorted in "Hash order"
//        TransactionSet txSet;
//
//        // NB: transactions are sorted in apply order here
//        // fees for all transactions are processed first
//        // followed by applying transactions
//        TransactionResultMeta txProcessing<>;
//
//        // upgrades are applied last
//        UpgradeEntryMeta upgradesProcessing<>;
//
//        // other misc information attached to the ledger close
//        SCPHistoryEntry scpInfo<>;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LedgerCloseMetaV0 {
    // TODO
    pub ledger_header: LedgerHeaderHistoryEntry,
    // TODO
    pub tx_set: TransactionSet,
    // TODO
    pub tx_processing: Vec<TransactionResultMeta>,
    // TODO
    pub upgrades_processing: Vec<UpgradeEntryMeta>,
    // TODO
    pub scp_info: Vec<ScpHistoryEntry>,
}

// LedgerCloseMeta is an XDR Union defines as:
//
//   union LedgerCloseMeta switch (int v)
//    {
//    case 0:
//        LedgerCloseMetaV0 v0;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LedgerCloseMeta {
    // NO IDEN 0
    V0(LedgerCloseMetaV0),
}

// ErrorCode is an XDR Enum defines as:
//
//   enum ErrorCode
//    {
//        ERR_MISC = 0, // Unspecific error
//        ERR_DATA = 1, // Malformed data
//        ERR_CONF = 2, // Misconfiguration error
//        ERR_AUTH = 3, // Authentication failure
//        ERR_LOAD = 4  // System overloaded
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ErrorCode {
    ErrMisc = 0,
    ErrData = 1,
    ErrConf = 2,
    ErrAuth = 3,
    ErrLoad = 4,
}

// SError is an XDR Struct defines as:
//
//   struct Error
//    {
//        ErrorCode code;
//        string msg<100>;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct SError {
    // TODO
    pub code: ErrorCode,
    #[array(var = 100)]
    pub msg: String,
}

// AuthCert is an XDR Struct defines as:
//
//   struct AuthCert
//    {
//        Curve25519Public pubkey;
//        uint64 expiration;
//        Signature sig;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct AuthCert {
    // TODO
    pub pubkey: Curve25519Public,
    // TODO
    pub expiration: Uint64,
    // TODO
    pub sig: Signature,
}

// Hello is an XDR Struct defines as:
//
//   struct Hello
//    {
//        uint32 ledgerVersion;
//        uint32 overlayVersion;
//        uint32 overlayMinVersion;
//        Hash networkID;
//        string versionStr<100>;
//        int listeningPort;
//        NodeID peerID;
//        AuthCert cert;
//        uint256 nonce;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Hello {
    // TODO
    pub ledger_version: Uint32,
    // TODO
    pub overlay_version: Uint32,
    // TODO
    pub overlay_min_version: Uint32,
    // TODO
    pub network_id: Hash,
    #[array(var = 100)]
    pub version_str: String,
    pub listening_port: i32,
    // TODO
    pub peer_id: NodeId,
    // TODO
    pub cert: AuthCert,
    // TODO
    pub nonce: Uint256,
}

// Auth is an XDR Struct defines as:
//
//   struct Auth
//    {
//        // Empty message, just to confirm
//        // establishment of MAC keys.
//        int unused;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Auth {
    pub unused: i32,
}

// IpAddrType is an XDR Enum defines as:
//
//   enum IPAddrType
//    {
//        IPv4 = 0,
//        IPv6 = 1
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum IpAddrType {
    IPv4 = 0,
    IPv6 = 1,
}

// PeerAddressIp is an XDR NestedUnion defines as:
//
//   union switch (IPAddrType type)
//        {
//        case IPv4:
//            opaque ipv4[4];
//        case IPv6:
//            opaque ipv6[16];
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum PeerAddressIp {
    // IDEN IPv4
    #[discriminant(value = "0")]
    IPv4(Vec<u8>),
    // IDEN IPv6
    #[discriminant(value = "1")]
    IPv6(Vec<u8>),
}

// PeerAddress is an XDR Struct defines as:
//
//   struct PeerAddress
//    {
//        union switch (IPAddrType type)
//        {
//        case IPv4:
//            opaque ipv4[4];
//        case IPv6:
//            opaque ipv6[16];
//        }
//        ip;
//        uint32 port;
//        uint32 numFailures;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct PeerAddress {
    // TODO
    pub ip: PeerAddressIp,
    // TODO
    pub port: Uint32,
    // TODO
    pub num_failures: Uint32,
}

// MessageType is an XDR Enum defines as:
//
//   enum MessageType
//    {
//        ERROR_MSG = 0,
//        AUTH = 2,
//        DONT_HAVE = 3,
//
//        GET_PEERS = 4, // gets a list of peers this guy knows about
//        PEERS = 5,
//
//        GET_TX_SET = 6, // gets a particular txset by hash
//        TX_SET = 7,
//
//        TRANSACTION = 8, // pass on a tx you have heard about
//
//        // SCP
//        GET_SCP_QUORUMSET = 9,
//        SCP_QUORUMSET = 10,
//        SCP_MESSAGE = 11,
//        GET_SCP_STATE = 12,
//
//        // new messages
//        HELLO = 13,
//
//        SURVEY_REQUEST = 14,
//        SURVEY_RESPONSE = 15
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum MessageType {
    ErrorMsg = 0,
    Auth = 2,
    DontHave = 3,
    GetPeers = 4,
    Peers = 5,
    GetTxSet = 6,
    TxSet = 7,
    Transaction = 8,
    GetScpQuorumset = 9,
    ScpQuorumset = 10,
    ScpMessage = 11,
    GetScpState = 12,
    Hello = 13,
    SurveyRequest = 14,
    SurveyResponse = 15,
}

// DontHave is an XDR Struct defines as:
//
//   struct DontHave
//    {
//        MessageType type;
//        uint256 reqHash;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct DontHave {
    // TODO
    pub type_: MessageType,
    // TODO
    pub req_hash: Uint256,
}

// SurveyMessageCommandType is an XDR Enum defines as:
//
//   enum SurveyMessageCommandType
//    {
//        SURVEY_TOPOLOGY = 0
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum SurveyMessageCommandType {
    SurveyTopology = 0,
}

// SurveyRequestMessage is an XDR Struct defines as:
//
//   struct SurveyRequestMessage
//    {
//        NodeID surveyorPeerID;
//        NodeID surveyedPeerID;
//        uint32 ledgerNum;
//        Curve25519Public encryptionKey;
//        SurveyMessageCommandType commandType;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct SurveyRequestMessage {
    // TODO
    pub surveyor_peer_id: NodeId,
    // TODO
    pub surveyed_peer_id: NodeId,
    // TODO
    pub ledger_num: Uint32,
    // TODO
    pub encryption_key: Curve25519Public,
    // TODO
    pub command_type: SurveyMessageCommandType,
}

// SignedSurveyRequestMessage is an XDR Struct defines as:
//
//   struct SignedSurveyRequestMessage
//    {
//        Signature requestSignature;
//        SurveyRequestMessage request;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct SignedSurveyRequestMessage {
    // TODO
    pub request_signature: Signature,
    // TODO
    pub request: SurveyRequestMessage,
}

// EncryptedBody is an XDR Typedef defines as:
//
//   typedef opaque EncryptedBody<64000>;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct EncryptedBody {
    #[array(var = 64000)]
    pub value: Vec<u8>,
}

impl EncryptedBody {
    pub fn new(value: Vec<u8>) -> EncryptedBody {
        EncryptedBody { value }
    }
}

// SurveyResponseMessage is an XDR Struct defines as:
//
//   struct SurveyResponseMessage
//    {
//        NodeID surveyorPeerID;
//        NodeID surveyedPeerID;
//        uint32 ledgerNum;
//        SurveyMessageCommandType commandType;
//        EncryptedBody encryptedBody;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct SurveyResponseMessage {
    // TODO
    pub surveyor_peer_id: NodeId,
    // TODO
    pub surveyed_peer_id: NodeId,
    // TODO
    pub ledger_num: Uint32,
    // TODO
    pub command_type: SurveyMessageCommandType,
    // TODO
    pub encrypted_body: EncryptedBody,
}

// SignedSurveyResponseMessage is an XDR Struct defines as:
//
//   struct SignedSurveyResponseMessage
//    {
//        Signature responseSignature;
//        SurveyResponseMessage response;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct SignedSurveyResponseMessage {
    // TODO
    pub response_signature: Signature,
    // TODO
    pub response: SurveyResponseMessage,
}

// PeerStats is an XDR Struct defines as:
//
//   struct PeerStats
//    {
//        NodeID id;
//        string versionStr<100>;
//        uint64 messagesRead;
//        uint64 messagesWritten;
//        uint64 bytesRead;
//        uint64 bytesWritten;
//        uint64 secondsConnected;
//
//        uint64 uniqueFloodBytesRecv;
//        uint64 duplicateFloodBytesRecv;
//        uint64 uniqueFetchBytesRecv;
//        uint64 duplicateFetchBytesRecv;
//
//        uint64 uniqueFloodMessageRecv;
//        uint64 duplicateFloodMessageRecv;
//        uint64 uniqueFetchMessageRecv;
//        uint64 duplicateFetchMessageRecv;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct PeerStats {
    // TODO
    pub id: NodeId,
    #[array(var = 100)]
    pub version_str: String,
    // TODO
    pub messages_read: Uint64,
    // TODO
    pub messages_written: Uint64,
    // TODO
    pub bytes_read: Uint64,
    // TODO
    pub bytes_written: Uint64,
    // TODO
    pub seconds_connected: Uint64,
    // TODO
    pub unique_flood_bytes_recv: Uint64,
    // TODO
    pub duplicate_flood_bytes_recv: Uint64,
    // TODO
    pub unique_fetch_bytes_recv: Uint64,
    // TODO
    pub duplicate_fetch_bytes_recv: Uint64,
    // TODO
    pub unique_flood_message_recv: Uint64,
    // TODO
    pub duplicate_flood_message_recv: Uint64,
    // TODO
    pub unique_fetch_message_recv: Uint64,
    // TODO
    pub duplicate_fetch_message_recv: Uint64,
}

// PeerStatList is an XDR Typedef defines as:
//
//   typedef PeerStats PeerStatList<25>;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct PeerStatList {
    // TODO
    pub value: Vec<PeerStats>,
}

impl PeerStatList {
    pub fn new(value: Vec<PeerStats>) -> PeerStatList {
        PeerStatList { value }
    }
}

// TopologyResponseBody is an XDR Struct defines as:
//
//   struct TopologyResponseBody
//    {
//        PeerStatList inboundPeers;
//        PeerStatList outboundPeers;
//
//        uint32 totalInboundPeerCount;
//        uint32 totalOutboundPeerCount;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TopologyResponseBody {
    // TODO
    pub inbound_peers: PeerStatList,
    // TODO
    pub outbound_peers: PeerStatList,
    // TODO
    pub total_inbound_peer_count: Uint32,
    // TODO
    pub total_outbound_peer_count: Uint32,
}

// SurveyResponseBody is an XDR Union defines as:
//
//   union SurveyResponseBody switch (SurveyMessageCommandType type)
//    {
//    case SURVEY_TOPOLOGY:
//        TopologyResponseBody topologyResponseBody;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum SurveyResponseBody {
    // IDEN SURVEY_TOPOLOGY
    SurveyTopology(TopologyResponseBody),
}

// StellarMessage is an XDR Union defines as:
//
//   union StellarMessage switch (MessageType type)
//    {
//    case ERROR_MSG:
//        Error error;
//    case HELLO:
//        Hello hello;
//    case AUTH:
//        Auth auth;
//    case DONT_HAVE:
//        DontHave dontHave;
//    case GET_PEERS:
//        void;
//    case PEERS:
//        PeerAddress peers<100>;
//
//    case GET_TX_SET:
//        uint256 txSetHash;
//    case TX_SET:
//        TransactionSet txSet;
//
//    case TRANSACTION:
//        TransactionEnvelope transaction;
//
//    case SURVEY_REQUEST:
//        SignedSurveyRequestMessage signedSurveyRequestMessage;
//
//    case SURVEY_RESPONSE:
//        SignedSurveyResponseMessage signedSurveyResponseMessage;
//
//    // SCP
//    case GET_SCP_QUORUMSET:
//        uint256 qSetHash;
//    case SCP_QUORUMSET:
//        SCPQuorumSet qSet;
//    case SCP_MESSAGE:
//        SCPEnvelope envelope;
//    case GET_SCP_STATE:
//        uint32 getSCPLedgerSeq; // ledger seq requested ; if 0, requests the latest
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum StellarMessage {
    // IDEN ERROR_MSG
    #[discriminant(value = "0")]
    ErrorMsg(SError),
    // IDEN HELLO
    #[discriminant(value = "13")]
    Hello(Hello),
    // IDEN AUTH
    #[discriminant(value = "2")]
    Auth(Auth),
    // IDEN DONT_HAVE
    #[discriminant(value = "3")]
    DontHave(DontHave),
    // IDEN GET_PEERS
    #[discriminant(value = "4")]
    GetPeers(()),
    // IDEN PEERS
    #[discriminant(value = "5")]
    Peers(Vec<PeerAddress>),
    // IDEN GET_TX_SET
    #[discriminant(value = "6")]
    GetTxSet(Uint256),
    // IDEN TX_SET
    #[discriminant(value = "7")]
    TxSet(TransactionSet),
    // IDEN TRANSACTION
    #[discriminant(value = "8")]
    Transaction(TransactionEnvelope),
    // IDEN SURVEY_REQUEST
    #[discriminant(value = "14")]
    SurveyRequest(SignedSurveyRequestMessage),
    // IDEN SURVEY_RESPONSE
    #[discriminant(value = "15")]
    SurveyResponse(SignedSurveyResponseMessage),
    // IDEN GET_SCP_QUORUMSET
    #[discriminant(value = "9")]
    GetScpQuorumset(Uint256),
    // IDEN SCP_QUORUMSET
    #[discriminant(value = "10")]
    ScpQuorumset(ScpQuorumSet),
    // IDEN SCP_MESSAGE
    #[discriminant(value = "11")]
    ScpMessage(ScpEnvelope),
    // IDEN GET_SCP_STATE
    #[discriminant(value = "12")]
    GetScpState(Uint32),
}

// AuthenticatedMessageV0 is an XDR NestedStruct defines as:
//
//   struct
//        {
//            uint64 sequence;
//            StellarMessage message;
//            HmacSha256Mac mac;
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct AuthenticatedMessageV0 {
    // TODO
    pub sequence: Uint64,
    // TODO
    pub message: StellarMessage,
    // TODO
    pub mac: HmacSha256Mac,
}

// AuthenticatedMessage is an XDR Union defines as:
//
//   union AuthenticatedMessage switch (uint32 v)
//    {
//    case 0:
//        struct
//        {
//            uint64 sequence;
//            StellarMessage message;
//            HmacSha256Mac mac;
//        } v0;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum AuthenticatedMessage {
    // NO IDEN 0
    V0(AuthenticatedMessageV0),
}

// Value is an XDR Typedef defines as:
//
//   typedef opaque Value<>;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Value {
    #[array(var = )]
    pub value: Vec<u8>,
}

impl Value {
    pub fn new(value: Vec<u8>) -> Value {
        Value { value }
    }
}

// ScpBallot is an XDR Struct defines as:
//
//   struct SCPBallot
//    {
//        uint32 counter; // n
//        Value value;    // x
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ScpBallot {
    // TODO
    pub counter: Uint32,
    // TODO
    pub value: Value,
}

// ScpStatementType is an XDR Enum defines as:
//
//   enum SCPStatementType
//    {
//        SCP_ST_PREPARE = 0,
//        SCP_ST_CONFIRM = 1,
//        SCP_ST_EXTERNALIZE = 2,
//        SCP_ST_NOMINATE = 3
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ScpStatementType {
    ScpStPrepare = 0,
    ScpStConfirm = 1,
    ScpStExternalize = 2,
    ScpStNominate = 3,
}

// ScpNomination is an XDR Struct defines as:
//
//   struct SCPNomination
//    {
//        Hash quorumSetHash; // D
//        Value votes<>;      // X
//        Value accepted<>;   // Y
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ScpNomination {
    // TODO
    pub quorum_set_hash: Hash,
    // TODO
    pub votes: Vec<Value>,
    // TODO
    pub accepted: Vec<Value>,
}

// ScpStatementPrepare is an XDR NestedStruct defines as:
//
//   struct
//            {
//                Hash quorumSetHash;       // D
//                SCPBallot ballot;         // b
//                SCPBallot* prepared;      // p
//                SCPBallot* preparedPrime; // p'
//                uint32 nC;                // c.n
//                uint32 nH;                // h.n
//            }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ScpStatementPrepare {
    // TODO
    pub quorum_set_hash: Hash,
    // TODO
    pub ballot: ScpBallot,
    // TODO
    pub prepared: Option<ScpBallot>,
    // TODO
    pub prepared_prime: Option<ScpBallot>,
    // TODO
    pub n_c: Uint32,
    // TODO
    pub n_h: Uint32,
}

// ScpStatementConfirm is an XDR NestedStruct defines as:
//
//   struct
//            {
//                SCPBallot ballot;   // b
//                uint32 nPrepared;   // p.n
//                uint32 nCommit;     // c.n
//                uint32 nH;          // h.n
//                Hash quorumSetHash; // D
//            }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ScpStatementConfirm {
    // TODO
    pub ballot: ScpBallot,
    // TODO
    pub n_prepared: Uint32,
    // TODO
    pub n_commit: Uint32,
    // TODO
    pub n_h: Uint32,
    // TODO
    pub quorum_set_hash: Hash,
}

// ScpStatementExternalize is an XDR NestedStruct defines as:
//
//   struct
//            {
//                SCPBallot commit;         // c
//                uint32 nH;                // h.n
//                Hash commitQuorumSetHash; // D used before EXTERNALIZE
//            }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ScpStatementExternalize {
    // TODO
    pub commit: ScpBallot,
    // TODO
    pub n_h: Uint32,
    // TODO
    pub commit_quorum_set_hash: Hash,
}

// ScpStatementPledges is an XDR NestedUnion defines as:
//
//   union switch (SCPStatementType type)
//        {
//        case SCP_ST_PREPARE:
//            struct
//            {
//                Hash quorumSetHash;       // D
//                SCPBallot ballot;         // b
//                SCPBallot* prepared;      // p
//                SCPBallot* preparedPrime; // p'
//                uint32 nC;                // c.n
//                uint32 nH;                // h.n
//            } prepare;
//        case SCP_ST_CONFIRM:
//            struct
//            {
//                SCPBallot ballot;   // b
//                uint32 nPrepared;   // p.n
//                uint32 nCommit;     // c.n
//                uint32 nH;          // h.n
//                Hash quorumSetHash; // D
//            } confirm;
//        case SCP_ST_EXTERNALIZE:
//            struct
//            {
//                SCPBallot commit;         // c
//                uint32 nH;                // h.n
//                Hash commitQuorumSetHash; // D used before EXTERNALIZE
//            } externalize;
//        case SCP_ST_NOMINATE:
//            SCPNomination nominate;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ScpStatementPledges {
    // IDEN SCP_ST_PREPARE
    ScpStPrepare(ScpStatementPrepare),
    // IDEN SCP_ST_CONFIRM
    ScpStConfirm(ScpStatementConfirm),
    // IDEN SCP_ST_EXTERNALIZE
    ScpStExternalize(ScpStatementExternalize),
    // IDEN SCP_ST_NOMINATE
    ScpStNominate(ScpNomination),
}

// ScpStatement is an XDR Struct defines as:
//
//   struct SCPStatement
//    {
//        NodeID nodeID;    // v
//        uint64 slotIndex; // i
//
//        union switch (SCPStatementType type)
//        {
//        case SCP_ST_PREPARE:
//            struct
//            {
//                Hash quorumSetHash;       // D
//                SCPBallot ballot;         // b
//                SCPBallot* prepared;      // p
//                SCPBallot* preparedPrime; // p'
//                uint32 nC;                // c.n
//                uint32 nH;                // h.n
//            } prepare;
//        case SCP_ST_CONFIRM:
//            struct
//            {
//                SCPBallot ballot;   // b
//                uint32 nPrepared;   // p.n
//                uint32 nCommit;     // c.n
//                uint32 nH;          // h.n
//                Hash quorumSetHash; // D
//            } confirm;
//        case SCP_ST_EXTERNALIZE:
//            struct
//            {
//                SCPBallot commit;         // c
//                uint32 nH;                // h.n
//                Hash commitQuorumSetHash; // D used before EXTERNALIZE
//            } externalize;
//        case SCP_ST_NOMINATE:
//            SCPNomination nominate;
//        }
//        pledges;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ScpStatement {
    // TODO
    pub node_id: NodeId,
    // TODO
    pub slot_index: Uint64,
    // TODO
    pub pledges: ScpStatementPledges,
}

// ScpEnvelope is an XDR Struct defines as:
//
//   struct SCPEnvelope
//    {
//        SCPStatement statement;
//        Signature signature;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ScpEnvelope {
    // TODO
    pub statement: ScpStatement,
    // TODO
    pub signature: Signature,
}

// ScpQuorumSet is an XDR Struct defines as:
//
//   struct SCPQuorumSet
//    {
//        uint32 threshold;
//        NodeID validators<>;
//        SCPQuorumSet innerSets<>;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ScpQuorumSet {
    // TODO
    pub threshold: Uint32,
    // TODO
    pub validators: Vec<NodeId>,
    // TODO
    pub inner_sets: Vec<ScpQuorumSet>,
}

// LiquidityPoolParameters is an XDR Union defines as:
//
//   union LiquidityPoolParameters switch (LiquidityPoolType type)
//    {
//    case LIQUIDITY_POOL_CONSTANT_PRODUCT:
//        LiquidityPoolConstantProductParameters constantProduct;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LiquidityPoolParameters {
    // IDEN LIQUIDITY_POOL_CONSTANT_PRODUCT
    LiquidityPoolConstantProduct(LiquidityPoolConstantProductParameters),
}

// MuxedAccountMed25519 is an XDR NestedStruct defines as:
//
//   struct
//        {
//            uint64 id;
//            uint256 ed25519;
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct MuxedAccountMed25519 {
    // TODO
    pub id: Uint64,
    // TODO
    pub ed25519: Uint256,
}

// MuxedAccount is an XDR Union defines as:
//
//   union MuxedAccount switch (CryptoKeyType type)
//    {
//    case KEY_TYPE_ED25519:
//        uint256 ed25519;
//    case KEY_TYPE_MUXED_ED25519:
//        struct
//        {
//            uint64 id;
//            uint256 ed25519;
//        } med25519;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum MuxedAccount {
    // IDEN KEY_TYPE_ED25519
    #[discriminant(value = "0")]
    KeyTypeEd25519(Uint256),
    // IDEN KEY_TYPE_MUXED_ED25519
    #[discriminant(value = "256")]
    KeyTypeMuxedEd25519(MuxedAccountMed25519),
}

// DecoratedSignature is an XDR Struct defines as:
//
//   struct DecoratedSignature
//    {
//        SignatureHint hint;  // last 4 bytes of the public key, used as a hint
//        Signature signature; // actual signature
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct DecoratedSignature {
    // TODO
    pub hint: SignatureHint,
    // TODO
    pub signature: Signature,
}

// OperationType is an XDR Enum defines as:
//
//   enum OperationType
//    {
//        CREATE_ACCOUNT = 0,
//        PAYMENT = 1,
//        PATH_PAYMENT_STRICT_RECEIVE = 2,
//        MANAGE_SELL_OFFER = 3,
//        CREATE_PASSIVE_SELL_OFFER = 4,
//        SET_OPTIONS = 5,
//        CHANGE_TRUST = 6,
//        ALLOW_TRUST = 7,
//        ACCOUNT_MERGE = 8,
//        INFLATION = 9,
//        MANAGE_DATA = 10,
//        BUMP_SEQUENCE = 11,
//        MANAGE_BUY_OFFER = 12,
//        PATH_PAYMENT_STRICT_SEND = 13,
//        CREATE_CLAIMABLE_BALANCE = 14,
//        CLAIM_CLAIMABLE_BALANCE = 15,
//        BEGIN_SPONSORING_FUTURE_RESERVES = 16,
//        END_SPONSORING_FUTURE_RESERVES = 17,
//        REVOKE_SPONSORSHIP = 18,
//        CLAWBACK = 19,
//        CLAWBACK_CLAIMABLE_BALANCE = 20,
//        SET_TRUST_LINE_FLAGS = 21,
//        LIQUIDITY_POOL_DEPOSIT = 22,
//        LIQUIDITY_POOL_WITHDRAW = 23
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum OperationType {
    CreateAccount = 0,
    Payment = 1,
    PathPaymentStrictReceive = 2,
    ManageSellOffer = 3,
    CreatePassiveSellOffer = 4,
    SetOptions = 5,
    ChangeTrust = 6,
    AllowTrust = 7,
    AccountMerge = 8,
    Inflation = 9,
    ManageData = 10,
    BumpSequence = 11,
    ManageBuyOffer = 12,
    PathPaymentStrictSend = 13,
    CreateClaimableBalance = 14,
    ClaimClaimableBalance = 15,
    BeginSponsoringFutureReserves = 16,
    EndSponsoringFutureReserves = 17,
    RevokeSponsorship = 18,
    Clawback = 19,
    ClawbackClaimableBalance = 20,
    SetTrustLineFlags = 21,
    LiquidityPoolDeposit = 22,
    LiquidityPoolWithdraw = 23,
}

// CreateAccountOp is an XDR Struct defines as:
//
//   struct CreateAccountOp
//    {
//        AccountID destination; // account to create
//        int64 startingBalance; // amount they end up with
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct CreateAccountOp {
    // TODO
    pub destination: AccountId,
    // TODO
    pub starting_balance: Int64,
}

// PaymentOp is an XDR Struct defines as:
//
//   struct PaymentOp
//    {
//        MuxedAccount destination; // recipient of the payment
//        Asset asset;              // what they end up with
//        int64 amount;             // amount they end up with
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct PaymentOp {
    // TODO
    pub destination: MuxedAccount,
    // TODO
    pub asset: Asset,
    // TODO
    pub amount: Int64,
}

// PathPaymentStrictReceiveOp is an XDR Struct defines as:
//
//   struct PathPaymentStrictReceiveOp
//    {
//        Asset sendAsset; // asset we pay with
//        int64 sendMax;   // the maximum amount of sendAsset to
//                         // send (excluding fees).
//                         // The operation will fail if can't be met
//
//        MuxedAccount destination; // recipient of the payment
//        Asset destAsset;          // what they end up with
//        int64 destAmount;         // amount they end up with
//
//        Asset path<5>; // additional hops it must go through to get there
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct PathPaymentStrictReceiveOp {
    // TODO
    pub send_asset: Asset,
    // TODO
    pub send_max: Int64,
    // TODO
    pub destination: MuxedAccount,
    // TODO
    pub dest_asset: Asset,
    // TODO
    pub dest_amount: Int64,
    // TODO
    pub path: Vec<Asset>,
}

// PathPaymentStrictSendOp is an XDR Struct defines as:
//
//   struct PathPaymentStrictSendOp
//    {
//        Asset sendAsset;  // asset we pay with
//        int64 sendAmount; // amount of sendAsset to send (excluding fees)
//
//        MuxedAccount destination; // recipient of the payment
//        Asset destAsset;          // what they end up with
//        int64 destMin;            // the minimum amount of dest asset to
//                                  // be received
//                                  // The operation will fail if it can't be met
//
//        Asset path<5>; // additional hops it must go through to get there
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct PathPaymentStrictSendOp {
    // TODO
    pub send_asset: Asset,
    // TODO
    pub send_amount: Int64,
    // TODO
    pub destination: MuxedAccount,
    // TODO
    pub dest_asset: Asset,
    // TODO
    pub dest_min: Int64,
    // TODO
    pub path: Vec<Asset>,
}

// ManageSellOfferOp is an XDR Struct defines as:
//
//   struct ManageSellOfferOp
//    {
//        Asset selling;
//        Asset buying;
//        int64 amount; // amount being sold. if set to 0, delete the offer
//        Price price;  // price of thing being sold in terms of what you are buying
//
//        // 0=create a new offer, otherwise edit an existing offer
//        int64 offerID;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ManageSellOfferOp {
    // TODO
    pub selling: Asset,
    // TODO
    pub buying: Asset,
    // TODO
    pub amount: Int64,
    // TODO
    pub price: Price,
    // TODO
    pub offer_id: Int64,
}

// ManageBuyOfferOp is an XDR Struct defines as:
//
//   struct ManageBuyOfferOp
//    {
//        Asset selling;
//        Asset buying;
//        int64 buyAmount; // amount being bought. if set to 0, delete the offer
//        Price price;     // price of thing being bought in terms of what you are
//                         // selling
//
//        // 0=create a new offer, otherwise edit an existing offer
//        int64 offerID;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ManageBuyOfferOp {
    // TODO
    pub selling: Asset,
    // TODO
    pub buying: Asset,
    // TODO
    pub buy_amount: Int64,
    // TODO
    pub price: Price,
    // TODO
    pub offer_id: Int64,
}

// CreatePassiveSellOfferOp is an XDR Struct defines as:
//
//   struct CreatePassiveSellOfferOp
//    {
//        Asset selling; // A
//        Asset buying;  // B
//        int64 amount;  // amount taker gets
//        Price price;   // cost of A in terms of B
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct CreatePassiveSellOfferOp {
    // TODO
    pub selling: Asset,
    // TODO
    pub buying: Asset,
    // TODO
    pub amount: Int64,
    // TODO
    pub price: Price,
}

// SetOptionsOp is an XDR Struct defines as:
//
//   struct SetOptionsOp
//    {
//        AccountID* inflationDest; // sets the inflation destination
//
//        uint32* clearFlags; // which flags to clear
//        uint32* setFlags;   // which flags to set
//
//        // account threshold manipulation
//        uint32* masterWeight; // weight of the master account
//        uint32* lowThreshold;
//        uint32* medThreshold;
//        uint32* highThreshold;
//
//        string32* homeDomain; // sets the home domain
//
//        // Add, update or remove a signer for the account
//        // signer is deleted if the weight is 0
//        Signer* signer;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct SetOptionsOp {
    // TODO
    pub inflation_dest: Option<AccountId>,
    // TODO
    pub clear_flags: Option<Uint32>,
    // TODO
    pub set_flags: Option<Uint32>,
    // TODO
    pub master_weight: Option<Uint32>,
    // TODO
    pub low_threshold: Option<Uint32>,
    // TODO
    pub med_threshold: Option<Uint32>,
    // TODO
    pub high_threshold: Option<Uint32>,
    // TODO
    pub home_domain: Option<String32>,
    // TODO
    pub signer: Option<Signer>,
}

// ChangeTrustAsset is an XDR Union defines as:
//
//   union ChangeTrustAsset switch (AssetType type)
//    {
//    case ASSET_TYPE_NATIVE: // Not credit
//        void;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM4:
//        AlphaNum4 alphaNum4;
//
//    case ASSET_TYPE_CREDIT_ALPHANUM12:
//        AlphaNum12 alphaNum12;
//
//    case ASSET_TYPE_POOL_SHARE:
//        LiquidityPoolParameters liquidityPool;
//
//        // add other asset types here in the future
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ChangeTrustAsset {
    // IDEN ASSET_TYPE_NATIVE
    AssetTypeNative(()),
    // IDEN ASSET_TYPE_CREDIT_ALPHANUM4
    AssetTypeCreditAlphanum4(AlphaNum4),
    // IDEN ASSET_TYPE_CREDIT_ALPHANUM12
    AssetTypeCreditAlphanum12(AlphaNum12),
    // IDEN ASSET_TYPE_POOL_SHARE
    AssetTypePoolShare(LiquidityPoolParameters),
}

// ChangeTrustOp is an XDR Struct defines as:
//
//   struct ChangeTrustOp
//    {
//        ChangeTrustAsset line;
//
//        // if limit is set to 0, deletes the trust line
//        int64 limit;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ChangeTrustOp {
    // TODO
    pub line: ChangeTrustAsset,
    // TODO
    pub limit: Int64,
}

// AllowTrustOp is an XDR Struct defines as:
//
//   struct AllowTrustOp
//    {
//        AccountID trustor;
//        AssetCode asset;
//
//        // One of 0, AUTHORIZED_FLAG, or AUTHORIZED_TO_MAINTAIN_LIABILITIES_FLAG
//        uint32 authorize;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct AllowTrustOp {
    // TODO
    pub trustor: AccountId,
    // TODO
    pub asset: AssetCode,
    // TODO
    pub authorize: Uint32,
}

// ManageDataOp is an XDR Struct defines as:
//
//   struct ManageDataOp
//    {
//        string64 dataName;
//        DataValue* dataValue; // set to null to clear
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ManageDataOp {
    // TODO
    pub data_name: String64,
    // TODO
    pub data_value: Option<DataValue>,
}

// BumpSequenceOp is an XDR Struct defines as:
//
//   struct BumpSequenceOp
//    {
//        SequenceNumber bumpTo;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct BumpSequenceOp {
    // TODO
    pub bump_to: SequenceNumber,
}

// CreateClaimableBalanceOp is an XDR Struct defines as:
//
//   struct CreateClaimableBalanceOp
//    {
//        Asset asset;
//        int64 amount;
//        Claimant claimants<10>;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct CreateClaimableBalanceOp {
    // TODO
    pub asset: Asset,
    // TODO
    pub amount: Int64,
    // TODO
    pub claimants: Vec<Claimant>,
}

// ClaimClaimableBalanceOp is an XDR Struct defines as:
//
//   struct ClaimClaimableBalanceOp
//    {
//        ClaimableBalanceID balanceID;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ClaimClaimableBalanceOp {
    // TODO
    pub balance_id: ClaimableBalanceId,
}

// BeginSponsoringFutureReservesOp is an XDR Struct defines as:
//
//   struct BeginSponsoringFutureReservesOp
//    {
//        AccountID sponsoredID;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct BeginSponsoringFutureReservesOp {
    // TODO
    pub sponsored_id: AccountId,
}

// RevokeSponsorshipType is an XDR Enum defines as:
//
//   enum RevokeSponsorshipType
//    {
//        REVOKE_SPONSORSHIP_LEDGER_ENTRY = 0,
//        REVOKE_SPONSORSHIP_SIGNER = 1
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum RevokeSponsorshipType {
    RevokeSponsorshipLedgerEntry = 0,
    RevokeSponsorshipSigner = 1,
}

// RevokeSponsorshipOpSigner is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID accountID;
//            SignerKey signerKey;
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct RevokeSponsorshipOpSigner {
    // TODO
    pub account_id: AccountId,
    // TODO
    pub signer_key: SignerKey,
}

// RevokeSponsorshipOp is an XDR Union defines as:
//
//   union RevokeSponsorshipOp switch (RevokeSponsorshipType type)
//    {
//    case REVOKE_SPONSORSHIP_LEDGER_ENTRY:
//        LedgerKey ledgerKey;
//    case REVOKE_SPONSORSHIP_SIGNER:
//        struct
//        {
//            AccountID accountID;
//            SignerKey signerKey;
//        } signer;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum RevokeSponsorshipOp {
    // IDEN REVOKE_SPONSORSHIP_LEDGER_ENTRY
    RevokeSponsorshipLedgerEntry(LedgerKey),
    // IDEN REVOKE_SPONSORSHIP_SIGNER
    RevokeSponsorshipSigner(RevokeSponsorshipOpSigner),
}

// ClawbackOp is an XDR Struct defines as:
//
//   struct ClawbackOp
//    {
//        Asset asset;
//        MuxedAccount from;
//        int64 amount;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ClawbackOp {
    // TODO
    pub asset: Asset,
    // TODO
    pub from: MuxedAccount,
    // TODO
    pub amount: Int64,
}

// ClawbackClaimableBalanceOp is an XDR Struct defines as:
//
//   struct ClawbackClaimableBalanceOp
//    {
//        ClaimableBalanceID balanceID;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ClawbackClaimableBalanceOp {
    // TODO
    pub balance_id: ClaimableBalanceId,
}

// SetTrustLineFlagsOp is an XDR Struct defines as:
//
//   struct SetTrustLineFlagsOp
//    {
//        AccountID trustor;
//        Asset asset;
//
//        uint32 clearFlags; // which flags to clear
//        uint32 setFlags;   // which flags to set
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct SetTrustLineFlagsOp {
    // TODO
    pub trustor: AccountId,
    // TODO
    pub asset: Asset,
    // TODO
    pub clear_flags: Uint32,
    // TODO
    pub set_flags: Uint32,
}

// LiquidityPoolFeeV18 is an XDR Const defines as:
//
//   const LIQUIDITY_POOL_FEE_V18 = 30;
//
const LIQUIDITY_POOL_FEE_V18: u64 = 30;

// LiquidityPoolDepositOp is an XDR Struct defines as:
//
//   struct LiquidityPoolDepositOp
//    {
//        PoolID liquidityPoolID;
//        int64 maxAmountA;     // maximum amount of first asset to deposit
//        int64 maxAmountB;     // maximum amount of second asset to deposit
//        Price minPrice;       // minimum depositA/depositB
//        Price maxPrice;       // maximum depositA/depositB
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LiquidityPoolDepositOp {
    // TODO
    pub liquidity_pool_id: PoolId,
    // TODO
    pub max_amount_a: Int64,
    // TODO
    pub max_amount_b: Int64,
    // TODO
    pub min_price: Price,
    // TODO
    pub max_price: Price,
}

// LiquidityPoolWithdrawOp is an XDR Struct defines as:
//
//   struct LiquidityPoolWithdrawOp
//    {
//        PoolID liquidityPoolID;
//        int64 amount;         // amount of pool shares to withdraw
//        int64 minAmountA;     // minimum amount of first asset to withdraw
//        int64 minAmountB;     // minimum amount of second asset to withdraw
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct LiquidityPoolWithdrawOp {
    // TODO
    pub liquidity_pool_id: PoolId,
    // TODO
    pub amount: Int64,
    // TODO
    pub min_amount_a: Int64,
    // TODO
    pub min_amount_b: Int64,
}

// OperationBody is an XDR NestedUnion defines as:
//
//   union switch (OperationType type)
//        {
//        case CREATE_ACCOUNT:
//            CreateAccountOp createAccountOp;
//        case PAYMENT:
//            PaymentOp paymentOp;
//        case PATH_PAYMENT_STRICT_RECEIVE:
//            PathPaymentStrictReceiveOp pathPaymentStrictReceiveOp;
//        case MANAGE_SELL_OFFER:
//            ManageSellOfferOp manageSellOfferOp;
//        case CREATE_PASSIVE_SELL_OFFER:
//            CreatePassiveSellOfferOp createPassiveSellOfferOp;
//        case SET_OPTIONS:
//            SetOptionsOp setOptionsOp;
//        case CHANGE_TRUST:
//            ChangeTrustOp changeTrustOp;
//        case ALLOW_TRUST:
//            AllowTrustOp allowTrustOp;
//        case ACCOUNT_MERGE:
//            MuxedAccount destination;
//        case INFLATION:
//            void;
//        case MANAGE_DATA:
//            ManageDataOp manageDataOp;
//        case BUMP_SEQUENCE:
//            BumpSequenceOp bumpSequenceOp;
//        case MANAGE_BUY_OFFER:
//            ManageBuyOfferOp manageBuyOfferOp;
//        case PATH_PAYMENT_STRICT_SEND:
//            PathPaymentStrictSendOp pathPaymentStrictSendOp;
//        case CREATE_CLAIMABLE_BALANCE:
//            CreateClaimableBalanceOp createClaimableBalanceOp;
//        case CLAIM_CLAIMABLE_BALANCE:
//            ClaimClaimableBalanceOp claimClaimableBalanceOp;
//        case BEGIN_SPONSORING_FUTURE_RESERVES:
//            BeginSponsoringFutureReservesOp beginSponsoringFutureReservesOp;
//        case END_SPONSORING_FUTURE_RESERVES:
//            void;
//        case REVOKE_SPONSORSHIP:
//            RevokeSponsorshipOp revokeSponsorshipOp;
//        case CLAWBACK:
//            ClawbackOp clawbackOp;
//        case CLAWBACK_CLAIMABLE_BALANCE:
//            ClawbackClaimableBalanceOp clawbackClaimableBalanceOp;
//        case SET_TRUST_LINE_FLAGS:
//            SetTrustLineFlagsOp setTrustLineFlagsOp;
//        case LIQUIDITY_POOL_DEPOSIT:
//            LiquidityPoolDepositOp liquidityPoolDepositOp;
//        case LIQUIDITY_POOL_WITHDRAW:
//            LiquidityPoolWithdrawOp liquidityPoolWithdrawOp;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum OperationBody {
    // IDEN CREATE_ACCOUNT
    #[discriminant(value = "0")]
    CreateAccount(CreateAccountOp),
    // IDEN PAYMENT
    #[discriminant(value = "1")]
    Payment(PaymentOp),
    // IDEN PATH_PAYMENT_STRICT_RECEIVE
    #[discriminant(value = "2")]
    PathPaymentStrictReceive(PathPaymentStrictReceiveOp),
    // IDEN MANAGE_SELL_OFFER
    #[discriminant(value = "3")]
    ManageSellOffer(ManageSellOfferOp),
    // IDEN CREATE_PASSIVE_SELL_OFFER
    #[discriminant(value = "4")]
    CreatePassiveSellOffer(CreatePassiveSellOfferOp),
    // IDEN SET_OPTIONS
    #[discriminant(value = "5")]
    SetOptions(SetOptionsOp),
    // IDEN CHANGE_TRUST
    #[discriminant(value = "6")]
    ChangeTrust(ChangeTrustOp),
    // IDEN ALLOW_TRUST
    #[discriminant(value = "7")]
    AllowTrust(AllowTrustOp),
    // IDEN ACCOUNT_MERGE
    #[discriminant(value = "8")]
    AccountMerge(MuxedAccount),
    // IDEN INFLATION
    #[discriminant(value = "9")]
    Inflation(()),
    // IDEN MANAGE_DATA
    #[discriminant(value = "10")]
    ManageData(ManageDataOp),
    // IDEN BUMP_SEQUENCE
    #[discriminant(value = "11")]
    BumpSequence(BumpSequenceOp),
    // IDEN MANAGE_BUY_OFFER
    #[discriminant(value = "12")]
    ManageBuyOffer(ManageBuyOfferOp),
    // IDEN PATH_PAYMENT_STRICT_SEND
    #[discriminant(value = "13")]
    PathPaymentStrictSend(PathPaymentStrictSendOp),
    // IDEN CREATE_CLAIMABLE_BALANCE
    #[discriminant(value = "14")]
    CreateClaimableBalance(CreateClaimableBalanceOp),
    // IDEN CLAIM_CLAIMABLE_BALANCE
    #[discriminant(value = "15")]
    ClaimClaimableBalance(ClaimClaimableBalanceOp),
    // IDEN BEGIN_SPONSORING_FUTURE_RESERVES
    #[discriminant(value = "16")]
    BeginSponsoringFutureReserves(BeginSponsoringFutureReservesOp),
    // IDEN END_SPONSORING_FUTURE_RESERVES
    #[discriminant(value = "17")]
    EndSponsoringFutureReserves(()),
    // IDEN REVOKE_SPONSORSHIP
    #[discriminant(value = "18")]
    RevokeSponsorship(RevokeSponsorshipOp),
    // IDEN CLAWBACK
    #[discriminant(value = "19")]
    Clawback(ClawbackOp),
    // IDEN CLAWBACK_CLAIMABLE_BALANCE
    #[discriminant(value = "20")]
    ClawbackClaimableBalance(ClawbackClaimableBalanceOp),
    // IDEN SET_TRUST_LINE_FLAGS
    #[discriminant(value = "21")]
    SetTrustLineFlags(SetTrustLineFlagsOp),
    // IDEN LIQUIDITY_POOL_DEPOSIT
    #[discriminant(value = "22")]
    LiquidityPoolDeposit(LiquidityPoolDepositOp),
    // IDEN LIQUIDITY_POOL_WITHDRAW
    #[discriminant(value = "23")]
    LiquidityPoolWithdraw(LiquidityPoolWithdrawOp),
}

// Operation is an XDR Struct defines as:
//
//   struct Operation
//    {
//        // sourceAccount is the account used to run the operation
//        // if not set, the runtime defaults to "sourceAccount" specified at
//        // the transaction level
//        MuxedAccount* sourceAccount;
//
//        union switch (OperationType type)
//        {
//        case CREATE_ACCOUNT:
//            CreateAccountOp createAccountOp;
//        case PAYMENT:
//            PaymentOp paymentOp;
//        case PATH_PAYMENT_STRICT_RECEIVE:
//            PathPaymentStrictReceiveOp pathPaymentStrictReceiveOp;
//        case MANAGE_SELL_OFFER:
//            ManageSellOfferOp manageSellOfferOp;
//        case CREATE_PASSIVE_SELL_OFFER:
//            CreatePassiveSellOfferOp createPassiveSellOfferOp;
//        case SET_OPTIONS:
//            SetOptionsOp setOptionsOp;
//        case CHANGE_TRUST:
//            ChangeTrustOp changeTrustOp;
//        case ALLOW_TRUST:
//            AllowTrustOp allowTrustOp;
//        case ACCOUNT_MERGE:
//            MuxedAccount destination;
//        case INFLATION:
//            void;
//        case MANAGE_DATA:
//            ManageDataOp manageDataOp;
//        case BUMP_SEQUENCE:
//            BumpSequenceOp bumpSequenceOp;
//        case MANAGE_BUY_OFFER:
//            ManageBuyOfferOp manageBuyOfferOp;
//        case PATH_PAYMENT_STRICT_SEND:
//            PathPaymentStrictSendOp pathPaymentStrictSendOp;
//        case CREATE_CLAIMABLE_BALANCE:
//            CreateClaimableBalanceOp createClaimableBalanceOp;
//        case CLAIM_CLAIMABLE_BALANCE:
//            ClaimClaimableBalanceOp claimClaimableBalanceOp;
//        case BEGIN_SPONSORING_FUTURE_RESERVES:
//            BeginSponsoringFutureReservesOp beginSponsoringFutureReservesOp;
//        case END_SPONSORING_FUTURE_RESERVES:
//            void;
//        case REVOKE_SPONSORSHIP:
//            RevokeSponsorshipOp revokeSponsorshipOp;
//        case CLAWBACK:
//            ClawbackOp clawbackOp;
//        case CLAWBACK_CLAIMABLE_BALANCE:
//            ClawbackClaimableBalanceOp clawbackClaimableBalanceOp;
//        case SET_TRUST_LINE_FLAGS:
//            SetTrustLineFlagsOp setTrustLineFlagsOp;
//        case LIQUIDITY_POOL_DEPOSIT:
//            LiquidityPoolDepositOp liquidityPoolDepositOp;
//        case LIQUIDITY_POOL_WITHDRAW:
//            LiquidityPoolWithdrawOp liquidityPoolWithdrawOp;
//        }
//        body;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Operation {
    // TODO
    pub source_account: Option<MuxedAccount>,
    // TODO
    pub body: OperationBody,
}

// HashIdPreimageOperationId is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID sourceAccount;
//            SequenceNumber seqNum;
//            uint32 opNum;
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct HashIdPreimageOperationId {
    // TODO
    pub source_account: AccountId,
    // TODO
    pub seq_num: SequenceNumber,
    // TODO
    pub op_num: Uint32,
}

// HashIdPreimageRevokeId is an XDR NestedStruct defines as:
//
//   struct
//        {
//            AccountID sourceAccount;
//            SequenceNumber seqNum;
//            uint32 opNum;
//            PoolID liquidityPoolID;
//            Asset asset;
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct HashIdPreimageRevokeId {
    // TODO
    pub source_account: AccountId,
    // TODO
    pub seq_num: SequenceNumber,
    // TODO
    pub op_num: Uint32,
    // TODO
    pub liquidity_pool_id: PoolId,
    // TODO
    pub asset: Asset,
}

// HashIdPreimage is an XDR Union defines as:
//
//   union HashIDPreimage switch (EnvelopeType type)
//    {
//    case ENVELOPE_TYPE_OP_ID:
//        struct
//        {
//            AccountID sourceAccount;
//            SequenceNumber seqNum;
//            uint32 opNum;
//        } operationID;
//    case ENVELOPE_TYPE_POOL_REVOKE_OP_ID:
//        struct
//        {
//            AccountID sourceAccount;
//            SequenceNumber seqNum;
//            uint32 opNum;
//            PoolID liquidityPoolID;
//            Asset asset;
//        } revokeID;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum HashIdPreimage {
    // IDEN ENVELOPE_TYPE_OP_ID
    #[discriminant(value = "6")]
    EnvelopeTypeOpId(HashIdPreimageOperationId),
    // IDEN ENVELOPE_TYPE_POOL_REVOKE_OP_ID
    #[discriminant(value = "7")]
    EnvelopeTypePoolRevokeOpId(HashIdPreimageRevokeId),
}

// MemoType is an XDR Enum defines as:
//
//   enum MemoType
//    {
//        MEMO_NONE = 0,
//        MEMO_TEXT = 1,
//        MEMO_ID = 2,
//        MEMO_HASH = 3,
//        MEMO_RETURN = 4
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum MemoType {
    MemoNone = 0,
    MemoText = 1,
    MemoId = 2,
    MemoHash = 3,
    MemoReturn = 4,
}

// Memo is an XDR Union defines as:
//
//   union Memo switch (MemoType type)
//    {
//    case MEMO_NONE:
//        void;
//    case MEMO_TEXT:
//        string text<28>;
//    case MEMO_ID:
//        uint64 id;
//    case MEMO_HASH:
//        Hash hash; // the hash of what to pull from the content server
//    case MEMO_RETURN:
//        Hash retHash; // the hash of the tx you are rejecting
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum Memo {
    // IDEN MEMO_NONE
    MemoNone(()),
    // IDEN MEMO_TEXT
    MemoText(String),
    // IDEN MEMO_ID
    MemoId(Uint64),
    // IDEN MEMO_HASH
    MemoHash(Hash),
    // IDEN MEMO_RETURN
    MemoReturn(Hash),
}

// TimeBounds is an XDR Struct defines as:
//
//   struct TimeBounds
//    {
//        TimePoint minTime;
//        TimePoint maxTime; // 0 here means no maxTime
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TimeBounds {
    // TODO
    pub min_time: TimePoint,
    // TODO
    pub max_time: TimePoint,
}

// MaxOpsPerTx is an XDR Const defines as:
//
//   const MAX_OPS_PER_TX = 100;
//
pub const MAX_OPS_PER_TX: u64 = 100;

// TransactionV0Ext is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum TransactionV0Ext {
    // NO IDEN 0
    V0(()),
}

// TransactionV0 is an XDR Struct defines as:
//
//   struct TransactionV0
//    {
//        uint256 sourceAccountEd25519;
//        uint32 fee;
//        SequenceNumber seqNum;
//        TimeBounds* timeBounds;
//        Memo memo;
//        Operation operations<MAX_OPS_PER_TX>;
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TransactionV0 {
    // TODO
    pub source_account_ed25519: Uint256,
    // TODO
    pub fee: Uint32,
    // TODO
    pub seq_num: SequenceNumber,
    // TODO
    pub time_bounds: Option<TimeBounds>,
    // TODO
    pub memo: Memo,
    // TODO
    pub operations: Vec<Operation>,
    // TODO
    pub ext: TransactionV0Ext,
}

// TransactionV0Envelope is an XDR Struct defines as:
//
//   struct TransactionV0Envelope
//    {
//        TransactionV0 tx;
//        /* Each decorated signature is a signature over the SHA256 hash of
//         * a TransactionSignaturePayload */
//        DecoratedSignature signatures<20>;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TransactionV0Envelope {
    // TODO
    pub tx: TransactionV0,
    // TODO
    pub signatures: Vec<DecoratedSignature>,
}

// TransactionExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum TransactionExt {
    // NO IDEN 0
    V0(()),
}

// Transaction is an XDR Struct defines as:
//
//   struct Transaction
//    {
//        // account used to run the transaction
//        MuxedAccount sourceAccount;
//
//        // the fee the sourceAccount will pay
//        uint32 fee;
//
//        // sequence number to consume in the account
//        SequenceNumber seqNum;
//
//        // validity range (inclusive) for the last ledger close time
//        TimeBounds* timeBounds;
//
//        Memo memo;
//
//        Operation operations<MAX_OPS_PER_TX>;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Transaction {
    // TODO
    pub source_account: MuxedAccount,
    // TODO
    pub fee: Uint32,
    // TODO
    pub seq_num: SequenceNumber,
    // TODO
    pub time_bounds: Option<TimeBounds>,
    // TODO
    pub memo: Memo,
    // TODO
    pub operations: Vec<Operation>,
    // TODO
    pub ext: TransactionExt,
}

// TransactionV1Envelope is an XDR Struct defines as:
//
//   struct TransactionV1Envelope
//    {
//        Transaction tx;
//        /* Each decorated signature is a signature over the SHA256 hash of
//         * a TransactionSignaturePayload */
//        DecoratedSignature signatures<20>;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TransactionV1Envelope {
    // TODO
    pub tx: Transaction,
    // TODO
    pub signatures: Vec<DecoratedSignature>,
}

// FeeBumpTransactionInnerTx is an XDR NestedUnion defines as:
//
//   union switch (EnvelopeType type)
//        {
//        case ENVELOPE_TYPE_TX:
//            TransactionV1Envelope v1;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum FeeBumpTransactionInnerTx {
    // IDEN ENVELOPE_TYPE_TX
    #[discriminant(value = "2")]
    EnvelopeTypeTx(TransactionV1Envelope),
}

// FeeBumpTransactionExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum FeeBumpTransactionExt {
    // NO IDEN 0
    V0(()),
}

// FeeBumpTransaction is an XDR Struct defines as:
//
//   struct FeeBumpTransaction
//    {
//        MuxedAccount feeSource;
//        int64 fee;
//        union switch (EnvelopeType type)
//        {
//        case ENVELOPE_TYPE_TX:
//            TransactionV1Envelope v1;
//        }
//        innerTx;
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct FeeBumpTransaction {
    // TODO
    pub fee_source: MuxedAccount,
    // TODO
    pub fee: Int64,
    // TODO
    pub inner_tx: FeeBumpTransactionInnerTx,
    // TODO
    pub ext: FeeBumpTransactionExt,
}

// FeeBumpTransactionEnvelope is an XDR Struct defines as:
//
//   struct FeeBumpTransactionEnvelope
//    {
//        FeeBumpTransaction tx;
//        /* Each decorated signature is a signature over the SHA256 hash of
//         * a TransactionSignaturePayload */
//        DecoratedSignature signatures<20>;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct FeeBumpTransactionEnvelope {
    // TODO
    pub tx: FeeBumpTransaction,
    // TODO
    pub signatures: Vec<DecoratedSignature>,
}

// TransactionEnvelope is an XDR Union defines as:
//
//   union TransactionEnvelope switch (EnvelopeType type)
//    {
//    case ENVELOPE_TYPE_TX_V0:
//        TransactionV0Envelope v0;
//    case ENVELOPE_TYPE_TX:
//        TransactionV1Envelope v1;
//    case ENVELOPE_TYPE_TX_FEE_BUMP:
//        FeeBumpTransactionEnvelope feeBump;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum TransactionEnvelope {
    // IDEN ENVELOPE_TYPE_TX_V0
    #[discriminant(value = "0")]
    EnvelopeTypeTxV0(TransactionV0Envelope),
    // IDEN ENVELOPE_TYPE_TX
    #[discriminant(value = "2")]
    EnvelopeTypeTx(TransactionV1Envelope),
    // IDEN ENVELOPE_TYPE_TX_FEE_BUMP
    #[discriminant(value = "5")]
    EnvelopeTypeTxFeeBump(FeeBumpTransactionEnvelope),
}

// TransactionSignaturePayloadTaggedTransaction is an XDR NestedUnion defines as:
//
//   union switch (EnvelopeType type)
//        {
//        // Backwards Compatibility: Use ENVELOPE_TYPE_TX to sign ENVELOPE_TYPE_TX_V0
//        case ENVELOPE_TYPE_TX:
//            Transaction tx;
//        case ENVELOPE_TYPE_TX_FEE_BUMP:
//            FeeBumpTransaction feeBump;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum TransactionSignaturePayloadTaggedTransaction {
    // IDEN ENVELOPE_TYPE_TX
    #[discriminant(value = "2")]
    EnvelopeTypeTx(Transaction),
    // IDEN ENVELOPE_TYPE_TX_FEE_BUMP
    #[discriminant(value = "5")]
    EnvelopeTypeTxFeeBump(FeeBumpTransaction),
}

// TransactionSignaturePayload is an XDR Struct defines as:
//
//   struct TransactionSignaturePayload
//    {
//        Hash networkId;
//        union switch (EnvelopeType type)
//        {
//        // Backwards Compatibility: Use ENVELOPE_TYPE_TX to sign ENVELOPE_TYPE_TX_V0
//        case ENVELOPE_TYPE_TX:
//            Transaction tx;
//        case ENVELOPE_TYPE_TX_FEE_BUMP:
//            FeeBumpTransaction feeBump;
//        }
//        taggedTransaction;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TransactionSignaturePayload {
    // TODO
    pub network_id: Hash,
    // TODO
    pub tagged_transaction: TransactionSignaturePayloadTaggedTransaction,
}

// ClaimAtomType is an XDR Enum defines as:
//
//   enum ClaimAtomType
//    {
//        CLAIM_ATOM_TYPE_V0 = 0,
//        CLAIM_ATOM_TYPE_ORDER_BOOK = 1,
//        CLAIM_ATOM_TYPE_LIQUIDITY_POOL = 2
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ClaimAtomType {
    ClaimAtomTypeV0 = 0,
    ClaimAtomTypeOrderBook = 1,
    ClaimAtomTypeLiquidityPool = 2,
}

// ClaimOfferAtomV0 is an XDR Struct defines as:
//
//   struct ClaimOfferAtomV0
//    {
//        // emitted to identify the offer
//        uint256 sellerEd25519; // Account that owns the offer
//        int64 offerID;
//
//        // amount and asset taken from the owner
//        Asset assetSold;
//        int64 amountSold;
//
//        // amount and asset sent to the owner
//        Asset assetBought;
//        int64 amountBought;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ClaimOfferAtomV0 {
    // TODO
    pub seller_ed25519: Uint256,
    // TODO
    pub offer_id: Int64,
    // TODO
    pub asset_sold: Asset,
    // TODO
    pub amount_sold: Int64,
    // TODO
    pub asset_bought: Asset,
    // TODO
    pub amount_bought: Int64,
}

// ClaimOfferAtom is an XDR Struct defines as:
//
//   struct ClaimOfferAtom
//    {
//        // emitted to identify the offer
//        AccountID sellerID; // Account that owns the offer
//        int64 offerID;
//
//        // amount and asset taken from the owner
//        Asset assetSold;
//        int64 amountSold;
//
//        // amount and asset sent to the owner
//        Asset assetBought;
//        int64 amountBought;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ClaimOfferAtom {
    // TODO
    pub seller_id: AccountId,
    // TODO
    pub offer_id: Int64,
    // TODO
    pub asset_sold: Asset,
    // TODO
    pub amount_sold: Int64,
    // TODO
    pub asset_bought: Asset,
    // TODO
    pub amount_bought: Int64,
}

// ClaimLiquidityAtom is an XDR Struct defines as:
//
//   struct ClaimLiquidityAtom
//    {
//        PoolID liquidityPoolID;
//
//        // amount and asset taken from the pool
//        Asset assetSold;
//        int64 amountSold;
//
//        // amount and asset sent to the pool
//        Asset assetBought;
//        int64 amountBought;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ClaimLiquidityAtom {
    // TODO
    pub liquidity_pool_id: PoolId,
    // TODO
    pub asset_sold: Asset,
    // TODO
    pub amount_sold: Int64,
    // TODO
    pub asset_bought: Asset,
    // TODO
    pub amount_bought: Int64,
}

// ClaimAtom is an XDR Union defines as:
//
//   union ClaimAtom switch (ClaimAtomType type)
//    {
//    case CLAIM_ATOM_TYPE_V0:
//        ClaimOfferAtomV0 v0;
//    case CLAIM_ATOM_TYPE_ORDER_BOOK:
//        ClaimOfferAtom orderBook;
//    case CLAIM_ATOM_TYPE_LIQUIDITY_POOL:
//        ClaimLiquidityAtom liquidityPool;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ClaimAtom {
    // IDEN CLAIM_ATOM_TYPE_V0
    ClaimAtomTypeV0(ClaimOfferAtomV0),
    // IDEN CLAIM_ATOM_TYPE_ORDER_BOOK
    ClaimAtomTypeOrderBook(ClaimOfferAtom),
    // IDEN CLAIM_ATOM_TYPE_LIQUIDITY_POOL
    ClaimAtomTypeLiquidityPool(ClaimLiquidityAtom),
}

// CreateAccountResultCode is an XDR Enum defines as:
//
//   enum CreateAccountResultCode
//    {
//        // codes considered as "success" for the operation
//        CREATE_ACCOUNT_SUCCESS = 0, // account was created
//
//        // codes considered as "failure" for the operation
//        CREATE_ACCOUNT_MALFORMED = -1,   // invalid destination
//        CREATE_ACCOUNT_UNDERFUNDED = -2, // not enough funds in source account
//        CREATE_ACCOUNT_LOW_RESERVE =
//            -3, // would create an account below the min reserve
//        CREATE_ACCOUNT_ALREADY_EXIST = -4 // account already exists
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum CreateAccountResultCode {
    CreateAccountSuccess = 0,
    CreateAccountMalformed = -1,
    CreateAccountUnderfunded = -2,
    CreateAccountLowReserve = -3,
    CreateAccountAlreadyExist = -4,
}

// CreateAccountResult is an XDR Union defines as:
//
//   union CreateAccountResult switch (CreateAccountResultCode code)
//    {
//    case CREATE_ACCOUNT_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum CreateAccountResult {
    // IDEN CREATE_ACCOUNT_SUCCESS
    #[discriminant(value = "0")]
    CreateAccountSuccess(()),
    #[discriminant(value = "-1")]
    CreateAccountMalformed(()),
    #[discriminant(value = "-2")]
    CreateAccountUnderfunded(()),
    #[discriminant(value = "-3")]
    CreateAccountLowReserve(()),
    #[discriminant(value = "-4")]
    CreateAccountAlreadyExist(()),
}

// PaymentResultCode is an XDR Enum defines as:
//
//   enum PaymentResultCode
//    {
//        // codes considered as "success" for the operation
//        PAYMENT_SUCCESS = 0, // payment successfully completed
//
//        // codes considered as "failure" for the operation
//        PAYMENT_MALFORMED = -1,          // bad input
//        PAYMENT_UNDERFUNDED = -2,        // not enough funds in source account
//        PAYMENT_SRC_NO_TRUST = -3,       // no trust line on source account
//        PAYMENT_SRC_NOT_AUTHORIZED = -4, // source not authorized to transfer
//        PAYMENT_NO_DESTINATION = -5,     // destination account does not exist
//        PAYMENT_NO_TRUST = -6,       // destination missing a trust line for asset
//        PAYMENT_NOT_AUTHORIZED = -7, // destination not authorized to hold asset
//        PAYMENT_LINE_FULL = -8,      // destination would go above their limit
//        PAYMENT_NO_ISSUER = -9       // missing issuer on asset
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum PaymentResultCode {
    PaymentSuccess = 0,
    PaymentMalformed = -1,
    PaymentUnderfunded = -2,
    PaymentSrcNoTrust = -3,
    PaymentSrcNotAuthorized = -4,
    PaymentNoDestination = -5,
    PaymentNoTrust = -6,
    PaymentNotAuthorized = -7,
    PaymentLineFull = -8,
    PaymentNoIssuer = -9,
}

// PaymentResult is an XDR Union defines as:
//
//   union PaymentResult switch (PaymentResultCode code)
//    {
//    case PAYMENT_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum PaymentResult {
    // IDEN PAYMENT_SUCCESS
    #[discriminant(value = "0")]
    PaymentSuccess(()),
    #[discriminant(value = "-1")]
    PaymentMalformed(()),
    #[discriminant(value = "-2")]
    PaymentUnderfunded(()),
    #[discriminant(value = "-3")]
    PaymentSrcNoTrust(()),
    #[discriminant(value = "-4")]
    PaymentSrcNotAuthorized(()),
    #[discriminant(value = "-5")]
    PaymentNoDestination(()),
    #[discriminant(value = "-6")]
    PaymentNoTrust(()),
    #[discriminant(value = "-7")]
    PaymentNotAuthorized(()),
    #[discriminant(value = "-8")]
    PaymentLineFull(()),
    #[discriminant(value = "-9")]
    PaymentNoIssuer(()),
}

// PathPaymentStrictReceiveResultCode is an XDR Enum defines as:
//
//   enum PathPaymentStrictReceiveResultCode
//    {
//        // codes considered as "success" for the operation
//        PATH_PAYMENT_STRICT_RECEIVE_SUCCESS = 0, // success
//
//        // codes considered as "failure" for the operation
//        PATH_PAYMENT_STRICT_RECEIVE_MALFORMED = -1, // bad input
//        PATH_PAYMENT_STRICT_RECEIVE_UNDERFUNDED =
//            -2, // not enough funds in source account
//        PATH_PAYMENT_STRICT_RECEIVE_SRC_NO_TRUST =
//            -3, // no trust line on source account
//        PATH_PAYMENT_STRICT_RECEIVE_SRC_NOT_AUTHORIZED =
//            -4, // source not authorized to transfer
//        PATH_PAYMENT_STRICT_RECEIVE_NO_DESTINATION =
//            -5, // destination account does not exist
//        PATH_PAYMENT_STRICT_RECEIVE_NO_TRUST =
//            -6, // dest missing a trust line for asset
//        PATH_PAYMENT_STRICT_RECEIVE_NOT_AUTHORIZED =
//            -7, // dest not authorized to hold asset
//        PATH_PAYMENT_STRICT_RECEIVE_LINE_FULL =
//            -8, // dest would go above their limit
//        PATH_PAYMENT_STRICT_RECEIVE_NO_ISSUER = -9, // missing issuer on one asset
//        PATH_PAYMENT_STRICT_RECEIVE_TOO_FEW_OFFERS =
//            -10, // not enough offers to satisfy path
//        PATH_PAYMENT_STRICT_RECEIVE_OFFER_CROSS_SELF =
//            -11, // would cross one of its own offers
//        PATH_PAYMENT_STRICT_RECEIVE_OVER_SENDMAX = -12 // could not satisfy sendmax
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum PathPaymentStrictReceiveResultCode {
    PathPaymentStrictReceiveSuccess = 0,
    PathPaymentStrictReceiveMalformed = -1,
    PathPaymentStrictReceiveUnderfunded = -2,
    PathPaymentStrictReceiveSrcNoTrust = -3,
    PathPaymentStrictReceiveSrcNotAuthorized = -4,
    PathPaymentStrictReceiveNoDestination = -5,
    PathPaymentStrictReceiveNoTrust = -6,
    PathPaymentStrictReceiveNotAuthorized = -7,
    PathPaymentStrictReceiveLineFull = -8,
    PathPaymentStrictReceiveNoIssuer = -9,
    PathPaymentStrictReceiveTooFewOffers = -10,
    PathPaymentStrictReceiveOfferCrossSelf = -11,
    PathPaymentStrictReceiveOverSendmax = -12,
}

// SimplePaymentResult is an XDR Struct defines as:
//
//   struct SimplePaymentResult
//    {
//        AccountID destination;
//        Asset asset;
//        int64 amount;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct SimplePaymentResult {
    // TODO
    pub destination: AccountId,
    // TODO
    pub asset: Asset,
    // TODO
    pub amount: Int64,
}

// PathPaymentStrictReceiveResultSuccess is an XDR NestedStruct defines as:
//
//   struct
//        {
//            ClaimAtom offers<>;
//            SimplePaymentResult last;
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct PathPaymentStrictReceiveResultSuccess {
    // TODO
    pub offers: Vec<ClaimAtom>,
    // TODO
    pub last: SimplePaymentResult,
}

// PathPaymentStrictReceiveResult is an XDR Union defines as:
//
//   union PathPaymentStrictReceiveResult switch (
//        PathPaymentStrictReceiveResultCode code)
//    {
//    case PATH_PAYMENT_STRICT_RECEIVE_SUCCESS:
//        struct
//        {
//            ClaimAtom offers<>;
//            SimplePaymentResult last;
//        } success;
//    case PATH_PAYMENT_STRICT_RECEIVE_NO_ISSUER:
//        Asset noIssuer; // the asset that caused the error
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum PathPaymentStrictReceiveResult {
    // IDEN PATH_PAYMENT_STRICT_RECEIVE_SUCCESS
    #[discriminant(value = "0")]
    PathPaymentStrictReceiveSuccess(PathPaymentStrictReceiveResultSuccess),
    #[discriminant(value = "-1")]
    PathPaymentStrictReceiveMalformed(()),
    #[discriminant(value = "-2")]
    PathPaymentStrictReceiveUnderfunded(()),
    #[discriminant(value = "-3")]
    PathPaymentStrictReceiveSrcNoTrust(()),
    #[discriminant(value = "-4")]
    PathPaymentStrictReceiveSrcNotAuthorized(()),
    #[discriminant(value = "-5")]
    PathPaymentStrictReceiveNoDestination(()),
    #[discriminant(value = "-6")]
    PathPaymentStrictReceiveNoTrust(()),
    #[discriminant(value = "-7")]
    PathPaymentStrictReceiveNotAuthorized(()),
    #[discriminant(value = "-8")]
    PathPaymentStrictReceiveLineFull(()),
    // IDEN PATH_PAYMENT_STRICT_RECEIVE_NO_ISSUER
    #[discriminant(value = "-9")]
    PathPaymentStrictReceiveNoIssuer(Asset),
    #[discriminant(value = "-10")]
    PathPaymentStrictReceiveTooFewOffers(()),
    #[discriminant(value = "-11")]
    PathPaymentStrictReceiveOfferCrossSelf(()),
    #[discriminant(value = "-12")]
    PathPaymentStrictReceiveOverSendmax(()),
}

// PathPaymentStrictSendResultCode is an XDR Enum defines as:
//
//   enum PathPaymentStrictSendResultCode
//    {
//        // codes considered as "success" for the operation
//        PATH_PAYMENT_STRICT_SEND_SUCCESS = 0, // success
//
//        // codes considered as "failure" for the operation
//        PATH_PAYMENT_STRICT_SEND_MALFORMED = -1, // bad input
//        PATH_PAYMENT_STRICT_SEND_UNDERFUNDED =
//            -2, // not enough funds in source account
//        PATH_PAYMENT_STRICT_SEND_SRC_NO_TRUST =
//            -3, // no trust line on source account
//        PATH_PAYMENT_STRICT_SEND_SRC_NOT_AUTHORIZED =
//            -4, // source not authorized to transfer
//        PATH_PAYMENT_STRICT_SEND_NO_DESTINATION =
//            -5, // destination account does not exist
//        PATH_PAYMENT_STRICT_SEND_NO_TRUST =
//            -6, // dest missing a trust line for asset
//        PATH_PAYMENT_STRICT_SEND_NOT_AUTHORIZED =
//            -7, // dest not authorized to hold asset
//        PATH_PAYMENT_STRICT_SEND_LINE_FULL = -8, // dest would go above their limit
//        PATH_PAYMENT_STRICT_SEND_NO_ISSUER = -9, // missing issuer on one asset
//        PATH_PAYMENT_STRICT_SEND_TOO_FEW_OFFERS =
//            -10, // not enough offers to satisfy path
//        PATH_PAYMENT_STRICT_SEND_OFFER_CROSS_SELF =
//            -11, // would cross one of its own offers
//        PATH_PAYMENT_STRICT_SEND_UNDER_DESTMIN = -12 // could not satisfy destMin
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum PathPaymentStrictSendResultCode {
    PathPaymentStrictSendSuccess = 0,
    PathPaymentStrictSendMalformed = -1,
    PathPaymentStrictSendUnderfunded = -2,
    PathPaymentStrictSendSrcNoTrust = -3,
    PathPaymentStrictSendSrcNotAuthorized = -4,
    PathPaymentStrictSendNoDestination = -5,
    PathPaymentStrictSendNoTrust = -6,
    PathPaymentStrictSendNotAuthorized = -7,
    PathPaymentStrictSendLineFull = -8,
    PathPaymentStrictSendNoIssuer = -9,
    PathPaymentStrictSendTooFewOffers = -10,
    PathPaymentStrictSendOfferCrossSelf = -11,
    PathPaymentStrictSendUnderDestmin = -12,
}

// PathPaymentStrictSendResultSuccess is an XDR NestedStruct defines as:
//
//   struct
//        {
//            ClaimAtom offers<>;
//            SimplePaymentResult last;
//        }
//
#[derive(Debug, XDROut, XDRIn)]
pub struct PathPaymentStrictSendResultSuccess {
    // TODO
    pub offers: Vec<ClaimAtom>,
    // TODO
    pub last: SimplePaymentResult,
}

// PathPaymentStrictSendResult is an XDR Union defines as:
//
//   union PathPaymentStrictSendResult switch (PathPaymentStrictSendResultCode code)
//    {
//    case PATH_PAYMENT_STRICT_SEND_SUCCESS:
//        struct
//        {
//            ClaimAtom offers<>;
//            SimplePaymentResult last;
//        } success;
//    case PATH_PAYMENT_STRICT_SEND_NO_ISSUER:
//        Asset noIssuer; // the asset that caused the error
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum PathPaymentStrictSendResult {
    // IDEN PATH_PAYMENT_STRICT_SEND_SUCCESS
    #[discriminant(value = "0")]
    PathPaymentStrictSendSuccess(PathPaymentStrictSendResultSuccess),
    #[discriminant(value = "-1")]
    PathPaymentStrictSendMalformed(()),
    #[discriminant(value = "-2")]
    PathPaymentStrictSendUnderfunded(()),
    #[discriminant(value = "-3")]
    PathPaymentStrictSendSrcNoTrust(()),
    #[discriminant(value = "-4")]
    PathPaymentStrictSendSrcNotAuthorized(()),
    #[discriminant(value = "-5")]
    PathPaymentStrictSendNoDestination(()),
    #[discriminant(value = "-6")]
    PathPaymentStrictSendNoTrust(()),
    #[discriminant(value = "-7")]
    PathPaymentStrictSendNotAuthorized(()),
    #[discriminant(value = "-8")]
    PathPaymentStrictSendLineFull(()),
    // IDEN PATH_PAYMENT_STRICT_SEND_NO_ISSUER
    #[discriminant(value = "-9")]
    PathPaymentStrictSendNoIssuer(Asset),
    #[discriminant(value = "-10")]
    PathPaymentStrictSendTooFewOffers(()),
    #[discriminant(value = "-11")]
    PathPaymentStrictSendOfferCrossSelf(()),
    #[discriminant(value = "-12")]
    PathPaymentStrictSendUnderDestmin(()),
}

// ManageSellOfferResultCode is an XDR Enum defines as:
//
//   enum ManageSellOfferResultCode
//    {
//        // codes considered as "success" for the operation
//        MANAGE_SELL_OFFER_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        MANAGE_SELL_OFFER_MALFORMED = -1, // generated offer would be invalid
//        MANAGE_SELL_OFFER_SELL_NO_TRUST =
//            -2,                              // no trust line for what we're selling
//        MANAGE_SELL_OFFER_BUY_NO_TRUST = -3, // no trust line for what we're buying
//        MANAGE_SELL_OFFER_SELL_NOT_AUTHORIZED = -4, // not authorized to sell
//        MANAGE_SELL_OFFER_BUY_NOT_AUTHORIZED = -5,  // not authorized to buy
//        MANAGE_SELL_OFFER_LINE_FULL = -6, // can't receive more of what it's buying
//        MANAGE_SELL_OFFER_UNDERFUNDED = -7, // doesn't hold what it's trying to sell
//        MANAGE_SELL_OFFER_CROSS_SELF =
//            -8, // would cross an offer from the same user
//        MANAGE_SELL_OFFER_SELL_NO_ISSUER = -9, // no issuer for what we're selling
//        MANAGE_SELL_OFFER_BUY_NO_ISSUER = -10, // no issuer for what we're buying
//
//        // update errors
//        MANAGE_SELL_OFFER_NOT_FOUND =
//            -11, // offerID does not match an existing offer
//
//        MANAGE_SELL_OFFER_LOW_RESERVE =
//            -12 // not enough funds to create a new Offer
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ManageSellOfferResultCode {
    ManageSellOfferSuccess = 0,
    ManageSellOfferMalformed = -1,
    ManageSellOfferSellNoTrust = -2,
    ManageSellOfferBuyNoTrust = -3,
    ManageSellOfferSellNotAuthorized = -4,
    ManageSellOfferBuyNotAuthorized = -5,
    ManageSellOfferLineFull = -6,
    ManageSellOfferUnderfunded = -7,
    ManageSellOfferCrossSelf = -8,
    ManageSellOfferSellNoIssuer = -9,
    ManageSellOfferBuyNoIssuer = -10,
    ManageSellOfferNotFound = -11,
    ManageSellOfferLowReserve = -12,
}

// ManageOfferEffect is an XDR Enum defines as:
//
//   enum ManageOfferEffect
//    {
//        MANAGE_OFFER_CREATED = 0,
//        MANAGE_OFFER_UPDATED = 1,
//        MANAGE_OFFER_DELETED = 2
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ManageOfferEffect {
    ManageOfferCreated = 0,
    ManageOfferUpdated = 1,
    ManageOfferDeleted = 2,
}

// ManageOfferSuccessResultOffer is an XDR NestedUnion defines as:
//
//   union switch (ManageOfferEffect effect)
//        {
//        case MANAGE_OFFER_CREATED:
//        case MANAGE_OFFER_UPDATED:
//            OfferEntry offer;
//        default:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ManageOfferSuccessResultOffer {
    // IDEN MANAGE_OFFER_CREATED
    #[discriminant(value = "0")]
    ManageOfferCreated(OfferEntry),
    // IDEN MANAGE_OFFER_UPDATED
    #[discriminant(value = "1")]
    ManageOfferUpdated(OfferEntry),
    #[discriminant(value = "2")]
    ManageOfferDeleted(()),
}

// ManageOfferSuccessResult is an XDR Struct defines as:
//
//   struct ManageOfferSuccessResult
//    {
//        // offers that got claimed while creating this offer
//        ClaimAtom offersClaimed<>;
//
//        union switch (ManageOfferEffect effect)
//        {
//        case MANAGE_OFFER_CREATED:
//        case MANAGE_OFFER_UPDATED:
//            OfferEntry offer;
//        default:
//            void;
//        }
//        offer;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct ManageOfferSuccessResult {
    // TODO
    pub offers_claimed: Vec<ClaimAtom>,
    // TODO
    pub offer: ManageOfferSuccessResultOffer,
}

// ManageSellOfferResult is an XDR Union defines as:
//
//   union ManageSellOfferResult switch (ManageSellOfferResultCode code)
//    {
//    case MANAGE_SELL_OFFER_SUCCESS:
//        ManageOfferSuccessResult success;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ManageSellOfferResult {
    // IDEN MANAGE_SELL_OFFER_SUCCESS
    #[discriminant(value = "0")]
    ManageSellOfferSuccess(ManageOfferSuccessResult),
    #[discriminant(value = "-1")]
    ManageSellOfferMalformed(()),
    #[discriminant(value = "-2")]
    ManageSellOfferSellNoTrust(()),
    #[discriminant(value = "-3")]
    ManageSellOfferBuyNoTrust(()),
    #[discriminant(value = "-4")]
    ManageSellOfferSellNotAuthorized(()),
    #[discriminant(value = "-5")]
    ManageSellOfferBuyNotAuthorized(()),
    #[discriminant(value = "-6")]
    ManageSellOfferLineFull(()),
    #[discriminant(value = "-7")]
    ManageSellOfferUnderfunded(()),
    #[discriminant(value = "-8")]
    ManageSellOfferCrossSelf(()),
    #[discriminant(value = "-9")]
    ManageSellOfferSellNoIssuer(()),
    #[discriminant(value = "-10")]
    ManageSellOfferBuyNoIssuer(()),
    #[discriminant(value = "-11")]
    ManageSellOfferNotFound(()),
    #[discriminant(value = "-12")]
    ManageSellOfferLowReserve(()),
}

// ManageBuyOfferResultCode is an XDR Enum defines as:
//
//   enum ManageBuyOfferResultCode
//    {
//        // codes considered as "success" for the operation
//        MANAGE_BUY_OFFER_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        MANAGE_BUY_OFFER_MALFORMED = -1,     // generated offer would be invalid
//        MANAGE_BUY_OFFER_SELL_NO_TRUST = -2, // no trust line for what we're selling
//        MANAGE_BUY_OFFER_BUY_NO_TRUST = -3,  // no trust line for what we're buying
//        MANAGE_BUY_OFFER_SELL_NOT_AUTHORIZED = -4, // not authorized to sell
//        MANAGE_BUY_OFFER_BUY_NOT_AUTHORIZED = -5,  // not authorized to buy
//        MANAGE_BUY_OFFER_LINE_FULL = -6,   // can't receive more of what it's buying
//        MANAGE_BUY_OFFER_UNDERFUNDED = -7, // doesn't hold what it's trying to sell
//        MANAGE_BUY_OFFER_CROSS_SELF = -8, // would cross an offer from the same user
//        MANAGE_BUY_OFFER_SELL_NO_ISSUER = -9, // no issuer for what we're selling
//        MANAGE_BUY_OFFER_BUY_NO_ISSUER = -10, // no issuer for what we're buying
//
//        // update errors
//        MANAGE_BUY_OFFER_NOT_FOUND =
//            -11, // offerID does not match an existing offer
//
//        MANAGE_BUY_OFFER_LOW_RESERVE = -12 // not enough funds to create a new Offer
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ManageBuyOfferResultCode {
    ManageBuyOfferSuccess = 0,
    ManageBuyOfferMalformed = -1,
    ManageBuyOfferSellNoTrust = -2,
    ManageBuyOfferBuyNoTrust = -3,
    ManageBuyOfferSellNotAuthorized = -4,
    ManageBuyOfferBuyNotAuthorized = -5,
    ManageBuyOfferLineFull = -6,
    ManageBuyOfferUnderfunded = -7,
    ManageBuyOfferCrossSelf = -8,
    ManageBuyOfferSellNoIssuer = -9,
    ManageBuyOfferBuyNoIssuer = -10,
    ManageBuyOfferNotFound = -11,
    ManageBuyOfferLowReserve = -12,
}

// ManageBuyOfferResult is an XDR Union defines as:
//
//   union ManageBuyOfferResult switch (ManageBuyOfferResultCode code)
//    {
//    case MANAGE_BUY_OFFER_SUCCESS:
//        ManageOfferSuccessResult success;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ManageBuyOfferResult {
    // IDEN MANAGE_BUY_OFFER_SUCCESS
    #[discriminant(value = "0")]
    ManageBuyOfferSuccess(ManageOfferSuccessResult),
    #[discriminant(value = "-1")]
    ManageBuyOfferMalformed(()),
    #[discriminant(value = "-2")]
    ManageBuyOfferSellNoTrust(()),
    #[discriminant(value = "-3")]
    ManageBuyOfferBuyNoTrust(()),
    #[discriminant(value = "-4")]
    ManageBuyOfferSellNotAuthorized(()),
    #[discriminant(value = "-5")]
    ManageBuyOfferBuyNotAuthorized(()),
    #[discriminant(value = "-6")]
    ManageBuyOfferLineFull(()),
    #[discriminant(value = "-7")]
    ManageBuyOfferUnderfunded(()),
    #[discriminant(value = "-8")]
    ManageBuyOfferCrossSelf(()),
    #[discriminant(value = "-9")]
    ManageBuyOfferSellNoIssuer(()),
    #[discriminant(value = "-10")]
    ManageBuyOfferBuyNoIssuer(()),
    #[discriminant(value = "-11")]
    ManageBuyOfferNotFound(()),
    #[discriminant(value = "-12")]
    ManageBuyOfferLowReserve(()),
}

// SetOptionsResultCode is an XDR Enum defines as:
//
//   enum SetOptionsResultCode
//    {
//        // codes considered as "success" for the operation
//        SET_OPTIONS_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        SET_OPTIONS_LOW_RESERVE = -1,      // not enough funds to add a signer
//        SET_OPTIONS_TOO_MANY_SIGNERS = -2, // max number of signers already reached
//        SET_OPTIONS_BAD_FLAGS = -3,        // invalid combination of clear/set flags
//        SET_OPTIONS_INVALID_INFLATION = -4,      // inflation account does not exist
//        SET_OPTIONS_CANT_CHANGE = -5,            // can no longer change this option
//        SET_OPTIONS_UNKNOWN_FLAG = -6,           // can't set an unknown flag
//        SET_OPTIONS_THRESHOLD_OUT_OF_RANGE = -7, // bad value for weight/threshold
//        SET_OPTIONS_BAD_SIGNER = -8,             // signer cannot be masterkey
//        SET_OPTIONS_INVALID_HOME_DOMAIN = -9,    // malformed home domain
//        SET_OPTIONS_AUTH_REVOCABLE_REQUIRED =
//            -10 // auth revocable is required for clawback
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum SetOptionsResultCode {
    SetOptionsSuccess = 0,
    SetOptionsLowReserve = -1,
    SetOptionsTooManySigners = -2,
    SetOptionsBadFlags = -3,
    SetOptionsInvalidInflation = -4,
    SetOptionsCantChange = -5,
    SetOptionsUnknownFlag = -6,
    SetOptionsThresholdOutOfRange = -7,
    SetOptionsBadSigner = -8,
    SetOptionsInvalidHomeDomain = -9,
    SetOptionsAuthRevocableRequired = -10,
}

// SetOptionsResult is an XDR Union defines as:
//
//   union SetOptionsResult switch (SetOptionsResultCode code)
//    {
//    case SET_OPTIONS_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum SetOptionsResult {
    // IDEN SET_OPTIONS_SUCCESS
    #[discriminant(value = "0")]
    SetOptionsSuccess(()),
    #[discriminant(value = "-1")]
    SetOptionsLowReserve(()),
    #[discriminant(value = "-2")]
    SetOptionsTooManySigners(()),
    #[discriminant(value = "-3")]
    SetOptionsBadFlags(()),
    #[discriminant(value = "-4")]
    SetOptionsInvalidInflation(()),
    #[discriminant(value = "-5")]
    SetOptionsCantChange(()),
    #[discriminant(value = "-6")]
    SetOptionsUnknownFlag(()),
    #[discriminant(value = "-7")]
    SetOptionsThresholdOutOfRange(()),
    #[discriminant(value = "-8")]
    SetOptionsBadSigner(()),
    #[discriminant(value = "-9")]
    SetOptionsInvalidHomeDomain(()),
    #[discriminant(value = "-10")]
    SetOptionsAuthRevocableRequired(()),
}

// ChangeTrustResultCode is an XDR Enum defines as:
//
//   enum ChangeTrustResultCode
//    {
//        // codes considered as "success" for the operation
//        CHANGE_TRUST_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        CHANGE_TRUST_MALFORMED = -1,     // bad input
//        CHANGE_TRUST_NO_ISSUER = -2,     // could not find issuer
//        CHANGE_TRUST_INVALID_LIMIT = -3, // cannot drop limit below balance
//                                         // cannot create with a limit of 0
//        CHANGE_TRUST_LOW_RESERVE =
//            -4, // not enough funds to create a new trust line,
//        CHANGE_TRUST_SELF_NOT_ALLOWED = -5, // trusting self is not allowed
//        CHANGE_TRUST_TRUST_LINE_MISSING = -6, // Asset trustline is missing for pool
//        CHANGE_TRUST_CANNOT_DELETE = -7, // Asset trustline is still referenced in a pool
//        CHANGE_TRUST_NOT_AUTH_MAINTAIN_LIABILITIES = -8 // Asset trustline is deauthorized
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ChangeTrustResultCode {
    ChangeTrustSuccess = 0,
    ChangeTrustMalformed = -1,
    ChangeTrustNoIssuer = -2,
    ChangeTrustInvalidLimit = -3,
    ChangeTrustLowReserve = -4,
    ChangeTrustSelfNotAllowed = -5,
    ChangeTrustTrustLineMissing = -6,
    ChangeTrustCannotDelete = -7,
    ChangeTrustNotAuthMaintainLiabilities = -8,
}

// ChangeTrustResult is an XDR Union defines as:
//
//   union ChangeTrustResult switch (ChangeTrustResultCode code)
//    {
//    case CHANGE_TRUST_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ChangeTrustResult {
    // IDEN CHANGE_TRUST_SUCCESS
    #[discriminant(value = "0")]
    ChangeTrustSuccess(()),
    #[discriminant(value = "-1")]
    ChangeTrustMalformed(()),
    #[discriminant(value = "-2")]
    ChangeTrustNoIssuer(()),
    #[discriminant(value = "-3")]
    ChangeTrustInvalidLimit(()),
    #[discriminant(value = "-4")]
    ChangeTrustLowReserve(()),
    #[discriminant(value = "-5")]
    ChangeTrustSelfNotAllowed(()),
    #[discriminant(value = "-6")]
    ChangeTrustTrustLineMissing(()),
    #[discriminant(value = "-7")]
    ChangeTrustCannotDelete(()),
    #[discriminant(value = "-8")]
    ChangeTrustNotAuthMaintainLiabilities(()),
}

// AllowTrustResultCode is an XDR Enum defines as:
//
//   enum AllowTrustResultCode
//    {
//        // codes considered as "success" for the operation
//        ALLOW_TRUST_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        ALLOW_TRUST_MALFORMED = -1,     // asset is not ASSET_TYPE_ALPHANUM
//        ALLOW_TRUST_NO_TRUST_LINE = -2, // trustor does not have a trustline
//                                        // source account does not require trust
//        ALLOW_TRUST_TRUST_NOT_REQUIRED = -3,
//        ALLOW_TRUST_CANT_REVOKE = -4,     // source account can't revoke trust,
//        ALLOW_TRUST_SELF_NOT_ALLOWED = -5, // trusting self is not allowed
//        ALLOW_TRUST_LOW_RESERVE = -6 // claimable balances can't be created
//                                     // on revoke due to low reserves
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum AllowTrustResultCode {
    AllowTrustSuccess = 0,
    AllowTrustMalformed = -1,
    AllowTrustNoTrustLine = -2,
    AllowTrustTrustNotRequired = -3,
    AllowTrustCantRevoke = -4,
    AllowTrustSelfNotAllowed = -5,
    AllowTrustLowReserve = -6,
}

// AllowTrustResult is an XDR Union defines as:
//
//   union AllowTrustResult switch (AllowTrustResultCode code)
//    {
//    case ALLOW_TRUST_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum AllowTrustResult {
    // IDEN ALLOW_TRUST_SUCCESS
    #[discriminant(value = "0")]
    AllowTrustSuccess(()),
    #[discriminant(value = "-1")]
    AllowTrustMalformed(()),
    #[discriminant(value = "-2")]
    AllowTrustNoTrustLine(()),
    #[discriminant(value = "-3")]
    AllowTrustTrustNotRequired(()),
    #[discriminant(value = "-4")]
    AllowTrustCantRevoke(()),
    #[discriminant(value = "-5")]
    AllowTrustSelfNotAllowed(()),
    #[discriminant(value = "-6")]
    AllowTrustLowReserve(()),
}

// AccountMergeResultCode is an XDR Enum defines as:
//
//   enum AccountMergeResultCode
//    {
//        // codes considered as "success" for the operation
//        ACCOUNT_MERGE_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        ACCOUNT_MERGE_MALFORMED = -1,       // can't merge onto itself
//        ACCOUNT_MERGE_NO_ACCOUNT = -2,      // destination does not exist
//        ACCOUNT_MERGE_IMMUTABLE_SET = -3,   // source account has AUTH_IMMUTABLE set
//        ACCOUNT_MERGE_HAS_SUB_ENTRIES = -4, // account has trust lines/offers
//        ACCOUNT_MERGE_SEQNUM_TOO_FAR = -5,  // sequence number is over max allowed
//        ACCOUNT_MERGE_DEST_FULL = -6,       // can't add source balance to
//                                            // destination balance
//        ACCOUNT_MERGE_IS_SPONSOR = -7       // can't merge account that is a sponsor
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum AccountMergeResultCode {
    AccountMergeSuccess = 0,
    AccountMergeMalformed = -1,
    AccountMergeNoAccount = -2,
    AccountMergeImmutableSet = -3,
    AccountMergeHasSubEntries = -4,
    AccountMergeSeqnumTooFar = -5,
    AccountMergeDestFull = -6,
    AccountMergeIsSponsor = -7,
}

// AccountMergeResult is an XDR Union defines as:
//
//   union AccountMergeResult switch (AccountMergeResultCode code)
//    {
//    case ACCOUNT_MERGE_SUCCESS:
//        int64 sourceAccountBalance; // how much got transferred from source account
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum AccountMergeResult {
    // IDEN ACCOUNT_MERGE_SUCCESS
    #[discriminant(value = "0")]
    AccountMergeSuccess(Int64),
    #[discriminant(value = "-1")]
    AccountMergeMalformed(()),
    #[discriminant(value = "-2")]
    AccountMergeNoAccount(()),
    #[discriminant(value = "-3")]
    AccountMergeImmutableSet(()),
    #[discriminant(value = "-4")]
    AccountMergeHasSubEntries(()),
    #[discriminant(value = "-5")]
    AccountMergeSeqnumTooFar(()),
    #[discriminant(value = "-6")]
    AccountMergeDestFull(()),
    #[discriminant(value = "-7")]
    AccountMergeIsSponsor(()),
}

// InflationResultCode is an XDR Enum defines as:
//
//   enum InflationResultCode
//    {
//        // codes considered as "success" for the operation
//        INFLATION_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        INFLATION_NOT_TIME = -1
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum InflationResultCode {
    InflationSuccess = 0,
    InflationNotTime = -1,
}

// InflationPayout is an XDR Struct defines as:
//
//   struct InflationPayout // or use PaymentResultAtom to limit types?
//    {
//        AccountID destination;
//        int64 amount;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct InflationPayout {
    // TODO
    pub destination: AccountId,
    // TODO
    pub amount: Int64,
}

// InflationResult is an XDR Union defines as:
//
//   union InflationResult switch (InflationResultCode code)
//    {
//    case INFLATION_SUCCESS:
//        InflationPayout payouts<>;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum InflationResult {
    // IDEN INFLATION_SUCCESS
    #[discriminant(value = "0")]
    InflationSuccess(Vec<InflationPayout>),
    #[discriminant(value = "-1")]
    InflationNotTime(()),
}

// ManageDataResultCode is an XDR Enum defines as:
//
//   enum ManageDataResultCode
//    {
//        // codes considered as "success" for the operation
//        MANAGE_DATA_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        MANAGE_DATA_NOT_SUPPORTED_YET =
//            -1, // The network hasn't moved to this protocol change yet
//        MANAGE_DATA_NAME_NOT_FOUND =
//            -2, // Trying to remove a Data Entry that isn't there
//        MANAGE_DATA_LOW_RESERVE = -3, // not enough funds to create a new Data Entry
//        MANAGE_DATA_INVALID_NAME = -4 // Name not a valid string
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ManageDataResultCode {
    ManageDataSuccess = 0,
    ManageDataNotSupportedYet = -1,
    ManageDataNameNotFound = -2,
    ManageDataLowReserve = -3,
    ManageDataInvalidName = -4,
}

// ManageDataResult is an XDR Union defines as:
//
//   union ManageDataResult switch (ManageDataResultCode code)
//    {
//    case MANAGE_DATA_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ManageDataResult {
    // IDEN MANAGE_DATA_SUCCESS
    #[discriminant(value = "0")]
    ManageDataSuccess(()),
    #[discriminant(value = "-1")]
    ManageDataNotSupportedYet(()),
    #[discriminant(value = "-2")]
    ManageDataNameNotFound(()),
    #[discriminant(value = "-3")]
    ManageDataLowReserve(()),
    #[discriminant(value = "-4")]
    ManageDataInvalidName(()),
}

// BumpSequenceResultCode is an XDR Enum defines as:
//
//   enum BumpSequenceResultCode
//    {
//        // codes considered as "success" for the operation
//        BUMP_SEQUENCE_SUCCESS = 0,
//        // codes considered as "failure" for the operation
//        BUMP_SEQUENCE_BAD_SEQ = -1 // `bumpTo` is not within bounds
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum BumpSequenceResultCode {
    BumpSequenceSuccess = 0,
    BumpSequenceBadSeq = -1,
}

// BumpSequenceResult is an XDR Union defines as:
//
//   union BumpSequenceResult switch (BumpSequenceResultCode code)
//    {
//    case BUMP_SEQUENCE_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum BumpSequenceResult {
    // IDEN BUMP_SEQUENCE_SUCCESS
    #[discriminant(value = "0")]
    BumpSequenceSuccess(()),
    #[discriminant(value = "-1")]
    BumpSequenceBadSeq(()),
}

// CreateClaimableBalanceResultCode is an XDR Enum defines as:
//
//   enum CreateClaimableBalanceResultCode
//    {
//        CREATE_CLAIMABLE_BALANCE_SUCCESS = 0,
//        CREATE_CLAIMABLE_BALANCE_MALFORMED = -1,
//        CREATE_CLAIMABLE_BALANCE_LOW_RESERVE = -2,
//        CREATE_CLAIMABLE_BALANCE_NO_TRUST = -3,
//        CREATE_CLAIMABLE_BALANCE_NOT_AUTHORIZED = -4,
//        CREATE_CLAIMABLE_BALANCE_UNDERFUNDED = -5
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum CreateClaimableBalanceResultCode {
    CreateClaimableBalanceSuccess = 0,
    CreateClaimableBalanceMalformed = -1,
    CreateClaimableBalanceLowReserve = -2,
    CreateClaimableBalanceNoTrust = -3,
    CreateClaimableBalanceNotAuthorized = -4,
    CreateClaimableBalanceUnderfunded = -5,
}

// CreateClaimableBalanceResult is an XDR Union defines as:
//
//   union CreateClaimableBalanceResult switch (
//        CreateClaimableBalanceResultCode code)
//    {
//    case CREATE_CLAIMABLE_BALANCE_SUCCESS:
//        ClaimableBalanceID balanceID;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum CreateClaimableBalanceResult {
    // IDEN CREATE_CLAIMABLE_BALANCE_SUCCESS
    #[discriminant(value = "0")]
    CreateClaimableBalanceSuccess(ClaimableBalanceId),
    #[discriminant(value = "-1")]
    CreateClaimableBalanceMalformed(()),
    #[discriminant(value = "-2")]
    CreateClaimableBalanceLowReserve(()),
    #[discriminant(value = "-3")]
    CreateClaimableBalanceNoTrust(()),
    #[discriminant(value = "-4")]
    CreateClaimableBalanceNotAuthorized(()),
    #[discriminant(value = "-5")]
    CreateClaimableBalanceUnderfunded(()),
}

// ClaimClaimableBalanceResultCode is an XDR Enum defines as:
//
//   enum ClaimClaimableBalanceResultCode
//    {
//        CLAIM_CLAIMABLE_BALANCE_SUCCESS = 0,
//        CLAIM_CLAIMABLE_BALANCE_DOES_NOT_EXIST = -1,
//        CLAIM_CLAIMABLE_BALANCE_CANNOT_CLAIM = -2,
//        CLAIM_CLAIMABLE_BALANCE_LINE_FULL = -3,
//        CLAIM_CLAIMABLE_BALANCE_NO_TRUST = -4,
//        CLAIM_CLAIMABLE_BALANCE_NOT_AUTHORIZED = -5
//
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ClaimClaimableBalanceResultCode {
    ClaimClaimableBalanceSuccess = 0,
    ClaimClaimableBalanceDoesNotExist = -1,
    ClaimClaimableBalanceCannotClaim = -2,
    ClaimClaimableBalanceLineFull = -3,
    ClaimClaimableBalanceNoTrust = -4,
    ClaimClaimableBalanceNotAuthorized = -5,
}

// ClaimClaimableBalanceResult is an XDR Union defines as:
//
//   union ClaimClaimableBalanceResult switch (ClaimClaimableBalanceResultCode code)
//    {
//    case CLAIM_CLAIMABLE_BALANCE_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ClaimClaimableBalanceResult {
    // IDEN CLAIM_CLAIMABLE_BALANCE_SUCCESS
    #[discriminant(value = "0")]
    ClaimClaimableBalanceSuccess(()),
    #[discriminant(value = "-1")]
    ClaimClaimableBalanceDoesNotExist(()),
    #[discriminant(value = "-2")]
    ClaimClaimableBalanceCannotClaim(()),
    #[discriminant(value = "-3")]
    ClaimClaimableBalanceLineFull(()),
    #[discriminant(value = "-4")]
    ClaimClaimableBalanceNoTrust(()),
    #[discriminant(value = "-5")]
    ClaimClaimableBalanceNotAuthorized(()),
}

// BeginSponsoringFutureReservesResultCode is an XDR Enum defines as:
//
//   enum BeginSponsoringFutureReservesResultCode
//    {
//        // codes considered as "success" for the operation
//        BEGIN_SPONSORING_FUTURE_RESERVES_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        BEGIN_SPONSORING_FUTURE_RESERVES_MALFORMED = -1,
//        BEGIN_SPONSORING_FUTURE_RESERVES_ALREADY_SPONSORED = -2,
//        BEGIN_SPONSORING_FUTURE_RESERVES_RECURSIVE = -3
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum BeginSponsoringFutureReservesResultCode {
    BeginSponsoringFutureReservesSuccess = 0,
    BeginSponsoringFutureReservesMalformed = -1,
    BeginSponsoringFutureReservesAlreadySponsored = -2,
    BeginSponsoringFutureReservesRecursive = -3,
}

// BeginSponsoringFutureReservesResult is an XDR Union defines as:
//
//   union BeginSponsoringFutureReservesResult switch (
//        BeginSponsoringFutureReservesResultCode code)
//    {
//    case BEGIN_SPONSORING_FUTURE_RESERVES_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum BeginSponsoringFutureReservesResult {
    // IDEN BEGIN_SPONSORING_FUTURE_RESERVES_SUCCESS
    #[discriminant(value = "0")]
    BeginSponsoringFutureReservesSuccess(()),
    #[discriminant(value = "-1")]
    BeginSponsoringFutureReservesMalformed(()),
    #[discriminant(value = "-2")]
    BeginSponsoringFutureReservesAlreadySponsored(()),
    #[discriminant(value = "-3")]
    BeginSponsoringFutureReservesRecursive(()),
}

// EndSponsoringFutureReservesResultCode is an XDR Enum defines as:
//
//   enum EndSponsoringFutureReservesResultCode
//    {
//        // codes considered as "success" for the operation
//        END_SPONSORING_FUTURE_RESERVES_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        END_SPONSORING_FUTURE_RESERVES_NOT_SPONSORED = -1
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum EndSponsoringFutureReservesResultCode {
    EndSponsoringFutureReservesSuccess = 0,
    EndSponsoringFutureReservesNotSponsored = -1,
}

// EndSponsoringFutureReservesResult is an XDR Union defines as:
//
//   union EndSponsoringFutureReservesResult switch (
//        EndSponsoringFutureReservesResultCode code)
//    {
//    case END_SPONSORING_FUTURE_RESERVES_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum EndSponsoringFutureReservesResult {
    // IDEN END_SPONSORING_FUTURE_RESERVES_SUCCESS
    #[discriminant(value = "0")]
    EndSponsoringFutureReservesSuccess(()),
    #[discriminant(value = "-1")]
    EndSponsoringFutureReservesNotSponsored(()),
}

// RevokeSponsorshipResultCode is an XDR Enum defines as:
//
//   enum RevokeSponsorshipResultCode
//    {
//        // codes considered as "success" for the operation
//        REVOKE_SPONSORSHIP_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        REVOKE_SPONSORSHIP_DOES_NOT_EXIST = -1,
//        REVOKE_SPONSORSHIP_NOT_SPONSOR = -2,
//        REVOKE_SPONSORSHIP_LOW_RESERVE = -3,
//        REVOKE_SPONSORSHIP_ONLY_TRANSFERABLE = -4,
//        REVOKE_SPONSORSHIP_MALFORMED = -5
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum RevokeSponsorshipResultCode {
    RevokeSponsorshipSuccess = 0,
    RevokeSponsorshipDoesNotExist = -1,
    RevokeSponsorshipNotSponsor = -2,
    RevokeSponsorshipLowReserve = -3,
    RevokeSponsorshipOnlyTransferable = -4,
    RevokeSponsorshipMalformed = -5,
}

// RevokeSponsorshipResult is an XDR Union defines as:
//
//   union RevokeSponsorshipResult switch (RevokeSponsorshipResultCode code)
//    {
//    case REVOKE_SPONSORSHIP_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum RevokeSponsorshipResult {
    // IDEN REVOKE_SPONSORSHIP_SUCCESS
    #[discriminant(value = "0")]
    RevokeSponsorshipSuccess(()),
    #[discriminant(value = "-1")]
    RevokeSponsorshipDoesNotExist(()),
    #[discriminant(value = "-2")]
    RevokeSponsorshipNotSponsor(()),
    #[discriminant(value = "-3")]
    RevokeSponsorshipLowReserve(()),
    #[discriminant(value = "-4")]
    RevokeSponsorshipOnlyTransferable(()),
    #[discriminant(value = "-5")]
    RevokeSponsorshipMalformed(()),
}

// ClawbackResultCode is an XDR Enum defines as:
//
//   enum ClawbackResultCode
//    {
//        // codes considered as "success" for the operation
//        CLAWBACK_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        CLAWBACK_MALFORMED = -1,
//        CLAWBACK_NOT_CLAWBACK_ENABLED = -2,
//        CLAWBACK_NO_TRUST = -3,
//        CLAWBACK_UNDERFUNDED = -4
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ClawbackResultCode {
    ClawbackSuccess = 0,
    ClawbackMalformed = -1,
    ClawbackNotClawbackEnabled = -2,
    ClawbackNoTrust = -3,
    ClawbackUnderfunded = -4,
}

// ClawbackResult is an XDR Union defines as:
//
//   union ClawbackResult switch (ClawbackResultCode code)
//    {
//    case CLAWBACK_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ClawbackResult {
    // IDEN CLAWBACK_SUCCESS
    #[discriminant(value = "0")]
    ClawbackSuccess(()),
    #[discriminant(value = "-1")]
    ClawbackMalformed(()),
    #[discriminant(value = "-2")]
    ClawbackNotClawbackEnabled(()),
    #[discriminant(value = "-3")]
    ClawbackNoTrust(()),
    #[discriminant(value = "-4")]
    ClawbackUnderfunded(()),
}

// ClawbackClaimableBalanceResultCode is an XDR Enum defines as:
//
//   enum ClawbackClaimableBalanceResultCode
//    {
//        // codes considered as "success" for the operation
//        CLAWBACK_CLAIMABLE_BALANCE_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        CLAWBACK_CLAIMABLE_BALANCE_DOES_NOT_EXIST = -1,
//        CLAWBACK_CLAIMABLE_BALANCE_NOT_ISSUER = -2,
//        CLAWBACK_CLAIMABLE_BALANCE_NOT_CLAWBACK_ENABLED = -3
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum ClawbackClaimableBalanceResultCode {
    ClawbackClaimableBalanceSuccess = 0,
    ClawbackClaimableBalanceDoesNotExist = -1,
    ClawbackClaimableBalanceNotIssuer = -2,
    ClawbackClaimableBalanceNotClawbackEnabled = -3,
}

// ClawbackClaimableBalanceResult is an XDR Union defines as:
//
//   union ClawbackClaimableBalanceResult switch (
//        ClawbackClaimableBalanceResultCode code)
//    {
//    case CLAWBACK_CLAIMABLE_BALANCE_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum ClawbackClaimableBalanceResult {
    // IDEN CLAWBACK_CLAIMABLE_BALANCE_SUCCESS
    #[discriminant(value = "0")]
    ClawbackClaimableBalanceSuccess(()),
    #[discriminant(value = "-1")]
    ClawbackClaimableBalanceDoesNotExist(()),
    #[discriminant(value = "-2")]
    ClawbackClaimableBalanceNotIssuer(()),
    #[discriminant(value = "-3")]
    ClawbackClaimableBalanceNotClawbackEnabled(()),
}

// SetTrustLineFlagsResultCode is an XDR Enum defines as:
//
//   enum SetTrustLineFlagsResultCode
//    {
//        // codes considered as "success" for the operation
//        SET_TRUST_LINE_FLAGS_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        SET_TRUST_LINE_FLAGS_MALFORMED = -1,
//        SET_TRUST_LINE_FLAGS_NO_TRUST_LINE = -2,
//        SET_TRUST_LINE_FLAGS_CANT_REVOKE = -3,
//        SET_TRUST_LINE_FLAGS_INVALID_STATE = -4,
//        SET_TRUST_LINE_FLAGS_LOW_RESERVE = -5 // claimable balances can't be created
//                                              // on revoke due to low reserves
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum SetTrustLineFlagsResultCode {
    SetTrustLineFlagsSuccess = 0,
    SetTrustLineFlagsMalformed = -1,
    SetTrustLineFlagsNoTrustLine = -2,
    SetTrustLineFlagsCantRevoke = -3,
    SetTrustLineFlagsInvalidState = -4,
    SetTrustLineFlagsLowReserve = -5,
}

// SetTrustLineFlagsResult is an XDR Union defines as:
//
//   union SetTrustLineFlagsResult switch (SetTrustLineFlagsResultCode code)
//    {
//    case SET_TRUST_LINE_FLAGS_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum SetTrustLineFlagsResult {
    // IDEN SET_TRUST_LINE_FLAGS_SUCCESS
    #[discriminant(value = "0")]
    SetTrustLineFlagsSuccess(()),
    #[discriminant(value = "-1")]
    SetTrustLineFlagsMalformed(()),
    #[discriminant(value = "-2")]
    SetTrustLineFlagsNoTrustLine(()),
    #[discriminant(value = "-3")]
    SetTrustLineFlagsCantRevoke(()),
    #[discriminant(value = "-4")]
    SetTrustLineFlagsInvalidState(()),
    #[discriminant(value = "-5")]
    SetTrustLineFlagsLowReserve(()),
}

// LiquidityPoolDepositResultCode is an XDR Enum defines as:
//
//   enum LiquidityPoolDepositResultCode
//    {
//        // codes considered as "success" for the operation
//        LIQUIDITY_POOL_DEPOSIT_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        LIQUIDITY_POOL_DEPOSIT_MALFORMED = -1,      // bad input
//        LIQUIDITY_POOL_DEPOSIT_NO_TRUST = -2,       // no trust line for one of the
//                                                    // assets
//        LIQUIDITY_POOL_DEPOSIT_NOT_AUTHORIZED = -3, // not authorized for one of the
//                                                    // assets
//        LIQUIDITY_POOL_DEPOSIT_UNDERFUNDED = -4,    // not enough balance for one of
//                                                    // the assets
//        LIQUIDITY_POOL_DEPOSIT_LINE_FULL = -5,      // pool share trust line doesn't
//                                                    // have sufficient limit
//        LIQUIDITY_POOL_DEPOSIT_BAD_PRICE = -6,      // deposit price outside bounds
//        LIQUIDITY_POOL_DEPOSIT_POOL_FULL = -7       // pool reserves are full
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum LiquidityPoolDepositResultCode {
    LiquidityPoolDepositSuccess = 0,
    LiquidityPoolDepositMalformed = -1,
    LiquidityPoolDepositNoTrust = -2,
    LiquidityPoolDepositNotAuthorized = -3,
    LiquidityPoolDepositUnderfunded = -4,
    LiquidityPoolDepositLineFull = -5,
    LiquidityPoolDepositBadPrice = -6,
    LiquidityPoolDepositPoolFull = -7,
}

// LiquidityPoolDepositResult is an XDR Union defines as:
//
//   union LiquidityPoolDepositResult switch (
//        LiquidityPoolDepositResultCode code)
//    {
//    case LIQUIDITY_POOL_DEPOSIT_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LiquidityPoolDepositResult {
    // IDEN LIQUIDITY_POOL_DEPOSIT_SUCCESS
    #[discriminant(value = "0")]
    LiquidityPoolDepositSuccess(()),
    #[discriminant(value = "-1")]
    LiquidityPoolDepositMalformed(()),
    #[discriminant(value = "-2")]
    LiquidityPoolDepositNoTrust(()),
    #[discriminant(value = "-3")]
    LiquidityPoolDepositNotAuthorized(()),
    #[discriminant(value = "-4")]
    LiquidityPoolDepositUnderfunded(()),
    #[discriminant(value = "-5")]
    LiquidityPoolDepositLineFull(()),
    #[discriminant(value = "-6")]
    LiquidityPoolDepositBadPrice(()),
    #[discriminant(value = "-7")]
    LiquidityPoolDepositPoolFull(()),
}

// LiquidityPoolWithdrawResultCode is an XDR Enum defines as:
//
//   enum LiquidityPoolWithdrawResultCode
//    {
//        // codes considered as "success" for the operation
//        LIQUIDITY_POOL_WITHDRAW_SUCCESS = 0,
//
//        // codes considered as "failure" for the operation
//        LIQUIDITY_POOL_WITHDRAW_MALFORMED = -1,      // bad input
//        LIQUIDITY_POOL_WITHDRAW_NO_TRUST = -2,       // no trust line for one of the
//                                                     // assets
//        LIQUIDITY_POOL_WITHDRAW_UNDERFUNDED = -3,    // not enough balance of the
//                                                     // pool share
//        LIQUIDITY_POOL_WITHDRAW_LINE_FULL = -4,      // would go above limit for one
//                                                     // of the assets
//        LIQUIDITY_POOL_WITHDRAW_UNDER_MINIMUM = -5   // didn't withdraw enough
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum LiquidityPoolWithdrawResultCode {
    LiquidityPoolWithdrawSuccess = 0,
    LiquidityPoolWithdrawMalformed = -1,
    LiquidityPoolWithdrawNoTrust = -2,
    LiquidityPoolWithdrawUnderfunded = -3,
    LiquidityPoolWithdrawLineFull = -4,
    LiquidityPoolWithdrawUnderMinimum = -5,
}

// LiquidityPoolWithdrawResult is an XDR Union defines as:
//
//   union LiquidityPoolWithdrawResult switch (
//        LiquidityPoolWithdrawResultCode code)
//    {
//    case LIQUIDITY_POOL_WITHDRAW_SUCCESS:
//        void;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum LiquidityPoolWithdrawResult {
    // IDEN LIQUIDITY_POOL_WITHDRAW_SUCCESS
    #[discriminant(value = "0")]
    LiquidityPoolWithdrawSuccess(()),
    #[discriminant(value = "-1")]
    LiquidityPoolWithdrawMalformed(()),
    #[discriminant(value = "-2")]
    LiquidityPoolWithdrawNoTrust(()),
    #[discriminant(value = "-3")]
    LiquidityPoolWithdrawUnderfunded(()),
    #[discriminant(value = "-4")]
    LiquidityPoolWithdrawLineFull(()),
    #[discriminant(value = "-5")]
    LiquidityPoolWithdrawUnderMinimum(()),
}

// OperationResultCode is an XDR Enum defines as:
//
//   enum OperationResultCode
//    {
//        opINNER = 0, // inner object result is valid
//
//        opBAD_AUTH = -1,            // too few valid signatures / wrong network
//        opNO_ACCOUNT = -2,          // source account was not found
//        opNOT_SUPPORTED = -3,       // operation not supported at this time
//        opTOO_MANY_SUBENTRIES = -4, // max number of subentries already reached
//        opEXCEEDED_WORK_LIMIT = -5, // operation did too much work
//        opTOO_MANY_SPONSORING = -6  // account is sponsoring too many entries
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum OperationResultCode {
    OpInner = 0,
    OpBadAuth = -1,
    OpNoAccount = -2,
    OpNotSupported = -3,
    OpTooManySubentries = -4,
    OpExceededWorkLimit = -5,
    OpTooManySponsoring = -6,
}

// OperationResultTr is an XDR NestedUnion defines as:
//
//   union switch (OperationType type)
//        {
//        case CREATE_ACCOUNT:
//            CreateAccountResult createAccountResult;
//        case PAYMENT:
//            PaymentResult paymentResult;
//        case PATH_PAYMENT_STRICT_RECEIVE:
//            PathPaymentStrictReceiveResult pathPaymentStrictReceiveResult;
//        case MANAGE_SELL_OFFER:
//            ManageSellOfferResult manageSellOfferResult;
//        case CREATE_PASSIVE_SELL_OFFER:
//            ManageSellOfferResult createPassiveSellOfferResult;
//        case SET_OPTIONS:
//            SetOptionsResult setOptionsResult;
//        case CHANGE_TRUST:
//            ChangeTrustResult changeTrustResult;
//        case ALLOW_TRUST:
//            AllowTrustResult allowTrustResult;
//        case ACCOUNT_MERGE:
//            AccountMergeResult accountMergeResult;
//        case INFLATION:
//            InflationResult inflationResult;
//        case MANAGE_DATA:
//            ManageDataResult manageDataResult;
//        case BUMP_SEQUENCE:
//            BumpSequenceResult bumpSeqResult;
//        case MANAGE_BUY_OFFER:
//            ManageBuyOfferResult manageBuyOfferResult;
//        case PATH_PAYMENT_STRICT_SEND:
//            PathPaymentStrictSendResult pathPaymentStrictSendResult;
//        case CREATE_CLAIMABLE_BALANCE:
//            CreateClaimableBalanceResult createClaimableBalanceResult;
//        case CLAIM_CLAIMABLE_BALANCE:
//            ClaimClaimableBalanceResult claimClaimableBalanceResult;
//        case BEGIN_SPONSORING_FUTURE_RESERVES:
//            BeginSponsoringFutureReservesResult beginSponsoringFutureReservesResult;
//        case END_SPONSORING_FUTURE_RESERVES:
//            EndSponsoringFutureReservesResult endSponsoringFutureReservesResult;
//        case REVOKE_SPONSORSHIP:
//            RevokeSponsorshipResult revokeSponsorshipResult;
//        case CLAWBACK:
//            ClawbackResult clawbackResult;
//        case CLAWBACK_CLAIMABLE_BALANCE:
//            ClawbackClaimableBalanceResult clawbackClaimableBalanceResult;
//        case SET_TRUST_LINE_FLAGS:
//            SetTrustLineFlagsResult setTrustLineFlagsResult;
//        case LIQUIDITY_POOL_DEPOSIT:
//            LiquidityPoolDepositResult liquidityPoolDepositResult;
//        case LIQUIDITY_POOL_WITHDRAW:
//            LiquidityPoolWithdrawResult liquidityPoolWithdrawResult;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum OperationResultTr {
    // IDEN CREATE_ACCOUNT
    #[discriminant(value = "0")]
    CreateAccount(CreateAccountResult),
    // IDEN PAYMENT
    #[discriminant(value = "1")]
    Payment(PaymentResult),
    // IDEN PATH_PAYMENT_STRICT_RECEIVE
    #[discriminant(value = "2")]
    PathPaymentStrictReceive(PathPaymentStrictReceiveResult),
    // IDEN MANAGE_SELL_OFFER
    #[discriminant(value = "3")]
    ManageSellOffer(ManageSellOfferResult),
    // IDEN CREATE_PASSIVE_SELL_OFFER
    #[discriminant(value = "4")]
    CreatePassiveSellOffer(ManageSellOfferResult),
    // IDEN SET_OPTIONS
    #[discriminant(value = "5")]
    SetOptions(SetOptionsResult),
    // IDEN CHANGE_TRUST
    #[discriminant(value = "6")]
    ChangeTrust(ChangeTrustResult),
    // IDEN ALLOW_TRUST
    #[discriminant(value = "7")]
    AllowTrust(AllowTrustResult),
    // IDEN ACCOUNT_MERGE
    #[discriminant(value = "8")]
    AccountMerge(AccountMergeResult),
    // IDEN INFLATION
    #[discriminant(value = "9")]
    Inflation(InflationResult),
    // IDEN MANAGE_DATA
    #[discriminant(value = "10")]
    ManageData(ManageDataResult),
    // IDEN BUMP_SEQUENCE
    #[discriminant(value = "11")]
    BumpSequence(BumpSequenceResult),
    // IDEN MANAGE_BUY_OFFER
    #[discriminant(value = "12")]
    ManageBuyOffer(ManageBuyOfferResult),
    // IDEN PATH_PAYMENT_STRICT_SEND
    #[discriminant(value = "13")]
    PathPaymentStrictSend(PathPaymentStrictSendResult),
    // IDEN CREATE_CLAIMABLE_BALANCE
    #[discriminant(value = "14")]
    CreateClaimableBalance(CreateClaimableBalanceResult),
    // IDEN CLAIM_CLAIMABLE_BALANCE
    #[discriminant(value = "15")]
    ClaimClaimableBalance(ClaimClaimableBalanceResult),
    // IDEN BEGIN_SPONSORING_FUTURE_RESERVES
    #[discriminant(value = "16")]
    BeginSponsoringFutureReserves(BeginSponsoringFutureReservesResult),
    // IDEN END_SPONSORING_FUTURE_RESERVES
    #[discriminant(value = "17")]
    EndSponsoringFutureReserves(EndSponsoringFutureReservesResult),
    // IDEN REVOKE_SPONSORSHIP
    #[discriminant(value = "18")]
    RevokeSponsorship(RevokeSponsorshipResult),
    // IDEN CLAWBACK
    #[discriminant(value = "19")]
    Clawback(ClawbackResult),
    // IDEN CLAWBACK_CLAIMABLE_BALANCE
    #[discriminant(value = "20")]
    ClawbackClaimableBalance(ClawbackClaimableBalanceResult),
    // IDEN SET_TRUST_LINE_FLAGS
    #[discriminant(value = "21")]
    SetTrustLineFlags(SetTrustLineFlagsResult),
    // IDEN LIQUIDITY_POOL_DEPOSIT
    #[discriminant(value = "22")]
    LiquidityPoolDeposit(LiquidityPoolDepositResult),
    // IDEN LIQUIDITY_POOL_WITHDRAW
    #[discriminant(value = "23")]
    LiquidityPoolWithdraw(LiquidityPoolWithdrawResult),
}

// OperationResult is an XDR Union defines as:
//
//   union OperationResult switch (OperationResultCode code)
//    {
//    case opINNER:
//        union switch (OperationType type)
//        {
//        case CREATE_ACCOUNT:
//            CreateAccountResult createAccountResult;
//        case PAYMENT:
//            PaymentResult paymentResult;
//        case PATH_PAYMENT_STRICT_RECEIVE:
//            PathPaymentStrictReceiveResult pathPaymentStrictReceiveResult;
//        case MANAGE_SELL_OFFER:
//            ManageSellOfferResult manageSellOfferResult;
//        case CREATE_PASSIVE_SELL_OFFER:
//            ManageSellOfferResult createPassiveSellOfferResult;
//        case SET_OPTIONS:
//            SetOptionsResult setOptionsResult;
//        case CHANGE_TRUST:
//            ChangeTrustResult changeTrustResult;
//        case ALLOW_TRUST:
//            AllowTrustResult allowTrustResult;
//        case ACCOUNT_MERGE:
//            AccountMergeResult accountMergeResult;
//        case INFLATION:
//            InflationResult inflationResult;
//        case MANAGE_DATA:
//            ManageDataResult manageDataResult;
//        case BUMP_SEQUENCE:
//            BumpSequenceResult bumpSeqResult;
//        case MANAGE_BUY_OFFER:
//            ManageBuyOfferResult manageBuyOfferResult;
//        case PATH_PAYMENT_STRICT_SEND:
//            PathPaymentStrictSendResult pathPaymentStrictSendResult;
//        case CREATE_CLAIMABLE_BALANCE:
//            CreateClaimableBalanceResult createClaimableBalanceResult;
//        case CLAIM_CLAIMABLE_BALANCE:
//            ClaimClaimableBalanceResult claimClaimableBalanceResult;
//        case BEGIN_SPONSORING_FUTURE_RESERVES:
//            BeginSponsoringFutureReservesResult beginSponsoringFutureReservesResult;
//        case END_SPONSORING_FUTURE_RESERVES:
//            EndSponsoringFutureReservesResult endSponsoringFutureReservesResult;
//        case REVOKE_SPONSORSHIP:
//            RevokeSponsorshipResult revokeSponsorshipResult;
//        case CLAWBACK:
//            ClawbackResult clawbackResult;
//        case CLAWBACK_CLAIMABLE_BALANCE:
//            ClawbackClaimableBalanceResult clawbackClaimableBalanceResult;
//        case SET_TRUST_LINE_FLAGS:
//            SetTrustLineFlagsResult setTrustLineFlagsResult;
//        case LIQUIDITY_POOL_DEPOSIT:
//            LiquidityPoolDepositResult liquidityPoolDepositResult;
//        case LIQUIDITY_POOL_WITHDRAW:
//            LiquidityPoolWithdrawResult liquidityPoolWithdrawResult;
//        }
//        tr;
//    default:
//        void;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum OperationResult {
    // IDEN opINNER
    #[discriminant(value = "0")]
    OpInner(OperationResultTr),
    #[discriminant(value = "-1")]
    OpBadAuth(()),
    #[discriminant(value = "-2")]
    OpNoAccount(()),
    #[discriminant(value = "-3")]
    OpNotSupported(()),
    #[discriminant(value = "-4")]
    OpTooManySubentries(()),
    #[discriminant(value = "-5")]
    OpExceededWorkLimit(()),
    #[discriminant(value = "-6")]
    OpTooManySponsoring(()),
}

// TransactionResultCode is an XDR Enum defines as:
//
//   enum TransactionResultCode
//    {
//        txFEE_BUMP_INNER_SUCCESS = 1, // fee bump inner transaction succeeded
//        txSUCCESS = 0,                // all operations succeeded
//
//        txFAILED = -1, // one of the operations failed (none were applied)
//
//        txTOO_EARLY = -2,         // ledger closeTime before minTime
//        txTOO_LATE = -3,          // ledger closeTime after maxTime
//        txMISSING_OPERATION = -4, // no operation was specified
//        txBAD_SEQ = -5,           // sequence number does not match source account
//
//        txBAD_AUTH = -6,             // too few valid signatures / wrong network
//        txINSUFFICIENT_BALANCE = -7, // fee would bring account below reserve
//        txNO_ACCOUNT = -8,           // source account not found
//        txINSUFFICIENT_FEE = -9,     // fee is too small
//        txBAD_AUTH_EXTRA = -10,      // unused signatures attached to transaction
//        txINTERNAL_ERROR = -11,      // an unknown error occurred
//
//        txNOT_SUPPORTED = -12,         // transaction type not supported
//        txFEE_BUMP_INNER_FAILED = -13, // fee bump inner transaction failed
//        txBAD_SPONSORSHIP = -14        // sponsorship not confirmed
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum TransactionResultCode {
    TxFeeBumpInnerSuccess = 1,
    TxSuccess = 0,
    TxFailed = -1,
    TxTooEarly = -2,
    TxTooLate = -3,
    TxMissingOperation = -4,
    TxBadSeq = -5,
    TxBadAuth = -6,
    TxInsufficientBalance = -7,
    TxNoAccount = -8,
    TxInsufficientFee = -9,
    TxBadAuthExtra = -10,
    TxInternalError = -11,
    TxNotSupported = -12,
    TxFeeBumpInnerFailed = -13,
    TxBadSponsorship = -14,
}

// InnerTransactionResultResult is an XDR NestedUnion defines as:
//
//   union switch (TransactionResultCode code)
//        {
//        // txFEE_BUMP_INNER_SUCCESS is not included
//        case txSUCCESS:
//        case txFAILED:
//            OperationResult results<>;
//        case txTOO_EARLY:
//        case txTOO_LATE:
//        case txMISSING_OPERATION:
//        case txBAD_SEQ:
//        case txBAD_AUTH:
//        case txINSUFFICIENT_BALANCE:
//        case txNO_ACCOUNT:
//        case txINSUFFICIENT_FEE:
//        case txBAD_AUTH_EXTRA:
//        case txINTERNAL_ERROR:
//        case txNOT_SUPPORTED:
//        // txFEE_BUMP_INNER_FAILED is not included
//        case txBAD_SPONSORSHIP:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum InnerTransactionResultResult {
    #[discriminant(value = "1")]
    TxFeeBumpInnerSuccess(()),
    // IDEN txSUCCESS
    #[discriminant(value = "0")]
    TxSuccess(Vec<OperationResult>),
    // IDEN txFAILED
    #[discriminant(value = "-1")]
    TxFailed(Vec<OperationResult>),
    // IDEN txTOO_EARLY
    #[discriminant(value = "-2")]
    TxTooEarly(()),
    // IDEN txTOO_LATE
    #[discriminant(value = "-3")]
    TxTooLate(()),
    // IDEN txMISSING_OPERATION
    #[discriminant(value = "-4")]
    TxMissingOperation(()),
    // IDEN txBAD_SEQ
    #[discriminant(value = "-5")]
    TxBadSeq(()),
    // IDEN txBAD_AUTH
    #[discriminant(value = "-6")]
    TxBadAuth(()),
    // IDEN txINSUFFICIENT_BALANCE
    #[discriminant(value = "-7")]
    TxInsufficientBalance(()),
    // IDEN txNO_ACCOUNT
    #[discriminant(value = "-8")]
    TxNoAccount(()),
    // IDEN txINSUFFICIENT_FEE
    #[discriminant(value = "-9")]
    TxInsufficientFee(()),
    // IDEN txBAD_AUTH_EXTRA
    #[discriminant(value = "-10")]
    TxBadAuthExtra(()),
    // IDEN txINTERNAL_ERROR
    #[discriminant(value = "-11")]
    TxInternalError(()),
    // IDEN txNOT_SUPPORTED
    #[discriminant(value = "-12")]
    TxNotSupported(()),
    #[discriminant(value = "-13")]
    TxFeeBumpInnerFailed(()),
    // IDEN txBAD_SPONSORSHIP
    #[discriminant(value = "-14")]
    TxBadSponsorship(()),
}

// InnerTransactionResultExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum InnerTransactionResultExt {
    // NO IDEN 0
    V0(()),
}

// InnerTransactionResult is an XDR Struct defines as:
//
//   struct InnerTransactionResult
//    {
//        // Always 0. Here for binary compatibility.
//        int64 feeCharged;
//
//        union switch (TransactionResultCode code)
//        {
//        // txFEE_BUMP_INNER_SUCCESS is not included
//        case txSUCCESS:
//        case txFAILED:
//            OperationResult results<>;
//        case txTOO_EARLY:
//        case txTOO_LATE:
//        case txMISSING_OPERATION:
//        case txBAD_SEQ:
//        case txBAD_AUTH:
//        case txINSUFFICIENT_BALANCE:
//        case txNO_ACCOUNT:
//        case txINSUFFICIENT_FEE:
//        case txBAD_AUTH_EXTRA:
//        case txINTERNAL_ERROR:
//        case txNOT_SUPPORTED:
//        // txFEE_BUMP_INNER_FAILED is not included
//        case txBAD_SPONSORSHIP:
//            void;
//        }
//        result;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct InnerTransactionResult {
    // TODO
    pub fee_charged: Int64,
    // TODO
    pub result: InnerTransactionResultResult,
    // TODO
    pub ext: InnerTransactionResultExt,
}

// InnerTransactionResultPair is an XDR Struct defines as:
//
//   struct InnerTransactionResultPair
//    {
//        Hash transactionHash;          // hash of the inner transaction
//        InnerTransactionResult result; // result for the inner transaction
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct InnerTransactionResultPair {
    // TODO
    pub transaction_hash: Hash,
    // TODO
    pub result: InnerTransactionResult,
}

// TransactionResultResult is an XDR NestedUnion defines as:
//
//   union switch (TransactionResultCode code)
//        {
//        case txFEE_BUMP_INNER_SUCCESS:
//        case txFEE_BUMP_INNER_FAILED:
//            InnerTransactionResultPair innerResultPair;
//        case txSUCCESS:
//        case txFAILED:
//            OperationResult results<>;
//        default:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum TransactionResultResult {
    // IDEN txFEE_BUMP_INNER_SUCCESS
    #[discriminant(value = "1")]
    TxFeeBumpInnerSuccess(InnerTransactionResultPair),
    // IDEN txSUCCESS
    #[discriminant(value = "0")]
    TxSuccess(Vec<OperationResult>),
    // IDEN txFAILED
    #[discriminant(value = "-1")]
    TxFailed(Vec<OperationResult>),
    #[discriminant(value = "-2")]
    TxTooEarly(()),
    #[discriminant(value = "-3")]
    TxTooLate(()),
    #[discriminant(value = "-4")]
    TxMissingOperation(()),
    #[discriminant(value = "-5")]
    TxBadSeq(()),
    #[discriminant(value = "-6")]
    TxBadAuth(()),
    #[discriminant(value = "-7")]
    TxInsufficientBalance(()),
    #[discriminant(value = "-8")]
    TxNoAccount(()),
    #[discriminant(value = "-9")]
    TxInsufficientFee(()),
    #[discriminant(value = "-10")]
    TxBadAuthExtra(()),
    #[discriminant(value = "-11")]
    TxInternalError(()),
    #[discriminant(value = "-12")]
    TxNotSupported(()),
    #[discriminant(value = "-13")]
    // IDEN txFEE_BUMP_INNER_FAILED
    TxFeeBumpInnerFailed(InnerTransactionResultPair),
    #[discriminant(value = "-14")]
    TxBadSponsorship(()),
}

// TransactionResultExt is an XDR NestedUnion defines as:
//
//   union switch (int v)
//        {
//        case 0:
//            void;
//        }
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum TransactionResultExt {
    // NO IDEN 0
    V0(()),
}

// TransactionResult is an XDR Struct defines as:
//
//   struct TransactionResult
//    {
//        int64 feeCharged; // actual fee charged for the transaction
//
//        union switch (TransactionResultCode code)
//        {
//        case txFEE_BUMP_INNER_SUCCESS:
//        case txFEE_BUMP_INNER_FAILED:
//            InnerTransactionResultPair innerResultPair;
//        case txSUCCESS:
//        case txFAILED:
//            OperationResult results<>;
//        default:
//            void;
//        }
//        result;
//
//        // reserved for future use
//        union switch (int v)
//        {
//        case 0:
//            void;
//        }
//        ext;
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct TransactionResult {
    // TODO
    pub fee_charged: Int64,
    // TODO
    pub result: TransactionResultResult,
    // TODO
    pub ext: TransactionResultExt,
}

// Hash is an XDR Typedef defines as:
//
//   typedef opaque Hash[32];
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Hash {
    #[array(fixed = 32)]
    pub value: Vec<u8>,
}

impl Hash {
    pub fn new(value: Vec<u8>) -> Hash {
        Hash { value }
    }
}

// Uint256 is an XDR Typedef defines as:
//
//   typedef opaque uint256[32];
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Uint256 {
    #[array(fixed = 32)]
    pub value: Vec<u8>,
}

impl Uint256 {
    pub fn new(value: Vec<u8>) -> Uint256 {
        Uint256 { value }
    }
}

// Uint32 is an XDR Typedef defines as:
//
//   typedef unsigned int uint32;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Uint32 {
    pub value: u32,
}

impl Uint32 {
    pub fn new(value: u32) -> Uint32 {
        Uint32 { value }
    }
}

// Int32 is an XDR Typedef defines as:
//
//   typedef int int32;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Int32 {
    pub value: i32,
}

impl Int32 {
    pub fn new(value: i32) -> Int32 {
        Int32 { value }
    }
}

// Uint64 is an XDR Typedef defines as:
//
//   typedef unsigned hyper uint64;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Uint64 {
    pub value: u64,
}

impl Uint64 {
    pub fn new(value: u64) -> Uint64 {
        Uint64 { value }
    }
}

// Int64 is an XDR Typedef defines as:
//
//   typedef hyper int64;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Int64 {
    pub value: i64,
}

impl Int64 {
    pub fn new(value: i64) -> Int64 {
        Int64 { value }
    }
}

// CryptoKeyType is an XDR Enum defines as:
//
//   enum CryptoKeyType
//    {
//        KEY_TYPE_ED25519 = 0,
//        KEY_TYPE_PRE_AUTH_TX = 1,
//        KEY_TYPE_HASH_X = 2,
//        // MUXED enum values for supported type are derived from the enum values
//        // above by ORing them with 0x100
//        KEY_TYPE_MUXED_ED25519 = 0x100
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum CryptoKeyType {
    KeyTypeEd25519 = 0,
    KeyTypePreAuthTx = 1,
    KeyTypeHashX = 2,
    KeyTypeMuxedEd25519 = 256,
}

// PublicKeyType is an XDR Enum defines as:
//
//   enum PublicKeyType
//    {
//        PUBLIC_KEY_TYPE_ED25519 = KEY_TYPE_ED25519
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum PublicKeyType {
    PublicKeyTypeEd25519 = 0,
}

// SignerKeyType is an XDR Enum defines as:
//
//   enum SignerKeyType
//    {
//        SIGNER_KEY_TYPE_ED25519 = KEY_TYPE_ED25519,
//        SIGNER_KEY_TYPE_PRE_AUTH_TX = KEY_TYPE_PRE_AUTH_TX,
//        SIGNER_KEY_TYPE_HASH_X = KEY_TYPE_HASH_X
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub enum SignerKeyType {
    SignerKeyTypeEd25519 = 0,
    SignerKeyTypePreAuthTx = 1,
    SignerKeyTypeHashX = 2,
}

// PublicKey is an XDR Union defines as:
//
//   union PublicKey switch (PublicKeyType type)
//    {
//    case PUBLIC_KEY_TYPE_ED25519:
//        uint256 ed25519;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum PublicKey {
    // IDEN PUBLIC_KEY_TYPE_ED25519
    #[discriminant(value = "0")]
    PublicKeyTypeEd25519(Uint256),
}

// SignerKey is an XDR Union defines as:
//
//   union SignerKey switch (SignerKeyType type)
//    {
//    case SIGNER_KEY_TYPE_ED25519:
//        uint256 ed25519;
//    case SIGNER_KEY_TYPE_PRE_AUTH_TX:
//        /* SHA-256 Hash of TransactionSignaturePayload structure */
//        uint256 preAuthTx;
//    case SIGNER_KEY_TYPE_HASH_X:
//        /* Hash of random 256 bit preimage X */
//        uint256 hashX;
//    };
//
// union
#[derive(Debug, XDROut, XDRIn)]
pub enum SignerKey {
    // IDEN SIGNER_KEY_TYPE_ED25519
    #[discriminant(value = "0")]
    SignerKeyTypeEd25519(Uint256),
    // IDEN SIGNER_KEY_TYPE_PRE_AUTH_TX
    SignerKeyTypePreAuthTx(Uint256),
    // IDEN SIGNER_KEY_TYPE_HASH_X
    SignerKeyTypeHashX(Uint256),
}

// Signature is an XDR Typedef defines as:
//
//   typedef opaque Signature<64>;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Signature {
    #[array(var = 64)]
    pub value: Vec<u8>,
}

impl Signature {
    pub fn new(value: Vec<u8>) -> Signature {
        Signature { value }
    }
}

// SignatureHint is an XDR Typedef defines as:
//
//   typedef opaque SignatureHint[4];
//
#[derive(Debug, XDROut, XDRIn)]
pub struct SignatureHint {
    #[array(fixed = 4)]
    pub value: Vec<u8>,
}

impl SignatureHint {
    pub fn new(value: Vec<u8>) -> SignatureHint {
        SignatureHint { value }
    }
}

// NodeId is an XDR Typedef defines as:
//
//   typedef PublicKey NodeID;
//
#[derive(Debug, XDROut, XDRIn)]
pub struct NodeId {
    // TODO
    pub value: PublicKey,
}

impl NodeId {
    pub fn new(value: PublicKey) -> NodeId {
        NodeId { value }
    }
}

// Curve25519Secret is an XDR Struct defines as:
//
//   struct Curve25519Secret
//    {
//        opaque key[32];
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Curve25519Secret {
    #[array(fixed = 32)]
    pub key: Vec<u8>,
}

// Curve25519Public is an XDR Struct defines as:
//
//   struct Curve25519Public
//    {
//        opaque key[32];
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct Curve25519Public {
    #[array(fixed = 32)]
    pub key: Vec<u8>,
}

// HmacSha256Key is an XDR Struct defines as:
//
//   struct HmacSha256Key
//    {
//        opaque key[32];
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct HmacSha256Key {
    #[array(fixed = 32)]
    pub key: Vec<u8>,
}

// HmacSha256Mac is an XDR Struct defines as:
//
//   struct HmacSha256Mac
//    {
//        opaque mac[32];
//    };
//
#[derive(Debug, XDROut, XDRIn)]
pub struct HmacSha256Mac {
    #[array(fixed = 32)]
    pub mac: Vec<u8>,
}
