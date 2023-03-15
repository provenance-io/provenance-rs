/// AccessGrant associates a collection of permissions with an address for delegated marker account control.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccessGrant {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration = "Access", repeated, packed = "false", tag = "2")]
    pub permissions: ::prost::alloc::vec::Vec<i32>,
}
/// Access defines the different types of permissions that a marker supports granting to an address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Access {
    /// ACCESS_UNSPECIFIED defines a no-op vote option.
    Unspecified = 0,
    /// ACCESS_MINT is the ability to increase the supply of a marker
    Mint = 1,
    /// ACCESS_BURN is the ability to decrease the supply of the marker using coin held by the marker.
    Burn = 2,
    /// ACCESS_DEPOSIT is the ability to set a marker reference to this marker in the metadata/scopes module
    Deposit = 3,
    /// ACCESS_WITHDRAW is the ability to remove marker references to this marker in from metadata/scopes or
    /// transfer coin from this marker account to another account.
    Withdraw = 4,
    /// ACCESS_DELETE is the ability to move a proposed, finalized or active marker into the cancelled state. This
    /// access also allows cancelled markers to be marked for deletion
    Delete = 5,
    /// ACCESS_ADMIN is the ability to add access grants for accounts to the list of marker permissions.
    Admin = 6,
    /// ACCESS_TRANSFER is the ability to invoke a send operation using the marker module to facilitate exchange.
    /// This access right is only supported on RESTRICTED markers.
    Transfer = 7,
}
impl Access {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Access::Unspecified => "ACCESS_UNSPECIFIED",
            Access::Mint => "ACCESS_MINT",
            Access::Burn => "ACCESS_BURN",
            Access::Deposit => "ACCESS_DEPOSIT",
            Access::Withdraw => "ACCESS_WITHDRAW",
            Access::Delete => "ACCESS_DELETE",
            Access::Admin => "ACCESS_ADMIN",
            Access::Transfer => "ACCESS_TRANSFER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCESS_UNSPECIFIED" => Some(Self::Unspecified),
            "ACCESS_MINT" => Some(Self::Mint),
            "ACCESS_BURN" => Some(Self::Burn),
            "ACCESS_DEPOSIT" => Some(Self::Deposit),
            "ACCESS_WITHDRAW" => Some(Self::Withdraw),
            "ACCESS_DELETE" => Some(Self::Delete),
            "ACCESS_ADMIN" => Some(Self::Admin),
            "ACCESS_TRANSFER" => Some(Self::Transfer),
            _ => None,
        }
    }
}
/// Params defines the set of params for the account module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// maximum amount of supply to allow a marker to be created with
    #[prost(uint64, tag = "1")]
    pub max_total_supply: u64,
    /// indicates if governance based controls of markers is allowed.
    #[prost(bool, tag = "2")]
    pub enable_governance: bool,
    /// a regular expression used to validate marker denom values from normal create requests (governance
    /// requests are only subject to platform coin validation denom expression)
    #[prost(string, tag = "3")]
    pub unrestricted_denom_regex: ::prost::alloc::string::String,
}
/// MarkerAccount holds the marker configuration information in addition to a base account structure.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkerAccount {
    /// base cosmos account information including address and coin holdings.
    #[prost(message, optional, tag = "1")]
    pub base_account: ::core::option::Option<cosmos_sdk_proto::cosmos::auth::v1beta1::BaseAccount>,
    /// Address that owns the marker configuration.  This account must sign any requests
    /// to change marker config (only valid for statuses prior to finalization)
    #[prost(string, tag = "2")]
    pub manager: ::prost::alloc::string::String,
    /// Access control lists
    #[prost(message, repeated, tag = "3")]
    pub access_control: ::prost::alloc::vec::Vec<AccessGrant>,
    /// Indicates the current status of this marker record.
    #[prost(enumeration = "MarkerStatus", tag = "4")]
    pub status: i32,
    /// value denomination and total supply for the token.
    #[prost(string, tag = "5")]
    pub denom: ::prost::alloc::string::String,
    /// the total supply expected for a marker.  This is the amount that is minted when a marker is created.
    #[prost(string, tag = "6")]
    pub supply: ::prost::alloc::string::String,
    /// Marker type information
    #[prost(enumeration = "MarkerType", tag = "7")]
    pub marker_type: i32,
    /// A fixed supply will mint additional coin automatically if the total supply decreases below a set value.  This
    /// may occur if the coin is burned or an account holding the coin is slashed. (default: true)
    #[prost(bool, tag = "8")]
    pub supply_fixed: bool,
    /// indicates that governance based control is allowed for this marker
    #[prost(bool, tag = "9")]
    pub allow_governance_control: bool,
}
/// EventMarkerAdd event emitted when marker is added
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarkerAdd {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub manager: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub marker_type: ::prost::alloc::string::String,
}
/// EventMarkerAddAccess event emitted when marker access is added
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarkerAddAccess {
    #[prost(message, optional, tag = "1")]
    pub access: ::core::option::Option<EventMarkerAccess>,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerAccess event access permissions for address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarkerAccess {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventMarkerDeleteAccess event emitted when marker access is revoked
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarkerDeleteAccess {
    #[prost(string, tag = "1")]
    pub remove_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerFinalize event emitted when marker is finalized
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarkerFinalize {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerActivate event emitted when marker is activated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarkerActivate {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerCancel event emitted when marker is cancelled
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarkerCancel {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerDelete event emitted when marker is deleted
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarkerDelete {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerMint event emitted when additional marker supply is minted
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarkerMint {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerBurn event emitted when coin is burned from marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarkerBurn {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
}
/// EventMarkerWithdraw event emitted when coins are withdrew from marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarkerWithdraw {
    #[prost(string, tag = "1")]
    pub coins: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub to_address: ::prost::alloc::string::String,
}
/// EventMarkerTransfer event emitted when coins are transfered to from account to another
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarkerTransfer {
    #[prost(string, tag = "1")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub from_address: ::prost::alloc::string::String,
}
/// EventMarkerSetDenomMetadata event emitted when metadata is set on marker with denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMarkerSetDenomMetadata {
    #[prost(string, tag = "1")]
    pub metadata_base: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub metadata_description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub metadata_display: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub metadata_denom_units: ::prost::alloc::vec::Vec<EventDenomUnit>,
    #[prost(string, tag = "5")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub metadata_name: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub metadata_symbol: ::prost::alloc::string::String,
}
/// EventDenomUnit denom units for set denom metadata event
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventDenomUnit {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub exponent: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub aliases: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MarkerType defines the types of marker
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MarkerType {
    /// MARKER_TYPE_UNSPECIFIED is an invalid/unknown marker type.
    Unspecified = 0,
    /// MARKER_TYPE_COIN is a marker that represents a standard fungible coin (default).
    Coin = 1,
    /// MARKER_TYPE_RESTRICTED is a marker that represents a denom with send_enabled = false.
    Restricted = 2,
}
impl MarkerType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MarkerType::Unspecified => "MARKER_TYPE_UNSPECIFIED",
            MarkerType::Coin => "MARKER_TYPE_COIN",
            MarkerType::Restricted => "MARKER_TYPE_RESTRICTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MARKER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "MARKER_TYPE_COIN" => Some(Self::Coin),
            "MARKER_TYPE_RESTRICTED" => Some(Self::Restricted),
            _ => None,
        }
    }
}
/// MarkerStatus defines the various states a marker account can be in.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum MarkerStatus {
    /// MARKER_STATUS_UNSPECIFIED - Unknown/Invalid Marker Status
    Unspecified = 0,
    /// MARKER_STATUS_PROPOSED - Initial configuration period, updates allowed, token supply not created.
    Proposed = 1,
    /// MARKER_STATUS_FINALIZED - Configuration finalized, ready for supply creation
    Finalized = 2,
    /// MARKER_STATUS_ACTIVE - Supply is created, rules are in force.
    Active = 3,
    /// MARKER_STATUS_CANCELLED - Marker has been cancelled, pending destroy
    Cancelled = 4,
    /// MARKER_STATUS_DESTROYED - Marker supply has all been recalled, marker is considered destroyed and no further
    /// actions allowed.
    Destroyed = 5,
}
impl MarkerStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            MarkerStatus::Unspecified => "MARKER_STATUS_UNSPECIFIED",
            MarkerStatus::Proposed => "MARKER_STATUS_PROPOSED",
            MarkerStatus::Finalized => "MARKER_STATUS_FINALIZED",
            MarkerStatus::Active => "MARKER_STATUS_ACTIVE",
            MarkerStatus::Cancelled => "MARKER_STATUS_CANCELLED",
            MarkerStatus::Destroyed => "MARKER_STATUS_DESTROYED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "MARKER_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "MARKER_STATUS_PROPOSED" => Some(Self::Proposed),
            "MARKER_STATUS_FINALIZED" => Some(Self::Finalized),
            "MARKER_STATUS_ACTIVE" => Some(Self::Active),
            "MARKER_STATUS_CANCELLED" => Some(Self::Cancelled),
            "MARKER_STATUS_DESTROYED" => Some(Self::Destroyed),
            _ => None,
        }
    }
}
/// MsgGrantAllowanceRequest validates permission to create a fee grant based on marker admin access. If
/// successful a feegrant is recorded where the marker account itself is the grantor
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrantAllowanceRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
    /// grantee is the address of the user being granted an allowance of another user's funds.
    #[prost(string, tag = "3")]
    pub grantee: ::prost::alloc::string::String,
    /// allowance can be any of basic and filtered fee allowance (fee FeeGrant module).
    #[prost(message, optional, tag = "4")]
    pub allowance: ::core::option::Option<::prost_types::Any>,
}
/// MsgGrantAllowanceResponse defines the Msg/GrantAllowanceResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrantAllowanceResponse {}
/// MsgAddMarkerRequest defines the Msg/AddMarker request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddMarkerRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub manager: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(enumeration = "MarkerStatus", tag = "5")]
    pub status: i32,
    #[prost(enumeration = "MarkerType", tag = "6")]
    pub marker_type: i32,
    #[prost(message, repeated, tag = "7")]
    pub access_list: ::prost::alloc::vec::Vec<AccessGrant>,
    #[prost(bool, tag = "8")]
    pub supply_fixed: bool,
    #[prost(bool, tag = "9")]
    pub allow_governance_control: bool,
}
/// MsgAddMarkerResponse defines the Msg/AddMarker response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddMarkerResponse {}
/// MsgAddAccessRequest defines the Msg/AddAccess request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddAccessRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub access: ::prost::alloc::vec::Vec<AccessGrant>,
}
/// MsgAddAccessResponse defines the Msg/AddAccess response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddAccessResponse {}
/// MsgDeleteAccessRequest defines the Msg/DeleteAccess request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAccessRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub removed_address: ::prost::alloc::string::String,
}
/// MsgDeleteAccessResponse defines the Msg/DeleteAccess response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteAccessResponse {}
/// MsgFinalizeRequest defines the Msg/Finalize request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFinalizeRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgFinalizeResponse defines the Msg/Finalize response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFinalizeResponse {}
/// MsgActivateRequest defines the Msg/Activate request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgActivateRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgActivateResponse defines the Msg/Activate response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgActivateResponse {}
/// MsgCancelRequest defines the Msg/Cancel request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgCancelResponse defines the Msg/Cancel response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelResponse {}
/// MsgDeleteRequest defines the Msg/Delete request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgDeleteResponse defines the Msg/Delete response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteResponse {}
/// MsgMintRequest defines the Msg/Mint request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgMintResponse defines the Msg/Mint response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgMintResponse {}
/// MsgBurnRequest defines the Msg/Burn request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgBurnResponse defines the Msg/Burn response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBurnResponse {}
/// MsgWithdrawRequest defines the Msg/Withdraw request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// MsgWithdrawResponse defines the Msg/Withdraw response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawResponse {}
/// MsgTransferRequest defines the Msg/Transfer request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub administrator: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub to_address: ::prost::alloc::string::String,
}
/// MsgTransferResponse defines the Msg/Transfer response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferResponse {}
/// MsgIbcTransferRequest defines the Msg/IbcTransfer request type for markers.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIbcTransferRequest {
    #[prost(message, optional, tag = "1")]
    pub transfer:
        ::core::option::Option<cosmos_sdk_proto::ibc::applications::transfer::v1::MsgTransfer>,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgIbcTransferResponse defines the Msg/IbcTransfer response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgIbcTransferResponse {}
/// MsgSetDenomMetadataRequest defines the Msg/SetDenomMetadata request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetDenomMetadataRequest {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<cosmos_sdk_proto::cosmos::bank::v1beta1::Metadata>,
    #[prost(string, tag = "2")]
    pub administrator: ::prost::alloc::string::String,
}
/// MsgSetDenomMetadataResponse defines the Msg/SetDenomMetadata response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetDenomMetadataResponse {}
/// MsgAddFinalizeActivateMarkerRequest defines the Msg/AddFinalizeActivateMarker request type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddFinalizeActivateMarkerRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub manager: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(enumeration = "MarkerType", tag = "5")]
    pub marker_type: i32,
    #[prost(message, repeated, tag = "6")]
    pub access_list: ::prost::alloc::vec::Vec<AccessGrant>,
    #[prost(bool, tag = "7")]
    pub supply_fixed: bool,
    #[prost(bool, tag = "8")]
    pub allow_governance_control: bool,
}
/// MsgAddFinalizeActivateMarkerResponse defines the Msg/AddFinalizeActivateMarker response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddFinalizeActivateMarkerResponse {}
/// MsgSupplyIncreaseProposalRequest defines a governance proposal to administer a marker and increase total supply of the marker
/// through minting coin and placing it within the marker or assigning it directly to an account
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSupplyIncreaseProposalRequest {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// an optional target address for the minted coin from this request
    #[prost(string, tag = "2")]
    pub target_address: ::prost::alloc::string::String,
    /// signer of the proposal
    #[prost(string, tag = "3")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgSupplyIncreaseProposalResponse defines the Msg/SupplyIncreaseProposal response type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSupplyIncreaseProposalResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the Marker Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl MsgClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MsgClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            MsgClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Finalize
        pub async fn finalize(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgFinalizeRequest>,
        ) -> Result<tonic::Response<super::MsgFinalizeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Finalize");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Activate
        pub async fn activate(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgActivateRequest>,
        ) -> Result<tonic::Response<super::MsgActivateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Activate");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Cancel
        pub async fn cancel(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelRequest>,
        ) -> Result<tonic::Response<super::MsgCancelResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Cancel");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Delete
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Delete");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Mint
        pub async fn mint(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgMintRequest>,
        ) -> Result<tonic::Response<super::MsgMintResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Mint");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Burn
        pub async fn burn(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBurnRequest>,
        ) -> Result<tonic::Response<super::MsgBurnResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Burn");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AddAccess
        pub async fn add_access(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddAccessRequest>,
        ) -> Result<tonic::Response<super::MsgAddAccessResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/AddAccess");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteAccess
        pub async fn delete_access(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteAccessRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteAccessResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/DeleteAccess");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Withdraw
        pub async fn withdraw(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawRequest>,
        ) -> Result<tonic::Response<super::MsgWithdrawResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Withdraw");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AddMarker
        pub async fn add_marker(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddMarkerRequest>,
        ) -> Result<tonic::Response<super::MsgAddMarkerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/AddMarker");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Transfer marker denominated coin between accounts
        pub async fn transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgTransferRequest>,
        ) -> Result<tonic::Response<super::MsgTransferResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/Transfer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Transfer over ibc any marker(including restricted markers) between ibc accounts.
        /// The relayer is still needed to accomplish ibc middleware relays.
        pub async fn ibc_transfer(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgIbcTransferRequest>,
        ) -> Result<tonic::Response<super::MsgIbcTransferResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/IbcTransfer");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Allows Denom Metadata (see bank module) to be set for the Marker's Denom
        pub async fn set_denom_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetDenomMetadataRequest>,
        ) -> Result<tonic::Response<super::MsgSetDenomMetadataResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/SetDenomMetadata");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// GrantAllowance grants fee allowance to the grantee on the granter's
        /// account with the provided expiration time.
        pub async fn grant_allowance(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGrantAllowanceRequest>,
        ) -> Result<tonic::Response<super::MsgGrantAllowanceResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Msg/GrantAllowance");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AddFinalizeActivateMarker
        pub async fn add_finalize_activate_marker(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddFinalizeActivateMarkerRequest>,
        ) -> Result<tonic::Response<super::MsgAddFinalizeActivateMarkerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.marker.v1.Msg/AddFinalizeActivateMarker",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// SupplyIncreaseProposal can only be called via gov proposal
        pub async fn supply_increase_proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSupplyIncreaseProposalRequest>,
        ) -> Result<tonic::Response<super::MsgSupplyIncreaseProposalResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.marker.v1.Msg/SupplyIncreaseProposal",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        /// Finalize
        async fn finalize(
            &self,
            request: tonic::Request<super::MsgFinalizeRequest>,
        ) -> Result<tonic::Response<super::MsgFinalizeResponse>, tonic::Status>;
        /// Activate
        async fn activate(
            &self,
            request: tonic::Request<super::MsgActivateRequest>,
        ) -> Result<tonic::Response<super::MsgActivateResponse>, tonic::Status>;
        /// Cancel
        async fn cancel(
            &self,
            request: tonic::Request<super::MsgCancelRequest>,
        ) -> Result<tonic::Response<super::MsgCancelResponse>, tonic::Status>;
        /// Delete
        async fn delete(
            &self,
            request: tonic::Request<super::MsgDeleteRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteResponse>, tonic::Status>;
        /// Mint
        async fn mint(
            &self,
            request: tonic::Request<super::MsgMintRequest>,
        ) -> Result<tonic::Response<super::MsgMintResponse>, tonic::Status>;
        /// Burn
        async fn burn(
            &self,
            request: tonic::Request<super::MsgBurnRequest>,
        ) -> Result<tonic::Response<super::MsgBurnResponse>, tonic::Status>;
        /// AddAccess
        async fn add_access(
            &self,
            request: tonic::Request<super::MsgAddAccessRequest>,
        ) -> Result<tonic::Response<super::MsgAddAccessResponse>, tonic::Status>;
        /// DeleteAccess
        async fn delete_access(
            &self,
            request: tonic::Request<super::MsgDeleteAccessRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteAccessResponse>, tonic::Status>;
        /// Withdraw
        async fn withdraw(
            &self,
            request: tonic::Request<super::MsgWithdrawRequest>,
        ) -> Result<tonic::Response<super::MsgWithdrawResponse>, tonic::Status>;
        /// AddMarker
        async fn add_marker(
            &self,
            request: tonic::Request<super::MsgAddMarkerRequest>,
        ) -> Result<tonic::Response<super::MsgAddMarkerResponse>, tonic::Status>;
        /// Transfer marker denominated coin between accounts
        async fn transfer(
            &self,
            request: tonic::Request<super::MsgTransferRequest>,
        ) -> Result<tonic::Response<super::MsgTransferResponse>, tonic::Status>;
        /// Transfer over ibc any marker(including restricted markers) between ibc accounts.
        /// The relayer is still needed to accomplish ibc middleware relays.
        async fn ibc_transfer(
            &self,
            request: tonic::Request<super::MsgIbcTransferRequest>,
        ) -> Result<tonic::Response<super::MsgIbcTransferResponse>, tonic::Status>;
        /// Allows Denom Metadata (see bank module) to be set for the Marker's Denom
        async fn set_denom_metadata(
            &self,
            request: tonic::Request<super::MsgSetDenomMetadataRequest>,
        ) -> Result<tonic::Response<super::MsgSetDenomMetadataResponse>, tonic::Status>;
        /// GrantAllowance grants fee allowance to the grantee on the granter's
        /// account with the provided expiration time.
        async fn grant_allowance(
            &self,
            request: tonic::Request<super::MsgGrantAllowanceRequest>,
        ) -> Result<tonic::Response<super::MsgGrantAllowanceResponse>, tonic::Status>;
        /// AddFinalizeActivateMarker
        async fn add_finalize_activate_marker(
            &self,
            request: tonic::Request<super::MsgAddFinalizeActivateMarkerRequest>,
        ) -> Result<tonic::Response<super::MsgAddFinalizeActivateMarkerResponse>, tonic::Status>;
        /// SupplyIncreaseProposal can only be called via gov proposal
        async fn supply_increase_proposal(
            &self,
            request: tonic::Request<super::MsgSupplyIncreaseProposalRequest>,
        ) -> Result<tonic::Response<super::MsgSupplyIncreaseProposalResponse>, tonic::Status>;
    }
    /// Msg defines the Marker Msg service.
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/provenance.marker.v1.Msg/Finalize" => {
                    #[allow(non_camel_case_types)]
                    struct FinalizeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgFinalizeRequest> for FinalizeSvc<T> {
                        type Response = super::MsgFinalizeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgFinalizeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).finalize(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = FinalizeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Activate" => {
                    #[allow(non_camel_case_types)]
                    struct ActivateSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgActivateRequest> for ActivateSvc<T> {
                        type Response = super::MsgActivateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgActivateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).activate(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ActivateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Cancel" => {
                    #[allow(non_camel_case_types)]
                    struct CancelSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCancelRequest> for CancelSvc<T> {
                        type Response = super::MsgCancelResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCancelRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).cancel(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CancelSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Delete" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteRequest> for DeleteSvc<T> {
                        type Response = super::MsgDeleteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Mint" => {
                    #[allow(non_camel_case_types)]
                    struct MintSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgMintRequest> for MintSvc<T> {
                        type Response = super::MsgMintResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgMintRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).mint(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MintSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Burn" => {
                    #[allow(non_camel_case_types)]
                    struct BurnSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBurnRequest> for BurnSvc<T> {
                        type Response = super::MsgBurnResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBurnRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).burn(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BurnSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/AddAccess" => {
                    #[allow(non_camel_case_types)]
                    struct AddAccessSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAddAccessRequest> for AddAccessSvc<T> {
                        type Response = super::MsgAddAccessResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddAccessRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_access(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddAccessSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/DeleteAccess" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAccessSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteAccessRequest> for DeleteAccessSvc<T> {
                        type Response = super::MsgDeleteAccessResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteAccessRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_access(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteAccessSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Withdraw" => {
                    #[allow(non_camel_case_types)]
                    struct WithdrawSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWithdrawRequest> for WithdrawSvc<T> {
                        type Response = super::MsgWithdrawResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWithdrawRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).withdraw(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WithdrawSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/AddMarker" => {
                    #[allow(non_camel_case_types)]
                    struct AddMarkerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAddMarkerRequest> for AddMarkerSvc<T> {
                        type Response = super::MsgAddMarkerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddMarkerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_marker(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddMarkerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/Transfer" => {
                    #[allow(non_camel_case_types)]
                    struct TransferSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgTransferRequest> for TransferSvc<T> {
                        type Response = super::MsgTransferResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgTransferRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).transfer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TransferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/IbcTransfer" => {
                    #[allow(non_camel_case_types)]
                    struct IbcTransferSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgIbcTransferRequest> for IbcTransferSvc<T> {
                        type Response = super::MsgIbcTransferResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgIbcTransferRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).ibc_transfer(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = IbcTransferSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/SetDenomMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct SetDenomMetadataSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgSetDenomMetadataRequest>
                        for SetDenomMetadataSvc<T>
                    {
                        type Response = super::MsgSetDenomMetadataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSetDenomMetadataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_denom_metadata(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetDenomMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/GrantAllowance" => {
                    #[allow(non_camel_case_types)]
                    struct GrantAllowanceSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGrantAllowanceRequest> for GrantAllowanceSvc<T> {
                        type Response = super::MsgGrantAllowanceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGrantAllowanceRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).grant_allowance(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GrantAllowanceSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/AddFinalizeActivateMarker" => {
                    #[allow(non_camel_case_types)]
                    struct AddFinalizeActivateMarkerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgAddFinalizeActivateMarkerRequest>
                        for AddFinalizeActivateMarkerSvc<T>
                    {
                        type Response = super::MsgAddFinalizeActivateMarkerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddFinalizeActivateMarkerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).add_finalize_activate_marker(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddFinalizeActivateMarkerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Msg/SupplyIncreaseProposal" => {
                    #[allow(non_camel_case_types)]
                    struct SupplyIncreaseProposalSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgSupplyIncreaseProposalRequest>
                        for SupplyIncreaseProposalSvc<T>
                    {
                        type Response = super::MsgSupplyIncreaseProposalResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgSupplyIncreaseProposalRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).supply_increase_proposal(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SupplyIncreaseProposalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "provenance.marker.v1.Msg";
    }
}
/// SIPrefix represents an International System of Units (SI) Prefix.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SiPrefix {
    /// 10^0    (none)
    None = 0,
    /// 10^1    deka   da
    Deka = 1,
    /// 10^2    hecto   h
    Hecto = 2,
    /// 10^3    kilo    k
    Kilo = 3,
    /// 10^6    mega    M
    Mega = 6,
    /// 10^9    giga    G
    Giga = 9,
    /// 10^12   tera    T
    Tera = 12,
    /// 10^15   peta    P
    Peta = 15,
    /// 10^18   exa     E
    Exa = 18,
    /// 10^21   zetta   Z
    Zetta = 21,
    /// 10^24   yotta   Y
    Yotta = 24,
    /// 10^-1   deci    d
    Deci = -1,
    /// 10^-2   centi   c
    Centi = -2,
    /// 10^-3   milli   m
    Milli = -3,
    /// 10^-6   micro   µ
    Micro = -6,
    /// 10^-9   nano    n
    Nano = -9,
    /// 10^-12  pico    p
    Pico = -12,
    /// 10^-15  femto   f
    Femto = -15,
    /// 10^-18  atto    a
    Atto = -18,
    /// 10^-21  zepto   z
    Zepto = -21,
    /// 10^-24  yocto   y
    Yocto = -24,
}
impl SiPrefix {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SiPrefix::None => "SI_PREFIX_NONE",
            SiPrefix::Deka => "SI_PREFIX_DEKA",
            SiPrefix::Hecto => "SI_PREFIX_HECTO",
            SiPrefix::Kilo => "SI_PREFIX_KILO",
            SiPrefix::Mega => "SI_PREFIX_MEGA",
            SiPrefix::Giga => "SI_PREFIX_GIGA",
            SiPrefix::Tera => "SI_PREFIX_TERA",
            SiPrefix::Peta => "SI_PREFIX_PETA",
            SiPrefix::Exa => "SI_PREFIX_EXA",
            SiPrefix::Zetta => "SI_PREFIX_ZETTA",
            SiPrefix::Yotta => "SI_PREFIX_YOTTA",
            SiPrefix::Deci => "SI_PREFIX_DECI",
            SiPrefix::Centi => "SI_PREFIX_CENTI",
            SiPrefix::Milli => "SI_PREFIX_MILLI",
            SiPrefix::Micro => "SI_PREFIX_MICRO",
            SiPrefix::Nano => "SI_PREFIX_NANO",
            SiPrefix::Pico => "SI_PREFIX_PICO",
            SiPrefix::Femto => "SI_PREFIX_FEMTO",
            SiPrefix::Atto => "SI_PREFIX_ATTO",
            SiPrefix::Zepto => "SI_PREFIX_ZEPTO",
            SiPrefix::Yocto => "SI_PREFIX_YOCTO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SI_PREFIX_NONE" => Some(Self::None),
            "SI_PREFIX_DEKA" => Some(Self::Deka),
            "SI_PREFIX_HECTO" => Some(Self::Hecto),
            "SI_PREFIX_KILO" => Some(Self::Kilo),
            "SI_PREFIX_MEGA" => Some(Self::Mega),
            "SI_PREFIX_GIGA" => Some(Self::Giga),
            "SI_PREFIX_TERA" => Some(Self::Tera),
            "SI_PREFIX_PETA" => Some(Self::Peta),
            "SI_PREFIX_EXA" => Some(Self::Exa),
            "SI_PREFIX_ZETTA" => Some(Self::Zetta),
            "SI_PREFIX_YOTTA" => Some(Self::Yotta),
            "SI_PREFIX_DECI" => Some(Self::Deci),
            "SI_PREFIX_CENTI" => Some(Self::Centi),
            "SI_PREFIX_MILLI" => Some(Self::Milli),
            "SI_PREFIX_MICRO" => Some(Self::Micro),
            "SI_PREFIX_NANO" => Some(Self::Nano),
            "SI_PREFIX_PICO" => Some(Self::Pico),
            "SI_PREFIX_FEMTO" => Some(Self::Femto),
            "SI_PREFIX_ATTO" => Some(Self::Atto),
            "SI_PREFIX_ZEPTO" => Some(Self::Zepto),
            "SI_PREFIX_YOCTO" => Some(Self::Yocto),
            _ => None,
        }
    }
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryAllMarkersRequest is the request type for the Query/AllMarkers method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllMarkersRequest {
    /// Optional status to filter request
    #[prost(enumeration = "MarkerStatus", tag = "1")]
    pub status: i32,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryAllMarkersResponse is the response type for the Query/AllMarkers method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllMarkersResponse {
    #[prost(message, repeated, tag = "1")]
    pub markers: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryMarkerRequest is the request type for the Query/Marker method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarkerRequest {
    /// the address or denom of the marker
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryMarkerResponse is the response type for the Query/Marker method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMarkerResponse {
    #[prost(message, optional, tag = "1")]
    pub marker: ::core::option::Option<::prost_types::Any>,
}
/// QueryHoldingRequest is the request type for the Query/MarkerHolders method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHoldingRequest {
    /// the address or denom of the marker
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryHoldingResponse is the response type for the Query/MarkerHolders method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryHoldingResponse {
    #[prost(message, repeated, tag = "1")]
    pub balances: ::prost::alloc::vec::Vec<Balance>,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QuerySupplyRequest is the request type for the Query/MarkerSupply method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyRequest {
    /// address or denom for the marker
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QuerySupplyResponse is the response type for the Query/MarkerSupply method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyResponse {
    /// amount is the supply of the marker.
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// QueryEscrowRequest is the request type for the Query/MarkerEscrow method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEscrowRequest {
    /// address or denom for the marker
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryEscrowResponse is the response type for the Query/MarkerEscrow method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEscrowResponse {
    #[prost(message, repeated, tag = "1")]
    pub escrow: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// QueryAccessRequest is the request type for the Query/MarkerAccess method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccessRequest {
    /// address or denom for the marker
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// QueryAccessResponse is the response type for the Query/MarkerAccess method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccessResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<AccessGrant>,
}
/// QueryDenomMetadataRequest is the request type for Query/DenomMetadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomMetadataRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryDenomMetadataResponse is the response type for the Query/DenomMetadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomMetadataResponse {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::core::option::Option<cosmos_sdk_proto::cosmos::bank::v1beta1::Metadata>,
}
/// Balance defines an account address and balance pair used in queries for accounts holding a marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Balance {
    /// address is the address of the balance holder.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// coins defines the different coins this balance holds.
    #[prost(message, repeated, tag = "2")]
    pub coins: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC querier service for marker module.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl QueryClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QueryClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QueryClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            QueryClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Params queries the parameters of x/bank module.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Returns a list of all markers on the blockchain
        pub async fn all_markers(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllMarkersRequest>,
        ) -> Result<tonic::Response<super::QueryAllMarkersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/AllMarkers");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query for a single marker by denom or address
        pub async fn marker(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryMarkerRequest>,
        ) -> Result<tonic::Response<super::QueryMarkerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/Marker");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query for all accounts holding the given marker coins
        pub async fn holding(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryHoldingRequest>,
        ) -> Result<tonic::Response<super::QueryHoldingResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/Holding");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query for supply of coin on a marker account
        pub async fn supply(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySupplyRequest>,
        ) -> Result<tonic::Response<super::QuerySupplyResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/Supply");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query for coins on a marker account
        pub async fn escrow(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryEscrowRequest>,
        ) -> Result<tonic::Response<super::QueryEscrowResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/Escrow");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query for access records on an account
        pub async fn access(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAccessRequest>,
        ) -> Result<tonic::Response<super::QueryAccessResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/Access");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// query for access records on an account
        pub async fn denom_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomMetadataRequest>,
        ) -> Result<tonic::Response<super::QueryDenomMetadataResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.marker.v1.Query/DenomMetadata");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        /// Params queries the parameters of x/bank module.
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        /// Returns a list of all markers on the blockchain
        async fn all_markers(
            &self,
            request: tonic::Request<super::QueryAllMarkersRequest>,
        ) -> Result<tonic::Response<super::QueryAllMarkersResponse>, tonic::Status>;
        /// query for a single marker by denom or address
        async fn marker(
            &self,
            request: tonic::Request<super::QueryMarkerRequest>,
        ) -> Result<tonic::Response<super::QueryMarkerResponse>, tonic::Status>;
        /// query for all accounts holding the given marker coins
        async fn holding(
            &self,
            request: tonic::Request<super::QueryHoldingRequest>,
        ) -> Result<tonic::Response<super::QueryHoldingResponse>, tonic::Status>;
        /// query for supply of coin on a marker account
        async fn supply(
            &self,
            request: tonic::Request<super::QuerySupplyRequest>,
        ) -> Result<tonic::Response<super::QuerySupplyResponse>, tonic::Status>;
        /// query for coins on a marker account
        async fn escrow(
            &self,
            request: tonic::Request<super::QueryEscrowRequest>,
        ) -> Result<tonic::Response<super::QueryEscrowResponse>, tonic::Status>;
        /// query for access records on an account
        async fn access(
            &self,
            request: tonic::Request<super::QueryAccessRequest>,
        ) -> Result<tonic::Response<super::QueryAccessResponse>, tonic::Status>;
        /// query for access records on an account
        async fn denom_metadata(
            &self,
            request: tonic::Request<super::QueryDenomMetadataRequest>,
        ) -> Result<tonic::Response<super::QueryDenomMetadataResponse>, tonic::Status>;
    }
    /// Query defines the gRPC querier service for marker module.
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/provenance.marker.v1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest> for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/AllMarkers" => {
                    #[allow(non_camel_case_types)]
                    struct AllMarkersSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAllMarkersRequest> for AllMarkersSvc<T> {
                        type Response = super::QueryAllMarkersResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAllMarkersRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).all_markers(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AllMarkersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/Marker" => {
                    #[allow(non_camel_case_types)]
                    struct MarkerSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryMarkerRequest> for MarkerSvc<T> {
                        type Response = super::QueryMarkerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryMarkerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).marker(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MarkerSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/Holding" => {
                    #[allow(non_camel_case_types)]
                    struct HoldingSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryHoldingRequest> for HoldingSvc<T> {
                        type Response = super::QueryHoldingResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryHoldingRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).holding(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HoldingSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/Supply" => {
                    #[allow(non_camel_case_types)]
                    struct SupplySvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QuerySupplyRequest> for SupplySvc<T> {
                        type Response = super::QuerySupplyResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QuerySupplyRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).supply(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SupplySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/Escrow" => {
                    #[allow(non_camel_case_types)]
                    struct EscrowSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryEscrowRequest> for EscrowSvc<T> {
                        type Response = super::QueryEscrowResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryEscrowRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).escrow(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EscrowSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/Access" => {
                    #[allow(non_camel_case_types)]
                    struct AccessSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryAccessRequest> for AccessSvc<T> {
                        type Response = super::QueryAccessResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryAccessRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).access(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AccessSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/provenance.marker.v1.Query/DenomMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct DenomMetadataSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDenomMetadataRequest>
                        for DenomMetadataSvc<T>
                    {
                        type Response = super::QueryDenomMetadataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDenomMetadataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).denom_metadata(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DenomMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "provenance.marker.v1.Query";
    }
}
/// MarkerTransferAuthorization gives the grantee permissions to execute
/// a marker transfer on behalf of the granter's account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarkerTransferAuthorization {
    /// transfer_limit is the total amount the grantee can transfer
    #[prost(message, repeated, tag = "1")]
    pub transfer_limit: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// allow_list specifies an optional list of addresses to whom the grantee can send restricted coins on behalf of the
    /// granter. If omitted, any recipient is allowed.
    #[prost(string, repeated, tag = "2")]
    pub allow_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// GenesisState defines the account module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// A collection of marker accounts to create on start
    #[prost(message, repeated, tag = "2")]
    pub markers: ::prost::alloc::vec::Vec<MarkerAccount>,
}
/// AddMarkerProposal defines defines a governance proposal to create a new marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddMarkerProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub manager: ::prost::alloc::string::String,
    #[prost(enumeration = "MarkerStatus", tag = "5")]
    pub status: i32,
    #[prost(enumeration = "MarkerType", tag = "6")]
    pub marker_type: i32,
    #[prost(message, repeated, tag = "7")]
    pub access_list: ::prost::alloc::vec::Vec<AccessGrant>,
    #[prost(bool, tag = "8")]
    pub supply_fixed: bool,
    #[prost(bool, tag = "9")]
    pub allow_governance_control: bool,
}
/// SupplyIncreaseProposal defines a governance proposal to administer a marker and increase total supply of the marker
/// through minting coin and placing it within the marker or assigning it directly to an account
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupplyIncreaseProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// an optional target address for the minted coin from this request
    #[prost(string, tag = "4")]
    pub target_address: ::prost::alloc::string::String,
}
/// SupplyDecreaseProposal defines a governance proposal to administer a marker and decrease the total supply through
/// burning coin held within the marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SupplyDecreaseProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// SetAdministratorProposal defines a governance proposal to administer a marker and set administrators with specific
/// access on the marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAdministratorProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub access: ::prost::alloc::vec::Vec<AccessGrant>,
}
/// RemoveAdministratorProposal defines a governance proposal to administer a marker and remove all permissions for a
/// given address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAdministratorProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub removed_address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ChangeStatusProposal defines a governance proposal to administer a marker to change its status
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeStatusProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(enumeration = "MarkerStatus", tag = "4")]
    pub new_status: i32,
}
/// WithdrawEscrowProposal defines a governance proposal to withdraw escrow coins from a marker
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawEscrowProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "5")]
    pub target_address: ::prost::alloc::string::String,
}
/// SetDenomMetadataProposal defines a governance proposal to set the metadata for a denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDenomMetadataProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub metadata: ::core::option::Option<cosmos_sdk_proto::cosmos::bank::v1beta1::Metadata>,
}
