/// Params defines the set of params for the metadata module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {}
/// ScopeIdInfo contains various info regarding a scope id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeIdInfo {
    /// scope_id is the raw bytes of the scope address.
    #[prost(bytes = "vec", tag = "1")]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    /// scope_id_prefix is the prefix portion of the scope_id.
    #[prost(bytes = "vec", tag = "2")]
    pub scope_id_prefix: ::prost::alloc::vec::Vec<u8>,
    /// scope_id_scope_uuid is the scope_uuid portion of the scope_id.
    #[prost(bytes = "vec", tag = "3")]
    pub scope_id_scope_uuid: ::prost::alloc::vec::Vec<u8>,
    /// scope_addr is the bech32 string version of the scope_id.
    #[prost(string, tag = "4")]
    pub scope_addr: ::prost::alloc::string::String,
    /// scope_uuid is the uuid hex string of the scope_id_scope_uuid.
    #[prost(string, tag = "5")]
    pub scope_uuid: ::prost::alloc::string::String,
}
/// SessionIdInfo contains various info regarding a session id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionIdInfo {
    /// session_id is the raw bytes of the session address.
    #[prost(bytes = "vec", tag = "1")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    /// session_id_prefix is the prefix portion of the session_id.
    #[prost(bytes = "vec", tag = "2")]
    pub session_id_prefix: ::prost::alloc::vec::Vec<u8>,
    /// session_id_scope_uuid is the scope_uuid portion of the session_id.
    #[prost(bytes = "vec", tag = "3")]
    pub session_id_scope_uuid: ::prost::alloc::vec::Vec<u8>,
    /// session_id_session_uuid is the session_uuid portion of the session_id.
    #[prost(bytes = "vec", tag = "4")]
    pub session_id_session_uuid: ::prost::alloc::vec::Vec<u8>,
    /// session_addr is the bech32 string version of the session_id.
    #[prost(string, tag = "5")]
    pub session_addr: ::prost::alloc::string::String,
    /// session_uuid is the uuid hex string of the session_id_session_uuid.
    #[prost(string, tag = "6")]
    pub session_uuid: ::prost::alloc::string::String,
    /// scope_id_info is information about the scope id referenced in the session_id.
    #[prost(message, optional, tag = "7")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
}
/// RecordIdInfo contains various info regarding a record id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordIdInfo {
    /// record_id is the raw bytes of the record address.
    #[prost(bytes = "vec", tag = "1")]
    pub record_id: ::prost::alloc::vec::Vec<u8>,
    /// record_id_prefix is the prefix portion of the record_id.
    #[prost(bytes = "vec", tag = "2")]
    pub record_id_prefix: ::prost::alloc::vec::Vec<u8>,
    /// record_id_scope_uuid is the scope_uuid portion of the record_id.
    #[prost(bytes = "vec", tag = "3")]
    pub record_id_scope_uuid: ::prost::alloc::vec::Vec<u8>,
    /// record_id_hashed_name is the hashed name portion of the record_id.
    #[prost(bytes = "vec", tag = "4")]
    pub record_id_hashed_name: ::prost::alloc::vec::Vec<u8>,
    /// record_addr is the bech32 string version of the record_id.
    #[prost(string, tag = "5")]
    pub record_addr: ::prost::alloc::string::String,
    /// scope_id_info is information about the scope id referenced in the record_id.
    #[prost(message, optional, tag = "6")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
}
/// ScopeSpecIdInfo contains various info regarding a scope specification id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeSpecIdInfo {
    /// scope_spec_id is the raw bytes of the scope specification address.
    #[prost(bytes = "vec", tag = "1")]
    pub scope_spec_id: ::prost::alloc::vec::Vec<u8>,
    /// scope_spec_id_prefix is the prefix portion of the scope_spec_id.
    #[prost(bytes = "vec", tag = "2")]
    pub scope_spec_id_prefix: ::prost::alloc::vec::Vec<u8>,
    /// scope_spec_id_scope_spec_uuid is the scope_spec_uuid portion of the scope_spec_id.
    #[prost(bytes = "vec", tag = "3")]
    pub scope_spec_id_scope_spec_uuid: ::prost::alloc::vec::Vec<u8>,
    /// scope_spec_addr is the bech32 string version of the scope_spec_id.
    #[prost(string, tag = "4")]
    pub scope_spec_addr: ::prost::alloc::string::String,
    /// scope_spec_uuid is the uuid hex string of the scope_spec_id_scope_spec_uuid.
    #[prost(string, tag = "5")]
    pub scope_spec_uuid: ::prost::alloc::string::String,
}
/// ContractSpecIdInfo contains various info regarding a contract specification id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractSpecIdInfo {
    /// contract_spec_id is the raw bytes of the contract specification address.
    #[prost(bytes = "vec", tag = "1")]
    pub contract_spec_id: ::prost::alloc::vec::Vec<u8>,
    /// contract_spec_id_prefix is the prefix portion of the contract_spec_id.
    #[prost(bytes = "vec", tag = "2")]
    pub contract_spec_id_prefix: ::prost::alloc::vec::Vec<u8>,
    /// contract_spec_id_contract_spec_uuid is the contract_spec_uuid portion of the contract_spec_id.
    #[prost(bytes = "vec", tag = "3")]
    pub contract_spec_id_contract_spec_uuid: ::prost::alloc::vec::Vec<u8>,
    /// contract_spec_addr is the bech32 string version of the contract_spec_id.
    #[prost(string, tag = "4")]
    pub contract_spec_addr: ::prost::alloc::string::String,
    /// contract_spec_uuid is the uuid hex string of the contract_spec_id_contract_spec_uuid.
    #[prost(string, tag = "5")]
    pub contract_spec_uuid: ::prost::alloc::string::String,
}
/// RecordSpecIdInfo contains various info regarding a record specification id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordSpecIdInfo {
    /// record_spec_id is the raw bytes of the record specification address.
    #[prost(bytes = "vec", tag = "1")]
    pub record_spec_id: ::prost::alloc::vec::Vec<u8>,
    /// record_spec_id_prefix is the prefix portion of the record_spec_id.
    #[prost(bytes = "vec", tag = "2")]
    pub record_spec_id_prefix: ::prost::alloc::vec::Vec<u8>,
    /// record_spec_id_contract_spec_uuid is the contract_spec_uuid portion of the record_spec_id.
    #[prost(bytes = "vec", tag = "3")]
    pub record_spec_id_contract_spec_uuid: ::prost::alloc::vec::Vec<u8>,
    /// record_spec_id_hashed_name is the hashed name portion of the record_spec_id.
    #[prost(bytes = "vec", tag = "4")]
    pub record_spec_id_hashed_name: ::prost::alloc::vec::Vec<u8>,
    /// record_spec_addr is the bech32 string version of the record_spec_id.
    #[prost(string, tag = "5")]
    pub record_spec_addr: ::prost::alloc::string::String,
    /// contract_spec_id_info is information about the contract spec id referenced in the record_spec_id.
    #[prost(message, optional, tag = "6")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
}
/// Defines an Locator object stored on chain, which represents a owner( blockchain address) associated with a endpoint
/// uri for it's associated object store.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectStoreLocator {
    /// account address the endpoint is owned by
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// locator endpoint uri
    #[prost(string, tag = "2")]
    pub locator_uri: ::prost::alloc::string::String,
    /// owners encryption key address
    #[prost(string, tag = "3")]
    pub encryption_key: ::prost::alloc::string::String,
}
/// Params defines the parameters for the metadata-locator module methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsLocatorParams {
    #[prost(uint32, tag = "1")]
    pub max_uri_length: u32,
}
/// ScopeSpecification defines the required parties, resources, conditions, and consideration outputs for a contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeSpecification {
    /// unique identifier for this specification on chain
    #[prost(bytes = "vec", tag = "1")]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    /// General information about this scope specification.
    #[prost(message, optional, tag = "2")]
    pub description: ::core::option::Option<Description>,
    /// Addresses of the owners of this scope specification.
    #[prost(string, repeated, tag = "3")]
    pub owner_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// A list of parties that must be present on a scope (and their associated roles)
    #[prost(enumeration = "PartyType", repeated, packed = "false", tag = "4")]
    pub parties_involved: ::prost::alloc::vec::Vec<i32>,
    /// A list of contract specification ids allowed for a scope based on this specification.
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub contract_spec_ids: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// ContractSpecification defines the required parties, resources, conditions, and consideration outputs for a contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractSpecification {
    /// unique identifier for this specification on chain
    #[prost(bytes = "vec", tag = "1")]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    /// Description information for this contract specification
    #[prost(message, optional, tag = "2")]
    pub description: ::core::option::Option<Description>,
    /// Address of the account that owns this specificaiton
    #[prost(string, repeated, tag = "3")]
    pub owner_addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// a list of party roles that must be fullfilled when signing a transaction for this contract specification
    #[prost(enumeration = "PartyType", repeated, packed = "false", tag = "4")]
    pub parties_involved: ::prost::alloc::vec::Vec<i32>,
    /// name of the class/type of this contract executable
    #[prost(string, tag = "7")]
    pub class_name: ::prost::alloc::string::String,
    /// Reference to a metadata record with a hash and type information for the instance of code that will process this
    /// contract
    #[prost(oneof = "contract_specification::Source", tags = "5, 6")]
    pub source: ::core::option::Option<contract_specification::Source>,
}
/// Nested message and enum types in `ContractSpecification`.
pub mod contract_specification {
    /// Reference to a metadata record with a hash and type information for the instance of code that will process this
    /// contract
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// the address of a record on chain that represents this contract
        #[prost(bytes, tag = "5")]
        ResourceId(::prost::alloc::vec::Vec<u8>),
        /// the hash of contract binary (off-chain instance)
        #[prost(string, tag = "6")]
        Hash(::prost::alloc::string::String),
    }
}
/// RecordSpecification defines the specification for a Record including allowed/required inputs/outputs
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordSpecification {
    /// unique identifier for this specification on chain
    #[prost(bytes = "vec", tag = "1")]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    /// Name of Record that will be created when this specification is used
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// A set of inputs that must be satisified to apply this RecordSpecification and create a Record
    #[prost(message, repeated, tag = "3")]
    pub inputs: ::prost::alloc::vec::Vec<InputSpecification>,
    /// A type name for data associated with this record (typically a class or proto name)
    #[prost(string, tag = "4")]
    pub type_name: ::prost::alloc::string::String,
    /// Type of result for this record specification (must be RECORD or RECORD_LIST)
    #[prost(enumeration = "DefinitionType", tag = "5")]
    pub result_type: i32,
    /// Type of party responsible for this record
    #[prost(enumeration = "PartyType", repeated, packed = "false", tag = "6")]
    pub responsible_parties: ::prost::alloc::vec::Vec<i32>,
}
/// InputSpecification defines a name, type_name, and source reference (either on or off chain) to define an input
/// parameter
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputSpecification {
    /// name for this input
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// a type_name (typically a proto name or class_name)
    #[prost(string, tag = "2")]
    pub type_name: ::prost::alloc::string::String,
    /// source is either on chain (record_id) or off-chain (hash)
    #[prost(oneof = "input_specification::Source", tags = "3, 4")]
    pub source: ::core::option::Option<input_specification::Source>,
}
/// Nested message and enum types in `InputSpecification`.
pub mod input_specification {
    /// source is either on chain (record_id) or off-chain (hash)
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// the address of a record on chain (For Established Records)
        #[prost(bytes, tag = "3")]
        RecordId(::prost::alloc::vec::Vec<u8>),
        /// the hash of an off-chain piece of information (For Proposed Records)
        #[prost(string, tag = "4")]
        Hash(::prost::alloc::string::String),
    }
}
/// Description holds general information that is handy to associate with a structure.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Description {
    /// A Name for this thing.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// A description of this thing.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// URL to find even more info.
    #[prost(string, tag = "4")]
    pub website_url: ::prost::alloc::string::String,
    /// URL of an icon.
    #[prost(string, tag = "5")]
    pub icon_url: ::prost::alloc::string::String,
}
/// DefinitionType indicates the required definition type for this value
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DefinitionType {
    /// DEFINITION_TYPE_UNSPECIFIED indicates an unknown/invalid value
    Unspecified = 0,
    /// DEFINITION_TYPE_PROPOSED indicates a proposed value is used here (a record that is not on-chain)
    Proposed = 1,
    /// DEFINITION_TYPE_RECORD indicates the value must be a reference to a record on chain
    Record = 2,
    /// DEFINITION_TYPE_RECORD_LIST indicates the value maybe a reference to a collection of values on chain having
    /// the same name
    RecordList = 3,
}
impl DefinitionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DefinitionType::Unspecified => "DEFINITION_TYPE_UNSPECIFIED",
            DefinitionType::Proposed => "DEFINITION_TYPE_PROPOSED",
            DefinitionType::Record => "DEFINITION_TYPE_RECORD",
            DefinitionType::RecordList => "DEFINITION_TYPE_RECORD_LIST",
        }
    }
}
/// PartyType are the different roles parties on a contract may use
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PartyType {
    /// PARTY_TYPE_UNSPECIFIED is an error condition
    Unspecified = 0,
    /// PARTY_TYPE_ORIGINATOR is an asset originator
    Originator = 1,
    /// PARTY_TYPE_SERVICER provides debt servicing functions
    Servicer = 2,
    /// PARTY_TYPE_INVESTOR is a generic investor
    Investor = 3,
    /// PARTY_TYPE_CUSTODIAN is an entity that provides custodian services for assets
    Custodian = 4,
    /// PARTY_TYPE_OWNER indicates this party is an owner of the item
    Owner = 5,
    /// PARTY_TYPE_AFFILIATE is a party with an affiliate agreement
    Affiliate = 6,
    /// PARTY_TYPE_OMNIBUS is a special type of party that controls an omnibus bank account
    Omnibus = 7,
    /// PARTY_TYPE_PROVENANCE is used to indicate this party represents the blockchain or a smart contract action
    Provenance = 8,
    /// PARTY_TYPE_CONTROLLER is an entity which controls a specific asset on chain (ie enote)
    Controller = 10,
    /// PARTY_TYPE_VALIDATOR is an entity which validates given assets on chain
    Validator = 11,
}
impl PartyType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PartyType::Unspecified => "PARTY_TYPE_UNSPECIFIED",
            PartyType::Originator => "PARTY_TYPE_ORIGINATOR",
            PartyType::Servicer => "PARTY_TYPE_SERVICER",
            PartyType::Investor => "PARTY_TYPE_INVESTOR",
            PartyType::Custodian => "PARTY_TYPE_CUSTODIAN",
            PartyType::Owner => "PARTY_TYPE_OWNER",
            PartyType::Affiliate => "PARTY_TYPE_AFFILIATE",
            PartyType::Omnibus => "PARTY_TYPE_OMNIBUS",
            PartyType::Provenance => "PARTY_TYPE_PROVENANCE",
            PartyType::Controller => "PARTY_TYPE_CONTROLLER",
            PartyType::Validator => "PARTY_TYPE_VALIDATOR",
        }
    }
}
/// Scope defines a root reference for a collection of records owned by one or more parties.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Scope {
    /// Unique ID for this scope.  Implements sdk.Address interface for use where addresses are required in Cosmos
    #[prost(bytes = "vec", tag = "1")]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    /// the scope specification that contains the specifications for data elements allowed within this scope
    #[prost(bytes = "vec", tag = "2")]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    /// These parties represent top level owners of the records within.  These parties must sign any requests that modify
    /// the data within the scope.  These addresses are in union with parties listed on the sessions.
    #[prost(message, repeated, tag = "3")]
    pub owners: ::prost::alloc::vec::Vec<Party>,
    /// Addessses in this list are authorized to recieve off-chain data associated with this scope.
    #[prost(string, repeated, tag = "4")]
    pub data_access: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// An address that controls the value associated with this scope.  Standard blockchain accounts and marker accounts
    /// are supported for this value.  This attribute may only be changed by the entity indicated once it is set.
    #[prost(string, tag = "5")]
    pub value_owner_address: ::prost::alloc::string::String,
}
///
/// A Session is created for an execution context against a specific specification instance
///
/// The context will have a specification and set of parties involved.  The Session may be updated several
/// times so long as the parties listed are signers on the transaction.  NOTE: When there are no Records within a Scope
/// that reference a Session it is removed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Session {
    #[prost(bytes = "vec", tag = "1")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    /// unique id of the contract specification that was used to create this session.
    #[prost(bytes = "vec", tag = "2")]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    /// parties is the set of identities that signed this contract
    #[prost(message, repeated, tag = "3")]
    pub parties: ::prost::alloc::vec::Vec<Party>,
    /// name to associate with this session execution context, typically classname
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// context is a field for storing client specific data associated with a session.
    #[prost(bytes = "vec", tag = "5")]
    pub context: ::prost::alloc::vec::Vec<u8>,
    /// Created by, updated by, timestamps, version number, and related info.
    #[prost(message, optional, tag = "99")]
    pub audit: ::core::option::Option<AuditFields>,
}
/// A record (of fact) is attached to a session or each consideration output from a contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Record {
    /// name/identifier for this record.  Value must be unique within the scope.  Also known as a Fact name
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// id of the session context that was used to create this record (use with filtered kvprefix iterator)
    #[prost(bytes = "vec", tag = "2")]
    pub session_id: ::prost::alloc::vec::Vec<u8>,
    /// process contain information used to uniquely identify an execution on or off chain that generated this record
    #[prost(message, optional, tag = "3")]
    pub process: ::core::option::Option<Process>,
    /// inputs used with the process to achieve the output on this record
    #[prost(message, repeated, tag = "4")]
    pub inputs: ::prost::alloc::vec::Vec<RecordInput>,
    /// output(s) is the results of executing the process on the given process indicated in this record
    #[prost(message, repeated, tag = "5")]
    pub outputs: ::prost::alloc::vec::Vec<RecordOutput>,
    /// specification_id is the id of the record specification that was used to create this record.
    #[prost(bytes = "vec", tag = "6")]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
}
/// Process contains information used to uniquely identify what was used to generate this record
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Process {
    /// a name associated with the process (type_name, classname or smart contract common name)
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// method is a name or reference to a specific operation (method) within a class/contract that was invoked
    #[prost(string, tag = "4")]
    pub method: ::prost::alloc::string::String,
    /// unique identifier for this process
    #[prost(oneof = "process::ProcessId", tags = "1, 2")]
    pub process_id: ::core::option::Option<process::ProcessId>,
}
/// Nested message and enum types in `Process`.
pub mod process {
    /// unique identifier for this process
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ProcessId {
        /// the address of a smart contract used for this process
        #[prost(string, tag = "1")]
        Address(::prost::alloc::string::String),
        /// the hash of an off-chain process used
        #[prost(string, tag = "2")]
        Hash(::prost::alloc::string::String),
    }
}
/// Tracks the inputs used to establish this record
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordInput {
    /// Name value included to link back to the definition spec.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// from proposed fact structure to unmarshal
    #[prost(string, tag = "4")]
    pub type_name: ::prost::alloc::string::String,
    /// Indicates if this input was a recorded fact on chain or just a given hashed input
    #[prost(enumeration = "RecordInputStatus", tag = "5")]
    pub status: i32,
    /// data source
    #[prost(oneof = "record_input::Source", tags = "2, 3")]
    pub source: ::core::option::Option<record_input::Source>,
}
/// Nested message and enum types in `RecordInput`.
pub mod record_input {
    /// data source
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Source {
        /// the address of a record on chain (For Established Records)
        #[prost(bytes, tag = "2")]
        RecordId(::prost::alloc::vec::Vec<u8>),
        /// the hash of an off-chain piece of information (For Proposed Records)
        #[prost(string, tag = "3")]
        Hash(::prost::alloc::string::String),
    }
}
/// RecordOutput encapsulates the output of a process recorded on chain
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordOutput {
    /// Hash of the data output that was output/generated for this record
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    /// Status of the process execution associated with this output indicating success,failure, or pending
    #[prost(enumeration = "ResultStatus", tag = "2")]
    pub status: i32,
}
/// A Party is an address with/in a given role associated with a contract
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Party {
    /// address of the account (on chain)
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// a role for this account within the context of the processes used
    #[prost(enumeration = "PartyType", tag = "2")]
    pub role: i32,
}
/// AuditFields capture information about the last account to make modifications and when they were made
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuditFields {
    /// the date/time when this entry was created
    #[prost(message, optional, tag = "1")]
    pub created_date: ::core::option::Option<::prost_types::Timestamp>,
    /// the address of the account that created this record
    #[prost(string, tag = "2")]
    pub created_by: ::prost::alloc::string::String,
    /// the date/time when this entry was last updated
    #[prost(message, optional, tag = "3")]
    pub updated_date: ::core::option::Option<::prost_types::Timestamp>,
    /// the address of the account that modified this record
    #[prost(string, tag = "4")]
    pub updated_by: ::prost::alloc::string::String,
    /// an optional version number that is incremented with each update
    #[prost(uint32, tag = "5")]
    pub version: u32,
    /// an optional message associated with the creation/update event
    #[prost(string, tag = "6")]
    pub message: ::prost::alloc::string::String,
}
/// A set of types for inputs on a record (of fact)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RecordInputStatus {
    /// RECORD_INPUT_STATUS_UNSPECIFIED indicates an invalid/unknown input type
    Unspecified = 0,
    /// RECORD_INPUT_STATUS_PROPOSED indicates this input was an arbitrary piece of data that was hashed
    Proposed = 1,
    /// RECORD_INPUT_STATUS_RECORD indicates this input is a reference to a previously recorded fact on blockchain
    Record = 2,
}
impl RecordInputStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RecordInputStatus::Unspecified => "RECORD_INPUT_STATUS_UNSPECIFIED",
            RecordInputStatus::Proposed => "RECORD_INPUT_STATUS_PROPOSED",
            RecordInputStatus::Record => "RECORD_INPUT_STATUS_RECORD",
        }
    }
}
/// ResultStatus indicates the various states of execution of a record
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResultStatus {
    /// RESULT_STATUS_UNSPECIFIED indicates an unset condition
    Unspecified = 0,
    /// RESULT_STATUS_PASS indicates the execution was successfult
    Pass = 1,
    /// RESULT_STATUS_SKIP indicates condition/consideration was skipped due to missing inputs or delayed execution
    Skip = 2,
    /// RESULT_STATUS_FAIL indicates the execution of the condition/consideration failed.
    Fail = 3,
}
impl ResultStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResultStatus::Unspecified => "RESULT_STATUS_UNSPECIFIED",
            ResultStatus::Pass => "RESULT_STATUS_PASS",
            ResultStatus::Skip => "RESULT_STATUS_SKIP",
            ResultStatus::Fail => "RESULT_STATUS_FAIL",
        }
    }
}
/// MsgWriteScopeRequest is the request type for the Msg/WriteScope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteScopeRequest {
    /// scope is the Scope you want added or updated.
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<Scope>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// scope_uuid is an optional uuid string, e.g. "91978ba2-5f35-459a-86a7-feca1b0512e0"
    /// If provided, it will be used to generate the MetadataAddress for the scope which will override the scope_id in the
    /// provided scope. If not provided (or it is an empty string), nothing special happens.
    /// If there is a value in scope.scope_id that is different from the one created from this uuid, an error is returned.
    #[prost(string, tag = "3")]
    pub scope_uuid: ::prost::alloc::string::String,
    /// spec_uuid is an optional scope specification uuid string, e.g. "dc83ea70-eacd-40fe-9adf-1cf6148bf8a2"
    /// If provided, it will be used to generate the MetadataAddress for the scope specification which will override the
    /// specification_id in the provided scope. If not provided (or it is an empty string), nothing special happens.
    /// If there is a value in scope.specification_id that is different from the one created from this uuid, an error is
    /// returned.
    #[prost(string, tag = "4")]
    pub spec_uuid: ::prost::alloc::string::String,
}
/// MsgWriteScopeResponse is the response type for the Msg/WriteScope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteScopeResponse {
    /// scope_id_info contains information about the id/address of the scope that was added or updated.
    #[prost(message, optional, tag = "1")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
}
/// MsgDeleteScopeRequest is the request type for the Msg/DeleteScope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteScopeRequest {
    /// Unique ID for the scope to delete
    #[prost(bytes = "vec", tag = "1")]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteScopeResponse is the response type for the Msg/DeleteScope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteScopeResponse {}
/// MsgAddScopeDataAccessRequest is the request to add data access AccAddress to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddScopeDataAccessRequest {
    /// scope MetadataAddress for updating data access
    #[prost(bytes = "vec", tag = "1")]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    /// AccAddress addresses to be added to scope
    #[prost(string, repeated, tag = "2")]
    pub data_access: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgAddScopeDataAccessResponse is the response for adding data access AccAddress to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddScopeDataAccessResponse {}
/// MsgDeleteScopeDataAccessRequest is the request to remove data access AccAddress to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteScopeDataAccessRequest {
    /// scope MetadataAddress for removing data access
    #[prost(bytes = "vec", tag = "1")]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    /// AccAddress address to be removed from scope
    #[prost(string, repeated, tag = "2")]
    pub data_access: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteScopeDataAccessResponse is the response from removing data access AccAddress to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteScopeDataAccessResponse {}
/// MsgAddScopeOwnerRequest is the request to add owner AccAddress to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddScopeOwnerRequest {
    /// scope MetadataAddress for updating data access
    #[prost(bytes = "vec", tag = "1")]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    /// AccAddress owner addresses to be added to scope
    #[prost(message, repeated, tag = "2")]
    pub owners: ::prost::alloc::vec::Vec<Party>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgAddScopeOwnerResponse is the response for adding owner AccAddresses to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddScopeOwnerResponse {}
/// MsgDeleteScopeOwnerRequest is the request to remove owner AccAddresses to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteScopeOwnerRequest {
    /// scope MetadataAddress for removing data access
    #[prost(bytes = "vec", tag = "1")]
    pub scope_id: ::prost::alloc::vec::Vec<u8>,
    /// AccAddress owner addresses to be removed from scope
    #[prost(string, repeated, tag = "2")]
    pub owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteScopeOwnerResponse is the response from removing owner AccAddress to scope
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteScopeOwnerResponse {}
/// MsgWriteSessionRequest is the request type for the Msg/WriteSession RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteSessionRequest {
    /// session is the Session you want added or updated.
    #[prost(message, optional, tag = "1")]
    pub session: ::core::option::Option<Session>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// SessionIDComponents is an optional (alternate) way of defining what the session_id should be in the provided
    /// session. If provided, it must have both a scope and session_uuid. Those components will be used to create the
    /// MetadataAddress for the session which will override the session_id in the provided session. If not provided (or
    /// all empty), nothing special happens.
    /// If there is a value in session.session_id that is different from the one created from these components, an error is
    /// returned.
    #[prost(message, optional, tag = "3")]
    pub session_id_components: ::core::option::Option<SessionIdComponents>,
    /// spec_uuid is an optional contract specification uuid string, e.g. "def6bc0a-c9dd-4874-948f-5206e6060a84"
    /// If provided, it will be used to generate the MetadataAddress for the contract specification which will override the
    /// specification_id in the provided session. If not provided (or it is an empty string), nothing special happens.
    /// If there is a value in session.specification_id that is different from the one created from this uuid, an error is
    /// returned.
    #[prost(string, tag = "4")]
    pub spec_uuid: ::prost::alloc::string::String,
}
/// SessionIDComponents contains fields for the components that make up a session id.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionIdComponents {
    /// session_uuid is a uuid string for identifying this session, e.g. "5803f8bc-6067-4eb5-951f-2121671c2ec0"
    #[prost(string, tag = "3")]
    pub session_uuid: ::prost::alloc::string::String,
    /// scope is used to define the scope this session belongs to.
    #[prost(oneof = "session_id_components::ScopeIdentifier", tags = "1, 2")]
    pub scope_identifier: ::core::option::Option<session_id_components::ScopeIdentifier>,
}
/// Nested message and enum types in `SessionIdComponents`.
pub mod session_id_components {
    /// scope is used to define the scope this session belongs to.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ScopeIdentifier {
        /// scope_uuid is the uuid string for the scope, e.g. "91978ba2-5f35-459a-86a7-feca1b0512e0"
        #[prost(string, tag = "1")]
        ScopeUuid(::prost::alloc::string::String),
        /// scope_addr is the bech32 address string for the scope, g.g. "scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel"
        #[prost(string, tag = "2")]
        ScopeAddr(::prost::alloc::string::String),
    }
}
/// MsgWriteSessionResponse is the response type for the Msg/WriteSession RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteSessionResponse {
    /// session_id_info contains information about the id/address of the session that was added or updated.
    #[prost(message, optional, tag = "1")]
    pub session_id_info: ::core::option::Option<SessionIdInfo>,
}
/// MsgWriteRecordRequest is the request type for the Msg/WriteRecord RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteRecordRequest {
    /// record is the Record you want added or updated.
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<Record>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// SessionIDComponents is an optional (alternate) way of defining what the session_id should be in the provided
    /// record. If provided, it must have both a scope and session_uuid. Those components will be used to create the
    /// MetadataAddress for the session which will override the session_id in the provided record. If not provided (or
    /// all empty), nothing special happens.
    /// If there is a value in record.session_id that is different from the one created from these components, an error is
    /// returned.
    #[prost(message, optional, tag = "3")]
    pub session_id_components: ::core::option::Option<SessionIdComponents>,
    /// contract_spec_uuid is an optional contract specification uuid string, e.g. "def6bc0a-c9dd-4874-948f-5206e6060a84"
    /// If provided, it will be combined with the record name to generate the MetadataAddress for the record specification
    /// which will override the specification_id in the provided record. If not provided (or it is an empty string),
    /// nothing special happens.
    /// If there is a value in record.specification_id that is different from the one created from this uuid and
    /// record.name, an error is returned.
    #[prost(string, tag = "4")]
    pub contract_spec_uuid: ::prost::alloc::string::String,
    /// parties is the list of parties involved with this record.
    #[prost(message, repeated, tag = "5")]
    pub parties: ::prost::alloc::vec::Vec<Party>,
}
/// MsgWriteRecordResponse is the response type for the Msg/WriteRecord RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteRecordResponse {
    /// record_id_info contains information about the id/address of the record that was added or updated.
    #[prost(message, optional, tag = "1")]
    pub record_id_info: ::core::option::Option<RecordIdInfo>,
}
/// MsgDeleteRecordRequest is the request type for the Msg/DeleteRecord RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteRecordRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub record_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteRecordResponse is the response type for the Msg/DeleteRecord RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteRecordResponse {}
/// MsgWriteScopeSpecificationRequest is the request type for the Msg/WriteScopeSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteScopeSpecificationRequest {
    /// specification is the ScopeSpecification you want added or updated.
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<ScopeSpecification>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// spec_uuid is an optional scope specification uuid string, e.g. "dc83ea70-eacd-40fe-9adf-1cf6148bf8a2"
    /// If provided, it will be used to generate the MetadataAddress for the scope specification which will override the
    /// specification_id in the provided specification. If not provided (or it is an empty string), nothing special
    /// happens.
    /// If there is a value in specification.specification_id that is different from the one created from this uuid, an
    /// error is returned.
    #[prost(string, tag = "3")]
    pub spec_uuid: ::prost::alloc::string::String,
}
/// MsgWriteScopeSpecificationResponse is the response type for the Msg/WriteScopeSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteScopeSpecificationResponse {
    /// scope_spec_id_info contains information about the id/address of the scope specification that was added or updated.
    #[prost(message, optional, tag = "1")]
    pub scope_spec_id_info: ::core::option::Option<ScopeSpecIdInfo>,
}
/// MsgDeleteScopeSpecificationRequest is the request type for the Msg/DeleteScopeSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteScopeSpecificationRequest {
    /// MetadataAddress for the scope specification to delete.
    #[prost(bytes = "vec", tag = "1")]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteScopeSpecificationResponse is the response type for the Msg/DeleteScopeSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteScopeSpecificationResponse {}
/// MsgWriteContractSpecificationRequest is the request type for the Msg/WriteContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteContractSpecificationRequest {
    /// specification is the ContractSpecification you want added or updated.
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<ContractSpecification>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// spec_uuid is an optional contract specification uuid string, e.g. "def6bc0a-c9dd-4874-948f-5206e6060a84"
    /// If provided, it will be used to generate the MetadataAddress for the contract specification which will override the
    /// specification_id in the provided specification. If not provided (or it is an empty string), nothing special
    /// happens.
    /// If there is a value in specification.specification_id that is different from the one created from this uuid, an
    /// error is returned.
    #[prost(string, tag = "3")]
    pub spec_uuid: ::prost::alloc::string::String,
}
/// MsgWriteContractSpecificationResponse is the response type for the Msg/WriteContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteContractSpecificationResponse {
    /// contract_spec_id_info contains information about the id/address of the contract specification that was added or
    /// updated.
    #[prost(message, optional, tag = "1")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
}
/// MsgAddContractSpecToScopeSpecRequest is the request type for the Msg/AddContractSpecToScopeSpec RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddContractSpecToScopeSpecRequest {
    /// MetadataAddress for the contract specification to add.
    #[prost(bytes = "vec", tag = "1")]
    pub contract_specification_id: ::prost::alloc::vec::Vec<u8>,
    /// MetadataAddress for the scope specification to add contract specification to.
    #[prost(bytes = "vec", tag = "2")]
    pub scope_specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgAddContractSpecToScopeSpecResponse is the response type for the Msg/AddContractSpecToScopeSpec RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddContractSpecToScopeSpecResponse {}
/// MsgDeleteContractSpecFromScopeSpecRequest is the request type for the Msg/DeleteContractSpecFromScopeSpec RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteContractSpecFromScopeSpecRequest {
    /// MetadataAddress for the contract specification to add.
    #[prost(bytes = "vec", tag = "1")]
    pub contract_specification_id: ::prost::alloc::vec::Vec<u8>,
    /// MetadataAddress for the scope specification to add contract specification to.
    #[prost(bytes = "vec", tag = "2")]
    pub scope_specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteContractSpecFromScopeSpecResponse is the response type for the Msg/DeleteContractSpecFromScopeSpec RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteContractSpecFromScopeSpecResponse {}
/// MsgDeleteContractSpecificationRequest is the request type for the Msg/DeleteContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteContractSpecificationRequest {
    /// MetadataAddress for the contract specification to delete.
    #[prost(bytes = "vec", tag = "1")]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteContractSpecificationResponse is the response type for the Msg/DeleteContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteContractSpecificationResponse {}
/// MsgWriteRecordSpecificationRequest is the request type for the Msg/WriteRecordSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteRecordSpecificationRequest {
    /// specification is the RecordSpecification you want added or updated.
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<RecordSpecification>,
    /// signers is the list of address of those signing this request.
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// contract_spec_uuid is an optional contract specification uuid string, e.g. "def6bc0a-c9dd-4874-948f-5206e6060a84"
    /// If provided, it will be combined with the record specification name to generate the MetadataAddress for the record
    /// specification which will override the specification_id in the provided specification. If not provided (or it is an
    /// empty string), nothing special happens.
    /// If there is a value in specification.specification_id that is different from the one created from this uuid and
    /// specification.name, an error is returned.
    #[prost(string, tag = "3")]
    pub contract_spec_uuid: ::prost::alloc::string::String,
}
/// MsgWriteRecordSpecificationResponse is the response type for the Msg/WriteRecordSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteRecordSpecificationResponse {
    /// record_spec_id_info contains information about the id/address of the record specification that was added or
    /// updated.
    #[prost(message, optional, tag = "1")]
    pub record_spec_id_info: ::core::option::Option<RecordSpecIdInfo>,
}
/// MsgDeleteRecordSpecificationRequest is the request type for the Msg/DeleteRecordSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteRecordSpecificationRequest {
    /// MetadataAddress for the record specification to delete.
    #[prost(bytes = "vec", tag = "1")]
    pub specification_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgDeleteRecordSpecificationResponse is the response type for the Msg/DeleteRecordSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteRecordSpecificationResponse {}
/// MsgWriteP8eContractSpecRequest is the request type for the Msg/WriteP8eContractSpec RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteP8eContractSpecRequest {
    /// ContractSpec v39 p8e ContractSpect to be converted into a v40
    #[prost(message, optional, tag = "1")]
    pub contractspec: ::core::option::Option<p8e::ContractSpec>,
    #[prost(string, repeated, tag = "2")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgWriteP8eContractSpecResponse is the response type for the Msg/WriteP8eContractSpec RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWriteP8eContractSpecResponse {
    /// contract_spec_id_info contains information about the id/address of the contract specification that was added or
    /// updated.
    #[prost(message, optional, tag = "1")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
    /// record_spec_id_infos contains information about the ids/addresses of the record specifications that were added or
    /// updated.
    #[prost(message, repeated, tag = "2")]
    pub record_spec_id_infos: ::prost::alloc::vec::Vec<RecordSpecIdInfo>,
}
/// MsgP8eMemorializeContractRequest is the request type for the Msg/P8eMemorializeContract RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgP8eMemorializeContractRequest {
    /// The scope id of the object being add or modified on blockchain.
    #[prost(string, tag = "1")]
    pub scope_id: ::prost::alloc::string::String,
    /// The uuid of the contract execution.
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
    /// The scope specification id.
    #[prost(string, tag = "3")]
    pub scope_specification_id: ::prost::alloc::string::String,
    /// The new recitals for the scope.  Used in leu of Contract for direct ownership changes.
    #[prost(message, optional, tag = "4")]
    pub recitals: ::core::option::Option<p8e::Recitals>,
    /// The executed contract.
    #[prost(message, optional, tag = "5")]
    pub contract: ::core::option::Option<p8e::Contract>,
    /// The contract signatures
    #[prost(message, optional, tag = "6")]
    pub signatures: ::core::option::Option<p8e::SignatureSet>,
    /// The bech32 address of the notary (ie the broadcaster of this message).
    #[prost(string, tag = "7")]
    pub invoker: ::prost::alloc::string::String,
}
/// MsgP8eMemorializeContractResponse is the response type for the Msg/P8eMemorializeContract RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgP8eMemorializeContractResponse {
    /// scope_id_info contains information about the id/address of the scope that was added or updated.
    #[prost(message, optional, tag = "1")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
    /// session_id_info contains information about the id/address of the session that was added or updated.
    #[prost(message, optional, tag = "2")]
    pub session_id_info: ::core::option::Option<SessionIdInfo>,
    /// record_id_infos contains information about the ids/addresses of the records that were added or updated.
    #[prost(message, repeated, tag = "3")]
    pub record_id_infos: ::prost::alloc::vec::Vec<RecordIdInfo>,
}
/// MsgBindOSLocatorRequest is the request type for the Msg/BindOSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBindOsLocatorRequest {
    /// The object locator to bind the address to bind to the URI.
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
/// MsgBindOSLocatorResponse is the response type for the Msg/BindOSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBindOsLocatorResponse {
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
/// MsgDeleteOSLocatorRequest is the request type for the Msg/DeleteOSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteOsLocatorRequest {
    /// The record being removed
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
/// MsgDeleteOSLocatorResponse is the response type for the Msg/DeleteOSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDeleteOsLocatorResponse {
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
/// MsgModifyOSLocatorRequest is the request type for the Msg/ModifyOSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgModifyOsLocatorRequest {
    /// The object locator to bind the address to bind to the URI.
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
/// MsgModifyOSLocatorResponse is the response type for the Msg/ModifyOSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgModifyOsLocatorResponse {
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the Metadata Msg service.
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
        /// WriteScope adds or updates a scope.
        pub async fn write_scope(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWriteScopeRequest>,
        ) -> Result<tonic::Response<super::MsgWriteScopeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/WriteScope");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteScope deletes a scope and all associated Records, Sessions.
        pub async fn delete_scope(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteScopeRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteScopeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/DeleteScope");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AddScopeDataAccess adds data access AccAddress to scope
        pub async fn add_scope_data_access(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddScopeDataAccessRequest>,
        ) -> Result<tonic::Response<super::MsgAddScopeDataAccessResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/AddScopeDataAccess",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteScopeDataAccess removes data access AccAddress from scope
        pub async fn delete_scope_data_access(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteScopeDataAccessRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteScopeDataAccessResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/DeleteScopeDataAccess",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AddScopeOwner adds new owner AccAddress to scope
        pub async fn add_scope_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddScopeOwnerRequest>,
        ) -> Result<tonic::Response<super::MsgAddScopeOwnerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/AddScopeOwner");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteScopeOwner removes data access AccAddress from scope
        pub async fn delete_scope_owner(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteScopeOwnerRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteScopeOwnerResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/DeleteScopeOwner",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// WriteSession adds or updates a session context.
        pub async fn write_session(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWriteSessionRequest>,
        ) -> Result<tonic::Response<super::MsgWriteSessionResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/WriteSession");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// WriteRecord adds or updates a record.
        pub async fn write_record(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWriteRecordRequest>,
        ) -> Result<tonic::Response<super::MsgWriteRecordResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/WriteRecord");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteRecord deletes a record.
        pub async fn delete_record(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteRecordRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteRecordResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/DeleteRecord");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// WriteScopeSpecification adds or updates a scope specification.
        pub async fn write_scope_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWriteScopeSpecificationRequest>,
        ) -> Result<tonic::Response<super::MsgWriteScopeSpecificationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/WriteScopeSpecification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteScopeSpecification deletes a scope specification.
        pub async fn delete_scope_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteScopeSpecificationRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteScopeSpecificationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/DeleteScopeSpecification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// WriteContractSpecification adds or updates a contract specification.
        pub async fn write_contract_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWriteContractSpecificationRequest>,
        ) -> Result<tonic::Response<super::MsgWriteContractSpecificationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/WriteContractSpecification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteContractSpecification deletes a contract specification.
        pub async fn delete_contract_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteContractSpecificationRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteContractSpecificationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/DeleteContractSpecification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// AddContractSpecToScopeSpec adds contract specification to a scope specification.
        pub async fn add_contract_spec_to_scope_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAddContractSpecToScopeSpecRequest>,
        ) -> Result<tonic::Response<super::MsgAddContractSpecToScopeSpecResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/AddContractSpecToScopeSpec",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteContractSpecFromScopeSpec deletes a contract specification from a scope specification.
        pub async fn delete_contract_spec_from_scope_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteContractSpecFromScopeSpecRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteContractSpecFromScopeSpecResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/DeleteContractSpecFromScopeSpec",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// WriteRecordSpecification adds or updates a record specification.
        pub async fn write_record_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWriteRecordSpecificationRequest>,
        ) -> Result<tonic::Response<super::MsgWriteRecordSpecificationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/WriteRecordSpecification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteRecordSpecification deletes a record specification.
        pub async fn delete_record_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteRecordSpecificationRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteRecordSpecificationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/DeleteRecordSpecification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// WriteP8eContractSpec adds a P8e v39 contract spec as a v40 ContractSpecification
        /// It only exists to help facilitate the transition. Users should transition to WriteContractSpecification.
        pub async fn write_p8e_contract_spec(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWriteP8eContractSpecRequest>,
        ) -> Result<tonic::Response<super::MsgWriteP8eContractSpecResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/WriteP8eContractSpec",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// P8EMemorializeContract records the results of a P8e contract execution as a session and set of records in a scope
        /// It only exists to help facilitate the transition. Users should transition to calling the individual Write methods.
        pub async fn p8e_memorialize_contract(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgP8eMemorializeContractRequest>,
        ) -> Result<tonic::Response<super::MsgP8eMemorializeContractResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Msg/P8eMemorializeContract",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// BindOSLocator binds an owner address to a uri.
        pub async fn bind_os_locator(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgBindOsLocatorRequest>,
        ) -> Result<tonic::Response<super::MsgBindOsLocatorResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/BindOSLocator");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// DeleteOSLocator deletes an existing ObjectStoreLocator record.
        pub async fn delete_os_locator(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDeleteOsLocatorRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteOsLocatorResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/DeleteOSLocator");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ModifyOSLocator updates an ObjectStoreLocator record by the current owner.
        pub async fn modify_os_locator(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgModifyOsLocatorRequest>,
        ) -> Result<tonic::Response<super::MsgModifyOsLocatorResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Msg/ModifyOSLocator");
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
        /// WriteScope adds or updates a scope.
        async fn write_scope(
            &self,
            request: tonic::Request<super::MsgWriteScopeRequest>,
        ) -> Result<tonic::Response<super::MsgWriteScopeResponse>, tonic::Status>;
        /// DeleteScope deletes a scope and all associated Records, Sessions.
        async fn delete_scope(
            &self,
            request: tonic::Request<super::MsgDeleteScopeRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteScopeResponse>, tonic::Status>;
        /// AddScopeDataAccess adds data access AccAddress to scope
        async fn add_scope_data_access(
            &self,
            request: tonic::Request<super::MsgAddScopeDataAccessRequest>,
        ) -> Result<tonic::Response<super::MsgAddScopeDataAccessResponse>, tonic::Status>;
        /// DeleteScopeDataAccess removes data access AccAddress from scope
        async fn delete_scope_data_access(
            &self,
            request: tonic::Request<super::MsgDeleteScopeDataAccessRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteScopeDataAccessResponse>, tonic::Status>;
        /// AddScopeOwner adds new owner AccAddress to scope
        async fn add_scope_owner(
            &self,
            request: tonic::Request<super::MsgAddScopeOwnerRequest>,
        ) -> Result<tonic::Response<super::MsgAddScopeOwnerResponse>, tonic::Status>;
        /// DeleteScopeOwner removes data access AccAddress from scope
        async fn delete_scope_owner(
            &self,
            request: tonic::Request<super::MsgDeleteScopeOwnerRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteScopeOwnerResponse>, tonic::Status>;
        /// WriteSession adds or updates a session context.
        async fn write_session(
            &self,
            request: tonic::Request<super::MsgWriteSessionRequest>,
        ) -> Result<tonic::Response<super::MsgWriteSessionResponse>, tonic::Status>;
        /// WriteRecord adds or updates a record.
        async fn write_record(
            &self,
            request: tonic::Request<super::MsgWriteRecordRequest>,
        ) -> Result<tonic::Response<super::MsgWriteRecordResponse>, tonic::Status>;
        /// DeleteRecord deletes a record.
        async fn delete_record(
            &self,
            request: tonic::Request<super::MsgDeleteRecordRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteRecordResponse>, tonic::Status>;
        /// WriteScopeSpecification adds or updates a scope specification.
        async fn write_scope_specification(
            &self,
            request: tonic::Request<super::MsgWriteScopeSpecificationRequest>,
        ) -> Result<tonic::Response<super::MsgWriteScopeSpecificationResponse>, tonic::Status>;
        /// DeleteScopeSpecification deletes a scope specification.
        async fn delete_scope_specification(
            &self,
            request: tonic::Request<super::MsgDeleteScopeSpecificationRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteScopeSpecificationResponse>, tonic::Status>;
        /// WriteContractSpecification adds or updates a contract specification.
        async fn write_contract_specification(
            &self,
            request: tonic::Request<super::MsgWriteContractSpecificationRequest>,
        ) -> Result<tonic::Response<super::MsgWriteContractSpecificationResponse>, tonic::Status>;
        /// DeleteContractSpecification deletes a contract specification.
        async fn delete_contract_specification(
            &self,
            request: tonic::Request<super::MsgDeleteContractSpecificationRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteContractSpecificationResponse>, tonic::Status>;
        /// AddContractSpecToScopeSpec adds contract specification to a scope specification.
        async fn add_contract_spec_to_scope_spec(
            &self,
            request: tonic::Request<super::MsgAddContractSpecToScopeSpecRequest>,
        ) -> Result<tonic::Response<super::MsgAddContractSpecToScopeSpecResponse>, tonic::Status>;
        /// DeleteContractSpecFromScopeSpec deletes a contract specification from a scope specification.
        async fn delete_contract_spec_from_scope_spec(
            &self,
            request: tonic::Request<super::MsgDeleteContractSpecFromScopeSpecRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteContractSpecFromScopeSpecResponse>, tonic::Status>;
        /// WriteRecordSpecification adds or updates a record specification.
        async fn write_record_specification(
            &self,
            request: tonic::Request<super::MsgWriteRecordSpecificationRequest>,
        ) -> Result<tonic::Response<super::MsgWriteRecordSpecificationResponse>, tonic::Status>;
        /// DeleteRecordSpecification deletes a record specification.
        async fn delete_record_specification(
            &self,
            request: tonic::Request<super::MsgDeleteRecordSpecificationRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteRecordSpecificationResponse>, tonic::Status>;
        /// WriteP8eContractSpec adds a P8e v39 contract spec as a v40 ContractSpecification
        /// It only exists to help facilitate the transition. Users should transition to WriteContractSpecification.
        async fn write_p8e_contract_spec(
            &self,
            request: tonic::Request<super::MsgWriteP8eContractSpecRequest>,
        ) -> Result<tonic::Response<super::MsgWriteP8eContractSpecResponse>, tonic::Status>;
        /// P8EMemorializeContract records the results of a P8e contract execution as a session and set of records in a scope
        /// It only exists to help facilitate the transition. Users should transition to calling the individual Write methods.
        async fn p8e_memorialize_contract(
            &self,
            request: tonic::Request<super::MsgP8eMemorializeContractRequest>,
        ) -> Result<tonic::Response<super::MsgP8eMemorializeContractResponse>, tonic::Status>;
        /// BindOSLocator binds an owner address to a uri.
        async fn bind_os_locator(
            &self,
            request: tonic::Request<super::MsgBindOsLocatorRequest>,
        ) -> Result<tonic::Response<super::MsgBindOsLocatorResponse>, tonic::Status>;
        /// DeleteOSLocator deletes an existing ObjectStoreLocator record.
        async fn delete_os_locator(
            &self,
            request: tonic::Request<super::MsgDeleteOsLocatorRequest>,
        ) -> Result<tonic::Response<super::MsgDeleteOsLocatorResponse>, tonic::Status>;
        /// ModifyOSLocator updates an ObjectStoreLocator record by the current owner.
        async fn modify_os_locator(
            &self,
            request: tonic::Request<super::MsgModifyOsLocatorRequest>,
        ) -> Result<tonic::Response<super::MsgModifyOsLocatorResponse>, tonic::Status>;
    }
    /// Msg defines the Metadata Msg service.
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
                "/provenance.metadata.v1.Msg/WriteScope" => {
                    #[allow(non_camel_case_types)]
                    struct WriteScopeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWriteScopeRequest> for WriteScopeSvc<T> {
                        type Response = super::MsgWriteScopeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWriteScopeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).write_scope(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WriteScopeSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteScope" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteScopeSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteScopeRequest> for DeleteScopeSvc<T> {
                        type Response = super::MsgDeleteScopeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteScopeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_scope(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteScopeSvc(inner);
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
                "/provenance.metadata.v1.Msg/AddScopeDataAccess" => {
                    #[allow(non_camel_case_types)]
                    struct AddScopeDataAccessSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAddScopeDataAccessRequest>
                        for AddScopeDataAccessSvc<T>
                    {
                        type Response = super::MsgAddScopeDataAccessResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddScopeDataAccessRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_scope_data_access(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddScopeDataAccessSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteScopeDataAccess" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteScopeDataAccessSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteScopeDataAccessRequest>
                        for DeleteScopeDataAccessSvc<T>
                    {
                        type Response = super::MsgDeleteScopeDataAccessResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteScopeDataAccessRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).delete_scope_data_access(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteScopeDataAccessSvc(inner);
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
                "/provenance.metadata.v1.Msg/AddScopeOwner" => {
                    #[allow(non_camel_case_types)]
                    struct AddScopeOwnerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgAddScopeOwnerRequest> for AddScopeOwnerSvc<T> {
                        type Response = super::MsgAddScopeOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddScopeOwnerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).add_scope_owner(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddScopeOwnerSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteScopeOwner" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteScopeOwnerSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteScopeOwnerRequest>
                        for DeleteScopeOwnerSvc<T>
                    {
                        type Response = super::MsgDeleteScopeOwnerResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteScopeOwnerRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_scope_owner(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteScopeOwnerSvc(inner);
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
                "/provenance.metadata.v1.Msg/WriteSession" => {
                    #[allow(non_camel_case_types)]
                    struct WriteSessionSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWriteSessionRequest> for WriteSessionSvc<T> {
                        type Response = super::MsgWriteSessionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWriteSessionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).write_session(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WriteSessionSvc(inner);
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
                "/provenance.metadata.v1.Msg/WriteRecord" => {
                    #[allow(non_camel_case_types)]
                    struct WriteRecordSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWriteRecordRequest> for WriteRecordSvc<T> {
                        type Response = super::MsgWriteRecordResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWriteRecordRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).write_record(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WriteRecordSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteRecord" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRecordSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteRecordRequest> for DeleteRecordSvc<T> {
                        type Response = super::MsgDeleteRecordResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteRecordRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_record(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteRecordSvc(inner);
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
                "/provenance.metadata.v1.Msg/WriteScopeSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct WriteScopeSpecificationSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgWriteScopeSpecificationRequest>
                        for WriteScopeSpecificationSvc<T>
                    {
                        type Response = super::MsgWriteScopeSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWriteScopeSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).write_scope_specification(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WriteScopeSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteScopeSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteScopeSpecificationSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgDeleteScopeSpecificationRequest>
                        for DeleteScopeSpecificationSvc<T>
                    {
                        type Response = super::MsgDeleteScopeSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteScopeSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).delete_scope_specification(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteScopeSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Msg/WriteContractSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct WriteContractSpecificationSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgWriteContractSpecificationRequest>
                        for WriteContractSpecificationSvc<T>
                    {
                        type Response = super::MsgWriteContractSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWriteContractSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).write_contract_specification(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WriteContractSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteContractSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteContractSpecificationSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgDeleteContractSpecificationRequest>
                        for DeleteContractSpecificationSvc<T>
                    {
                        type Response = super::MsgDeleteContractSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteContractSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_contract_specification(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteContractSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Msg/AddContractSpecToScopeSpec" => {
                    #[allow(non_camel_case_types)]
                    struct AddContractSpecToScopeSpecSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgAddContractSpecToScopeSpecRequest>
                        for AddContractSpecToScopeSpecSvc<T>
                    {
                        type Response = super::MsgAddContractSpecToScopeSpecResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgAddContractSpecToScopeSpecRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).add_contract_spec_to_scope_spec(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddContractSpecToScopeSpecSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteContractSpecFromScopeSpec" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteContractSpecFromScopeSpecSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<
                            super::MsgDeleteContractSpecFromScopeSpecRequest,
                        > for DeleteContractSpecFromScopeSpecSvc<T>
                    {
                        type Response = super::MsgDeleteContractSpecFromScopeSpecResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::MsgDeleteContractSpecFromScopeSpecRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_contract_spec_from_scope_spec(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteContractSpecFromScopeSpecSvc(inner);
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
                "/provenance.metadata.v1.Msg/WriteRecordSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct WriteRecordSpecificationSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgWriteRecordSpecificationRequest>
                        for WriteRecordSpecificationSvc<T>
                    {
                        type Response = super::MsgWriteRecordSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWriteRecordSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).write_record_specification(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WriteRecordSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteRecordSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRecordSpecificationSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgDeleteRecordSpecificationRequest>
                        for DeleteRecordSpecificationSvc<T>
                    {
                        type Response = super::MsgDeleteRecordSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteRecordSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).delete_record_specification(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteRecordSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Msg/WriteP8eContractSpec" => {
                    #[allow(non_camel_case_types)]
                    struct WriteP8eContractSpecSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWriteP8eContractSpecRequest>
                        for WriteP8eContractSpecSvc<T>
                    {
                        type Response = super::MsgWriteP8eContractSpecResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWriteP8eContractSpecRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).write_p8e_contract_spec(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = WriteP8eContractSpecSvc(inner);
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
                "/provenance.metadata.v1.Msg/P8eMemorializeContract" => {
                    #[allow(non_camel_case_types)]
                    struct P8eMemorializeContractSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg>
                        tonic::server::UnaryService<super::MsgP8eMemorializeContractRequest>
                        for P8eMemorializeContractSvc<T>
                    {
                        type Response = super::MsgP8eMemorializeContractResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgP8eMemorializeContractRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).p8e_memorialize_contract(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = P8eMemorializeContractSvc(inner);
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
                "/provenance.metadata.v1.Msg/BindOSLocator" => {
                    #[allow(non_camel_case_types)]
                    struct BindOSLocatorSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgBindOsLocatorRequest> for BindOSLocatorSvc<T> {
                        type Response = super::MsgBindOsLocatorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgBindOsLocatorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).bind_os_locator(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BindOSLocatorSvc(inner);
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
                "/provenance.metadata.v1.Msg/DeleteOSLocator" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteOSLocatorSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDeleteOsLocatorRequest>
                        for DeleteOSLocatorSvc<T>
                    {
                        type Response = super::MsgDeleteOsLocatorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDeleteOsLocatorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete_os_locator(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteOSLocatorSvc(inner);
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
                "/provenance.metadata.v1.Msg/ModifyOSLocator" => {
                    #[allow(non_camel_case_types)]
                    struct ModifyOSLocatorSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgModifyOsLocatorRequest>
                        for ModifyOSLocatorSvc<T>
                    {
                        type Response = super::MsgModifyOsLocatorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgModifyOsLocatorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).modify_os_locator(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ModifyOSLocatorSvc(inner);
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
        const NAME: &'static str = "provenance.metadata.v1.Msg";
    }
}
/// EventTxCompleted is an event message indicating that a TX has completed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventTxCompleted {
    /// module is the module the TX belongs to.
    #[prost(string, tag = "1")]
    pub module: ::prost::alloc::string::String,
    /// endpoint is the rpc endpoint that was just completed.
    #[prost(string, tag = "2")]
    pub endpoint: ::prost::alloc::string::String,
    /// signers are the bech32 address strings of the signers of this TX.
    #[prost(string, repeated, tag = "3")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// EventScopeCreated is an event message indicating a scope has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventScopeCreated {
    /// scope_addr is the bech32 address string of the scope id that was created.
    #[prost(string, tag = "1")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventScopeUpdated is an event message indicating a scope has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventScopeUpdated {
    /// scope_addr is the bech32 address string of the scope id that was updated.
    #[prost(string, tag = "1")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventScopeDeleted is an event message indicating a scope has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventScopeDeleted {
    /// scope_addr is the bech32 address string of the scope id that was deleted.
    #[prost(string, tag = "1")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventSessionCreated is an event message indicating a session has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSessionCreated {
    /// session_addr is the bech32 address string of the session id that was created.
    #[prost(string, tag = "1")]
    pub session_addr: ::prost::alloc::string::String,
    /// scope_addr is the bech32 address string of the scope id this session belongs to.
    #[prost(string, tag = "2")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventSessionUpdated is an event message indicating a session has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSessionUpdated {
    /// session_addr is the bech32 address string of the session id that was updated.
    #[prost(string, tag = "1")]
    pub session_addr: ::prost::alloc::string::String,
    /// scope_addr is the bech32 address string of the scope id this session belongs to.
    #[prost(string, tag = "2")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventSessionDeleted is an event message indicating a session has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSessionDeleted {
    /// session_addr is the bech32 address string of the session id that was deleted.
    #[prost(string, tag = "1")]
    pub session_addr: ::prost::alloc::string::String,
    /// scope_addr is the bech32 address string of the scope id this session belongs to.
    #[prost(string, tag = "2")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventRecordCreated is an event message indicating a record has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRecordCreated {
    /// record_addr is the bech32 address string of the record id that was created.
    #[prost(string, tag = "1")]
    pub record_addr: ::prost::alloc::string::String,
    /// session_addr is the bech32 address string of the session id this record belongs to.
    #[prost(string, tag = "2")]
    pub session_addr: ::prost::alloc::string::String,
    /// scope_addr is the bech32 address string of the scope id this record belongs to.
    #[prost(string, tag = "3")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventRecordUpdated is an event message indicating a record has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRecordUpdated {
    /// record_addr is the bech32 address string of the record id that was updated.
    #[prost(string, tag = "1")]
    pub record_addr: ::prost::alloc::string::String,
    /// session_addr is the bech32 address string of the session id this record belongs to.
    #[prost(string, tag = "2")]
    pub session_addr: ::prost::alloc::string::String,
    /// scope_addr is the bech32 address string of the scope id this record belongs to.
    #[prost(string, tag = "3")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventRecordDeleted is an event message indicating a record has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRecordDeleted {
    /// record is the bech32 address string of the record id that was deleted.
    #[prost(string, tag = "1")]
    pub record_addr: ::prost::alloc::string::String,
    /// scope_addr is the bech32 address string of the scope id this record belonged to.
    #[prost(string, tag = "3")]
    pub scope_addr: ::prost::alloc::string::String,
}
/// EventScopeSpecificationCreated is an event message indicating a scope specification has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventScopeSpecificationCreated {
    /// scope_specification_addr is the bech32 address string of the specification id of the scope specification that was
    /// created.
    #[prost(string, tag = "1")]
    pub scope_specification_addr: ::prost::alloc::string::String,
}
/// EventScopeSpecificationUpdated is an event message indicating a scope specification has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventScopeSpecificationUpdated {
    /// scope_specification_addr is the bech32 address string of the specification id of the scope specification that was
    /// updated.
    #[prost(string, tag = "1")]
    pub scope_specification_addr: ::prost::alloc::string::String,
}
/// EventScopeSpecificationDeleted is an event message indicating a scope specification has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventScopeSpecificationDeleted {
    /// scope_specification_addr is the bech32 address string of the specification id of the scope specification that was
    /// deleted.
    #[prost(string, tag = "1")]
    pub scope_specification_addr: ::prost::alloc::string::String,
}
/// EventContractSpecificationCreated is an event message indicating a contract specification has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventContractSpecificationCreated {
    /// contract_specification_addr is the bech32 address string of the specification id of the contract specification that
    /// was created.
    #[prost(string, tag = "1")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
/// EventContractSpecificationUpdated is an event message indicating a contract specification has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventContractSpecificationUpdated {
    /// contract_specification_addr is the bech32 address string of the specification id of the contract specification that
    /// was updated.
    #[prost(string, tag = "1")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
/// EventContractSpecificationDeleted is an event message indicating a contract specification has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventContractSpecificationDeleted {
    /// contract_specification_addr is the bech32 address string of the specification id of the contract specification that
    /// was deleted.
    #[prost(string, tag = "1")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
/// EventRecordSpecificationCreated is an event message indicating a record specification has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRecordSpecificationCreated {
    /// record_specification_addr is the bech32 address string of the specification id of the record specification that was
    /// created.
    #[prost(string, tag = "1")]
    pub record_specification_addr: ::prost::alloc::string::String,
    /// contract_specification_addr is the bech32 address string of the contract specification id this record specification
    /// belongs to.
    #[prost(string, tag = "2")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
/// EventRecordSpecificationUpdated is an event message indicating a record specification has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRecordSpecificationUpdated {
    /// record_specification_addr is the bech32 address string of the specification id of the record specification that was
    /// updated.
    #[prost(string, tag = "1")]
    pub record_specification_addr: ::prost::alloc::string::String,
    /// contract_specification_addr is the bech32 address string of the contract specification id this record specification
    /// belongs to.
    #[prost(string, tag = "2")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
/// EventRecordSpecificationDeleted is an event message indicating a record specification has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventRecordSpecificationDeleted {
    /// record_specification_addr is the bech32 address string of the specification id of the record specification that was
    /// deleted.
    #[prost(string, tag = "1")]
    pub record_specification_addr: ::prost::alloc::string::String,
    /// contract_specification_addr is the bech32 address string of the contract specification id this record specification
    /// belongs to.
    #[prost(string, tag = "2")]
    pub contract_specification_addr: ::prost::alloc::string::String,
}
/// EventOSLocatorCreated is an event message indicating an object store locator has been created.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventOsLocatorCreated {
    /// owner is the owner in the object store locator that was created.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
/// EventOSLocatorUpdated is an event message indicating an object store locator has been updated.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventOsLocatorUpdated {
    /// owner is the owner in the object store locator that was updated.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
/// EventOSLocatorDeleted is an event message indicating an object store locator has been deleted.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventOsLocatorDeleted {
    /// owner is the owner in the object store locator that was deleted.
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
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
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<QueryParamsRequest>,
}
/// ScopeRequest is the request type for the Query/Scope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeRequest {
    /// scope_id can either be a uuid, e.g. 91978ba2-5f35-459a-86a7-feca1b0512e0 or a bech32 scope address, e.g.
    /// scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel.
    #[prost(string, tag = "1")]
    pub scope_id: ::prost::alloc::string::String,
    /// session_addr is a bech32 session address, e.g.
    /// session1qxge0zaztu65tx5x5llv5xc9zts9sqlch3sxwn44j50jzgt8rshvqyfrjcr.
    #[prost(string, tag = "2")]
    pub session_addr: ::prost::alloc::string::String,
    /// record_addr is a bech32 record address, e.g. record1q2ge0zaztu65tx5x5llv5xc9ztsw42dq2jdvmdazuwzcaddhh8gmu3mcze3.
    #[prost(string, tag = "3")]
    pub record_addr: ::prost::alloc::string::String,
    /// include_sessions is a flag for whether or not the sessions in the scope should be included.
    #[prost(bool, tag = "10")]
    pub include_sessions: bool,
    /// include_records is a flag for whether or not the records in the scope should be included.
    #[prost(bool, tag = "11")]
    pub include_records: bool,
}
/// ScopeResponse is the response type for the Query/Scope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeResponse {
    /// scope is the wrapped scope result.
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<ScopeWrapper>,
    /// sessions is any number of wrapped sessions in this scope (if requested).
    #[prost(message, repeated, tag = "2")]
    pub sessions: ::prost::alloc::vec::Vec<SessionWrapper>,
    /// records is any number of wrapped records in this scope (if requested).
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<RecordWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ScopeRequest>,
}
/// SessionWrapper contains a single scope and its uuid.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeWrapper {
    /// scope is the on-chain scope message.
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<Scope>,
    /// scope_id_info contains information about the id/address of the scope.
    #[prost(message, optional, tag = "2")]
    pub scope_id_info: ::core::option::Option<ScopeIdInfo>,
    /// scope_spec_id_info contains information about the id/address of the scope specification.
    #[prost(message, optional, tag = "3")]
    pub scope_spec_id_info: ::core::option::Option<ScopeSpecIdInfo>,
}
/// ScopesAllRequest is the request type for the Query/ScopesAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopesAllRequest {
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// ScopesAllResponse is the response type for the Query/ScopesAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopesAllResponse {
    /// scopes are the wrapped scopes.
    #[prost(message, repeated, tag = "1")]
    pub scopes: ::prost::alloc::vec::Vec<ScopeWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ScopesAllRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// SessionsRequest is the request type for the Query/Sessions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionsRequest {
    /// scope_id can either be a uuid, e.g. 91978ba2-5f35-459a-86a7-feca1b0512e0 or a bech32 scope address, e.g.
    /// scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel.
    #[prost(string, tag = "1")]
    pub scope_id: ::prost::alloc::string::String,
    /// session_id can either be a uuid, e.g. 5803f8bc-6067-4eb5-951f-2121671c2ec0 or a bech32 session address, e.g.
    /// session1qxge0zaztu65tx5x5llv5xc9zts9sqlch3sxwn44j50jzgt8rshvqyfrjcr. This can only be a uuid if a scope_id is also
    /// provided.
    #[prost(string, tag = "2")]
    pub session_id: ::prost::alloc::string::String,
    /// record_addr is a bech32 record address, e.g. record1q2ge0zaztu65tx5x5llv5xc9ztsw42dq2jdvmdazuwzcaddhh8gmu3mcze3.
    #[prost(string, tag = "3")]
    pub record_addr: ::prost::alloc::string::String,
    /// record_name is the name of the record to find the session for in the provided scope.
    #[prost(string, tag = "4")]
    pub record_name: ::prost::alloc::string::String,
    /// include_scope is a flag for whether or not the scope containing these sessions should be included.
    #[prost(bool, tag = "10")]
    pub include_scope: bool,
    /// include_records is a flag for whether or not the records in these sessions should be included.
    #[prost(bool, tag = "11")]
    pub include_records: bool,
}
/// SessionsResponse is the response type for the Query/Sessions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionsResponse {
    /// scope is the wrapped scope that holds these sessions (if requested).
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<ScopeWrapper>,
    /// sessions is any number of wrapped session results.
    #[prost(message, repeated, tag = "2")]
    pub sessions: ::prost::alloc::vec::Vec<SessionWrapper>,
    /// records is any number of wrapped records contained in these sessions (if requested).
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<RecordWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<SessionsRequest>,
}
/// SessionWrapper contains a single session and some extra identifiers for it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionWrapper {
    /// session is the on-chain session message.
    #[prost(message, optional, tag = "1")]
    pub session: ::core::option::Option<Session>,
    /// session_id_info contains information about the id/address of the session.
    #[prost(message, optional, tag = "2")]
    pub session_id_info: ::core::option::Option<SessionIdInfo>,
    /// contract_spec_id_info contains information about the id/address of the contract specification.
    #[prost(message, optional, tag = "3")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
}
/// SessionsAllRequest is the request type for the Query/SessionsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionsAllRequest {
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// SessionsAllResponse is the response type for the Query/SessionsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionsAllResponse {
    /// sessions are the wrapped sessions.
    #[prost(message, repeated, tag = "1")]
    pub sessions: ::prost::alloc::vec::Vec<SessionWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<SessionsAllRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// RecordsRequest is the request type for the Query/Records RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordsRequest {
    /// record_addr is a bech32 record address, e.g. record1q2ge0zaztu65tx5x5llv5xc9ztsw42dq2jdvmdazuwzcaddhh8gmu3mcze3.
    #[prost(string, tag = "1")]
    pub record_addr: ::prost::alloc::string::String,
    /// scope_id can either be a uuid, e.g. 91978ba2-5f35-459a-86a7-feca1b0512e0 or a bech32 scope address, e.g.
    /// scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel.
    #[prost(string, tag = "2")]
    pub scope_id: ::prost::alloc::string::String,
    /// session_id can either be a uuid, e.g. 5803f8bc-6067-4eb5-951f-2121671c2ec0 or a bech32 session address, e.g.
    /// session1qxge0zaztu65tx5x5llv5xc9zts9sqlch3sxwn44j50jzgt8rshvqyfrjcr. This can only be a uuid if a scope_id is also
    /// provided.
    #[prost(string, tag = "3")]
    pub session_id: ::prost::alloc::string::String,
    /// name is the name of the record to look for
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    /// include_scope is a flag for whether or not the scope containing these records should be included.
    #[prost(bool, tag = "10")]
    pub include_scope: bool,
    /// include_sessions is a flag for whether or not the sessions containing these records should be included.
    #[prost(bool, tag = "11")]
    pub include_sessions: bool,
}
/// RecordsResponse is the response type for the Query/Records RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordsResponse {
    /// scope is the wrapped scope that holds these records (if requested).
    #[prost(message, optional, tag = "1")]
    pub scope: ::core::option::Option<ScopeWrapper>,
    /// sessions is any number of wrapped sessions that hold these records (if requested).
    #[prost(message, repeated, tag = "2")]
    pub sessions: ::prost::alloc::vec::Vec<SessionWrapper>,
    /// records is any number of wrapped record results.
    #[prost(message, repeated, tag = "3")]
    pub records: ::prost::alloc::vec::Vec<RecordWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<RecordsRequest>,
}
/// RecordWrapper contains a single record and some extra identifiers for it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordWrapper {
    /// record is the on-chain record message.
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<Record>,
    /// record_id_info contains information about the id/address of the record.
    #[prost(message, optional, tag = "2")]
    pub record_id_info: ::core::option::Option<RecordIdInfo>,
    /// record_spec_id_info contains information about the id/address of the record specification.
    #[prost(message, optional, tag = "3")]
    pub record_spec_id_info: ::core::option::Option<RecordSpecIdInfo>,
}
/// RecordsAllRequest is the request type for the Query/RecordsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordsAllRequest {
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// RecordsAllResponse is the response type for the Query/RecordsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordsAllResponse {
    /// records are the wrapped records.
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<RecordWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<RecordsAllRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// OwnershipRequest is the request type for the Query/Ownership RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnershipRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// OwnershipResponse is the response type for the Query/Ownership RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OwnershipResponse {
    /// A list of scope ids (uuid) associated with the given address.
    #[prost(string, repeated, tag = "1")]
    pub scope_uuids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OwnershipRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// ValueOwnershipRequest is the request type for the Query/ValueOwnership RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueOwnershipRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// ValueOwnershipResponse is the response type for the Query/ValueOwnership RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueOwnershipResponse {
    /// A list of scope ids (uuid) associated with the given address.
    #[prost(string, repeated, tag = "1")]
    pub scope_uuids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ValueOwnershipRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// ScopeSpecificationRequest is the request type for the Query/ScopeSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeSpecificationRequest {
    /// specification_id can either be a uuid, e.g. dc83ea70-eacd-40fe-9adf-1cf6148bf8a2 or a bech32 scope specification
    /// address, e.g. scopespec1qnwg86nsatx5pl56muw0v9ytlz3qu3jx6m.
    #[prost(string, tag = "1")]
    pub specification_id: ::prost::alloc::string::String,
}
/// ScopeSpecificationResponse is the response type for the Query/ScopeSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeSpecificationResponse {
    /// scope_specification is the wrapped scope specification.
    #[prost(message, optional, tag = "1")]
    pub scope_specification: ::core::option::Option<ScopeSpecificationWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ScopeSpecificationRequest>,
}
/// ScopeSpecificationWrapper contains a single scope specification and some extra identifiers for it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeSpecificationWrapper {
    /// specification is the on-chain scope specification message.
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<ScopeSpecification>,
    /// scope_spec_id_info contains information about the id/address of the scope specification.
    #[prost(message, optional, tag = "2")]
    pub scope_spec_id_info: ::core::option::Option<ScopeSpecIdInfo>,
}
/// ScopeSpecificationsAllRequest is the request type for the Query/ScopeSpecificationsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeSpecificationsAllRequest {
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// ScopeSpecificationsAllResponse is the response type for the Query/ScopeSpecificationsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScopeSpecificationsAllResponse {
    /// scope_specifications are the wrapped scope specifications.
    #[prost(message, repeated, tag = "1")]
    pub scope_specifications: ::prost::alloc::vec::Vec<ScopeSpecificationWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ScopeSpecificationsAllRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// ContractSpecificationRequest is the request type for the Query/ContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractSpecificationRequest {
    /// specification_id can either be a uuid, e.g. def6bc0a-c9dd-4874-948f-5206e6060a84 or a bech32 contract specification
    /// address, e.g. contractspec1q000d0q2e8w5say53afqdesxp2zqzkr4fn.
    /// It can also be a record specification address, e.g.
    /// recspec1qh00d0q2e8w5say53afqdesxp2zw42dq2jdvmdazuwzcaddhh8gmuqhez44.
    #[prost(string, tag = "1")]
    pub specification_id: ::prost::alloc::string::String,
    /// include_record_specs is a flag for whether or not the record specifications in this contract specification should
    /// be included in the result.
    #[prost(bool, tag = "10")]
    pub include_record_specs: bool,
}
/// ContractSpecificationResponse is the response type for the Query/ContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractSpecificationResponse {
    /// contract_specification is the wrapped contract specification.
    #[prost(message, optional, tag = "1")]
    pub contract_specification: ::core::option::Option<ContractSpecificationWrapper>,
    /// record_specifications is any number or wrapped record specifications associated with this contract_specification
    /// (if requested).
    #[prost(message, repeated, tag = "3")]
    pub record_specifications: ::prost::alloc::vec::Vec<RecordSpecificationWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ContractSpecificationRequest>,
}
/// ContractSpecificationWrapper contains a single contract specification and some extra identifiers for it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractSpecificationWrapper {
    /// specification is the on-chain contract specification message.
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<ContractSpecification>,
    /// contract_spec_id_info contains information about the id/address of the contract specification.
    #[prost(message, optional, tag = "2")]
    pub contract_spec_id_info: ::core::option::Option<ContractSpecIdInfo>,
}
/// ContractSpecificationsAllRequest is the request type for the Query/ContractSpecificationsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractSpecificationsAllRequest {
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// ContractSpecificationsAllResponse is the response type for the Query/ContractSpecificationsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContractSpecificationsAllResponse {
    /// contract_specifications are the wrapped contract specifications.
    #[prost(message, repeated, tag = "1")]
    pub contract_specifications: ::prost::alloc::vec::Vec<ContractSpecificationWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<ContractSpecificationsAllRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// RecordSpecificationsForContractSpecificationRequest is the request type for the
/// Query/RecordSpecificationsForContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordSpecificationsForContractSpecificationRequest {
    /// specification_id can either be a uuid, e.g. def6bc0a-c9dd-4874-948f-5206e6060a84 or a bech32 contract specification
    /// address, e.g. contractspec1q000d0q2e8w5say53afqdesxp2zqzkr4fn.
    /// It can also be a record specification address, e.g.
    /// recspec1qh00d0q2e8w5say53afqdesxp2zw42dq2jdvmdazuwzcaddhh8gmuqhez44.
    #[prost(string, tag = "1")]
    pub specification_id: ::prost::alloc::string::String,
}
/// RecordSpecificationsForContractSpecificationResponse is the response type for the
/// Query/RecordSpecificationsForContractSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordSpecificationsForContractSpecificationResponse {
    /// record_specifications is any number of wrapped record specifications associated with this contract_specification.
    #[prost(message, repeated, tag = "1")]
    pub record_specifications: ::prost::alloc::vec::Vec<RecordSpecificationWrapper>,
    /// contract_specification_uuid is the uuid of this contract specification.
    #[prost(string, tag = "2")]
    pub contract_specification_uuid: ::prost::alloc::string::String,
    /// contract_specification_addr is the contract specification address as a bech32 encoded string.
    #[prost(string, tag = "3")]
    pub contract_specification_addr: ::prost::alloc::string::String,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<RecordSpecificationsForContractSpecificationRequest>,
}
/// RecordSpecificationRequest is the request type for the Query/RecordSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordSpecificationRequest {
    /// specification_id can either be a uuid, e.g. def6bc0a-c9dd-4874-948f-5206e6060a84 or a bech32 contract specification
    /// address, e.g. contractspec1q000d0q2e8w5say53afqdesxp2zqzkr4fn.
    /// It can also be a record specification address, e.g.
    /// recspec1qh00d0q2e8w5say53afqdesxp2zw42dq2jdvmdazuwzcaddhh8gmuqhez44.
    #[prost(string, tag = "1")]
    pub specification_id: ::prost::alloc::string::String,
    /// name is the name of the record to look up.
    /// It is required if the specification_id is a uuid or contract specification address.
    /// It is ignored if the specification_id is a record specification address.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
/// RecordSpecificationResponse is the response type for the Query/RecordSpecification RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordSpecificationResponse {
    /// record_specification is the wrapped record specification.
    #[prost(message, optional, tag = "1")]
    pub record_specification: ::core::option::Option<RecordSpecificationWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<RecordSpecificationRequest>,
}
/// RecordSpecificationWrapper contains a single record specification and some extra identifiers for it.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordSpecificationWrapper {
    /// specification is the on-chain record specification message.
    #[prost(message, optional, tag = "1")]
    pub specification: ::core::option::Option<RecordSpecification>,
    /// record_spec_id_info contains information about the id/address of the record specification.
    #[prost(message, optional, tag = "2")]
    pub record_spec_id_info: ::core::option::Option<RecordSpecIdInfo>,
}
/// RecordSpecificationsAllRequest is the request type for the Query/RecordSpecificationsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordSpecificationsAllRequest {
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// RecordSpecificationsAllResponse is the response type for the Query/RecordSpecificationsAll RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecordSpecificationsAllResponse {
    /// record_specifications are the wrapped record specifications.
    #[prost(message, repeated, tag = "1")]
    pub record_specifications: ::prost::alloc::vec::Vec<RecordSpecificationWrapper>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<RecordSpecificationsAllRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// OSLocatorParamsRequest is the request type for the Query/OSLocatorParams RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsLocatorParamsRequest {}
/// OSLocatorParamsResponse is the response type for the Query/OSLocatorParams RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsLocatorParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<OsLocatorParams>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsLocatorParamsRequest>,
}
/// OSLocatorRequest is the request type for the Query/OSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsLocatorRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
/// OSLocatorResponse is the response type for the Query/OSLocator RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsLocatorResponse {
    #[prost(message, optional, tag = "1")]
    pub locator: ::core::option::Option<ObjectStoreLocator>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsLocatorRequest>,
}
/// OSLocatorsByURIRequest is the request type for the Query/OSLocatorsByURI RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsLocatorsByUriRequest {
    #[prost(string, tag = "1")]
    pub uri: ::prost::alloc::string::String,
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// OSLocatorsByURIResponse is the response type for the Query/OSLocatorsByURI RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsLocatorsByUriResponse {
    #[prost(message, repeated, tag = "1")]
    pub locators: ::prost::alloc::vec::Vec<ObjectStoreLocator>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsLocatorsByUriRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// OSLocatorsByScopeRequest is the request type for the Query/OSLocatorsByScope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsLocatorsByScopeRequest {
    #[prost(string, tag = "1")]
    pub scope_id: ::prost::alloc::string::String,
}
/// OSLocatorsByScopeResponse is the response type for the Query/OSLocatorsByScope RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsLocatorsByScopeResponse {
    #[prost(message, repeated, tag = "1")]
    pub locators: ::prost::alloc::vec::Vec<ObjectStoreLocator>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsLocatorsByScopeRequest>,
}
/// OSAllLocatorsRequest is the request type for the Query/OSAllLocators RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsAllLocatorsRequest {
    /// pagination defines optional pagination parameters for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// OSAllLocatorsResponse is the response type for the Query/OSAllLocators RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OsAllLocatorsResponse {
    #[prost(message, repeated, tag = "1")]
    pub locators: ::prost::alloc::vec::Vec<ObjectStoreLocator>,
    /// request is a copy of the request that generated these results.
    #[prost(message, optional, tag = "98")]
    pub request: ::core::option::Option<OsAllLocatorsRequest>,
    /// pagination provides the pagination information of this response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the Metadata Query service.
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
        /// Params queries the parameters of x/metadata module.
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
            let path = http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/Params");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Scope searches for a scope.
        ///
        /// The scope id, if provided, must either be scope uuid, e.g. 91978ba2-5f35-459a-86a7-feca1b0512e0 or a scope address,
        /// e.g. scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel. The session addr, if provided, must be a bech32 session address,
        /// e.g. session1qxge0zaztu65tx5x5llv5xc9zts9sqlch3sxwn44j50jzgt8rshvqyfrjcr. The record_addr, if provided, must be a
        /// bech32 record address, e.g. record1q2ge0zaztu65tx5x5llv5xc9ztsw42dq2jdvmdazuwzcaddhh8gmu3mcze3.
        ///
        /// * If only a scope_id is provided, that scope is returned.
        /// * If only a session_addr is provided, the scope containing that session is returned.
        /// * If only a record_addr is provided, the scope containing that record is returned.
        /// * If more than one of scope_id, session_addr, and record_addr are provided, and they don't refer to the same scope,
        /// a bad request is returned.
        ///
        /// Providing a session addr or record addr does not limit the sessions and records returned (if requested).
        /// Those parameters are only used to find the scope.
        ///
        /// By default, sessions and records are not included.
        /// Set include_sessions and/or include_records to true to include sessions and/or records.
        pub async fn scope(
            &mut self,
            request: impl tonic::IntoRequest<super::ScopeRequest>,
        ) -> Result<tonic::Response<super::ScopeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/Scope");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ScopesAll retrieves all scopes.
        pub async fn scopes_all(
            &mut self,
            request: impl tonic::IntoRequest<super::ScopesAllRequest>,
        ) -> Result<tonic::Response<super::ScopesAllResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/ScopesAll");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Sessions searches for sessions.
        ///
        /// The scope_id can either be scope uuid, e.g. 91978ba2-5f35-459a-86a7-feca1b0512e0 or a scope address, e.g.
        /// scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel. Similarly, the session_id can either be a uuid or session address, e.g.
        /// session1qxge0zaztu65tx5x5llv5xc9zts9sqlch3sxwn44j50jzgt8rshvqyfrjcr. The record_addr, if provided, must be a
        /// bech32 record address, e.g. record1q2ge0zaztu65tx5x5llv5xc9ztsw42dq2jdvmdazuwzcaddhh8gmu3mcze3.
        ///
        /// * If only a scope_id is provided, all sessions in that scope are returned.
        /// * If only a session_id is provided, it must be an address, and that single session is returned.
        /// * If the session_id is a uuid, then either a scope_id or record_addr must also be provided, and that single session
        /// is returned.
        /// * If only a record_addr is provided, the session containing that record will be returned.
        /// * If a record_name is provided then either a scope_id, session_id as an address, or record_addr must also be
        /// provided, and the session containing that record will be returned.
        ///
        /// A bad request is returned if:
        /// * The session_id is a uuid and is provided without a scope_id or record_addr.
        /// * A record_name is provided without any way to identify the scope (e.g. a scope_id, a session_id as an address, or
        /// a record_addr).
        /// * Two or more of scope_id, session_id as an address, and record_addr are provided and don't all refer to the same
        /// scope.
        /// * A record_addr (or scope_id and record_name) is provided with a session_id and that session does not contain such
        /// a record.
        /// * A record_addr and record_name are both provided, but reference different records.
        ///
        /// By default, the scope and records are not included.
        /// Set include_scope and/or include_records to true to include the scope and/or records.
        pub async fn sessions(
            &mut self,
            request: impl tonic::IntoRequest<super::SessionsRequest>,
        ) -> Result<tonic::Response<super::SessionsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/Sessions");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// SessionsAll retrieves all sessions.
        pub async fn sessions_all(
            &mut self,
            request: impl tonic::IntoRequest<super::SessionsAllRequest>,
        ) -> Result<tonic::Response<super::SessionsAllResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/SessionsAll");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Records searches for records.
        ///
        /// The record_addr, if provided, must be a bech32 record address, e.g.
        /// record1q2ge0zaztu65tx5x5llv5xc9ztsw42dq2jdvmdazuwzcaddhh8gmu3mcze3. The scope-id can either be scope uuid, e.g.
        /// 91978ba2-5f35-459a-86a7-feca1b0512e0 or a scope address, e.g. scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel. Similarly,
        /// the session_id can either be a uuid or session address, e.g.
        /// session1qxge0zaztu65tx5x5llv5xc9zts9sqlch3sxwn44j50jzgt8rshvqyfrjcr. The name is the name of the record you're
        /// interested in.
        ///
        /// * If only a record_addr is provided, that single record will be returned.
        /// * If only a scope_id is provided, all records in that scope will be returned.
        /// * If only a session_id (or scope_id/session_id), all records in that session will be returned.
        /// * If a name is provided with a scope_id and/or session_id, that single record will be returned.
        ///
        /// A bad request is returned if:
        /// * The session_id is a uuid and no scope_id is provided.
        /// * There are two or more of record_addr, session_id, and scope_id, and they don't all refer to the same scope.
        /// * A name is provided, but not a scope_id and/or a session_id.
        /// * A name and record_addr are provided and the name doesn't match the record_addr.
        ///
        /// By default, the scope and sessions are not included.
        /// Set include_scope and/or include_sessions to true to include the scope and/or sessions.
        pub async fn records(
            &mut self,
            request: impl tonic::IntoRequest<super::RecordsRequest>,
        ) -> Result<tonic::Response<super::RecordsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/Records");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RecordsAll retrieves all records.
        pub async fn records_all(
            &mut self,
            request: impl tonic::IntoRequest<super::RecordsAllRequest>,
        ) -> Result<tonic::Response<super::RecordsAllResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/RecordsAll");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// Ownership returns the scope identifiers that list the given address as either a data or value owner.
        pub async fn ownership(
            &mut self,
            request: impl tonic::IntoRequest<super::OwnershipRequest>,
        ) -> Result<tonic::Response<super::OwnershipResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/Ownership");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ValueOwnership returns the scope identifiers that list the given address as the value owner.
        pub async fn value_ownership(
            &mut self,
            request: impl tonic::IntoRequest<super::ValueOwnershipRequest>,
        ) -> Result<tonic::Response<super::ValueOwnershipResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/ValueOwnership",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ScopeSpecification returns a scope specification for the given specification id.
        ///
        /// The specification_id can either be a uuid, e.g. dc83ea70-eacd-40fe-9adf-1cf6148bf8a2 or a bech32 scope
        /// specification address, e.g. scopespec1qnwg86nsatx5pl56muw0v9ytlz3qu3jx6m.
        pub async fn scope_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::ScopeSpecificationRequest>,
        ) -> Result<tonic::Response<super::ScopeSpecificationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/ScopeSpecification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ScopeSpecificationsAll retrieves all scope specifications.
        pub async fn scope_specifications_all(
            &mut self,
            request: impl tonic::IntoRequest<super::ScopeSpecificationsAllRequest>,
        ) -> Result<tonic::Response<super::ScopeSpecificationsAllResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/ScopeSpecificationsAll",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ContractSpecification returns a contract specification for the given specification id.
        ///
        /// The specification_id can either be a uuid, e.g. def6bc0a-c9dd-4874-948f-5206e6060a84, a bech32 contract
        /// specification address, e.g. contractspec1q000d0q2e8w5say53afqdesxp2zqzkr4fn, or a bech32 record specification
        /// address, e.g. recspec1qh00d0q2e8w5say53afqdesxp2zw42dq2jdvmdazuwzcaddhh8gmuqhez44. If it is a record specification
        /// address, then the contract specification that contains that record specification is looked up.
        ///
        /// By default, the record specifications for this contract specification are not included.
        /// Set include_record_specs to true to include them in the result.
        pub async fn contract_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::ContractSpecificationRequest>,
        ) -> Result<tonic::Response<super::ContractSpecificationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/ContractSpecification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ContractSpecificationsAll retrieves all contract specifications.
        pub async fn contract_specifications_all(
            &mut self,
            request: impl tonic::IntoRequest<super::ContractSpecificationsAllRequest>,
        ) -> Result<tonic::Response<super::ContractSpecificationsAllResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/ContractSpecificationsAll",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RecordSpecificationsForContractSpecification returns the record specifications for the given input.
        ///
        /// The specification_id can either be a uuid, e.g. def6bc0a-c9dd-4874-948f-5206e6060a84, a bech32 contract
        /// specification address, e.g. contractspec1q000d0q2e8w5say53afqdesxp2zqzkr4fn, or a bech32 record specification
        /// address, e.g. recspec1qh00d0q2e8w5say53afqdesxp2zw42dq2jdvmdazuwzcaddhh8gmuqhez44. If it is a record specification
        /// address, then the contract specification that contains that record specification is used.
        pub async fn record_specifications_for_contract_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::RecordSpecificationsForContractSpecificationRequest>,
        ) -> Result<
            tonic::Response<super::RecordSpecificationsForContractSpecificationResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/RecordSpecificationsForContractSpecification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RecordSpecification returns a record specification for the given input.
        pub async fn record_specification(
            &mut self,
            request: impl tonic::IntoRequest<super::RecordSpecificationRequest>,
        ) -> Result<tonic::Response<super::RecordSpecificationResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/RecordSpecification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RecordSpecificationsAll retrieves all record specifications.
        pub async fn record_specifications_all(
            &mut self,
            request: impl tonic::IntoRequest<super::RecordSpecificationsAllRequest>,
        ) -> Result<tonic::Response<super::RecordSpecificationsAllResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/RecordSpecificationsAll",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// OSLocatorParams returns all parameters for the object store locator sub module.
        pub async fn os_locator_params(
            &mut self,
            request: impl tonic::IntoRequest<super::OsLocatorParamsRequest>,
        ) -> Result<tonic::Response<super::OsLocatorParamsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/OSLocatorParams",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// OSLocator returns an ObjectStoreLocator by its owner's address.
        pub async fn os_locator(
            &mut self,
            request: impl tonic::IntoRequest<super::OsLocatorRequest>,
        ) -> Result<tonic::Response<super::OsLocatorResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/OSLocator");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// OSLocatorsByURI returns all ObjectStoreLocator entries for a locator uri.
        pub async fn os_locators_by_uri(
            &mut self,
            request: impl tonic::IntoRequest<super::OsLocatorsByUriRequest>,
        ) -> Result<tonic::Response<super::OsLocatorsByUriResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/OSLocatorsByURI",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// OSLocatorsByScope returns all ObjectStoreLocator entries for a for all signer's present in the specified scope.
        pub async fn os_locators_by_scope(
            &mut self,
            request: impl tonic::IntoRequest<super::OsLocatorsByScopeRequest>,
        ) -> Result<tonic::Response<super::OsLocatorsByScopeResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.metadata.v1.Query/OSLocatorsByScope",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// OSAllLocators returns all ObjectStoreLocator entries.
        pub async fn os_all_locators(
            &mut self,
            request: impl tonic::IntoRequest<super::OsAllLocatorsRequest>,
        ) -> Result<tonic::Response<super::OsAllLocatorsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.metadata.v1.Query/OSAllLocators");
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
        /// Params queries the parameters of x/metadata module.
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        /// Scope searches for a scope.
        ///
        /// The scope id, if provided, must either be scope uuid, e.g. 91978ba2-5f35-459a-86a7-feca1b0512e0 or a scope address,
        /// e.g. scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel. The session addr, if provided, must be a bech32 session address,
        /// e.g. session1qxge0zaztu65tx5x5llv5xc9zts9sqlch3sxwn44j50jzgt8rshvqyfrjcr. The record_addr, if provided, must be a
        /// bech32 record address, e.g. record1q2ge0zaztu65tx5x5llv5xc9ztsw42dq2jdvmdazuwzcaddhh8gmu3mcze3.
        ///
        /// * If only a scope_id is provided, that scope is returned.
        /// * If only a session_addr is provided, the scope containing that session is returned.
        /// * If only a record_addr is provided, the scope containing that record is returned.
        /// * If more than one of scope_id, session_addr, and record_addr are provided, and they don't refer to the same scope,
        /// a bad request is returned.
        ///
        /// Providing a session addr or record addr does not limit the sessions and records returned (if requested).
        /// Those parameters are only used to find the scope.
        ///
        /// By default, sessions and records are not included.
        /// Set include_sessions and/or include_records to true to include sessions and/or records.
        async fn scope(
            &self,
            request: tonic::Request<super::ScopeRequest>,
        ) -> Result<tonic::Response<super::ScopeResponse>, tonic::Status>;
        /// ScopesAll retrieves all scopes.
        async fn scopes_all(
            &self,
            request: tonic::Request<super::ScopesAllRequest>,
        ) -> Result<tonic::Response<super::ScopesAllResponse>, tonic::Status>;
        /// Sessions searches for sessions.
        ///
        /// The scope_id can either be scope uuid, e.g. 91978ba2-5f35-459a-86a7-feca1b0512e0 or a scope address, e.g.
        /// scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel. Similarly, the session_id can either be a uuid or session address, e.g.
        /// session1qxge0zaztu65tx5x5llv5xc9zts9sqlch3sxwn44j50jzgt8rshvqyfrjcr. The record_addr, if provided, must be a
        /// bech32 record address, e.g. record1q2ge0zaztu65tx5x5llv5xc9ztsw42dq2jdvmdazuwzcaddhh8gmu3mcze3.
        ///
        /// * If only a scope_id is provided, all sessions in that scope are returned.
        /// * If only a session_id is provided, it must be an address, and that single session is returned.
        /// * If the session_id is a uuid, then either a scope_id or record_addr must also be provided, and that single session
        /// is returned.
        /// * If only a record_addr is provided, the session containing that record will be returned.
        /// * If a record_name is provided then either a scope_id, session_id as an address, or record_addr must also be
        /// provided, and the session containing that record will be returned.
        ///
        /// A bad request is returned if:
        /// * The session_id is a uuid and is provided without a scope_id or record_addr.
        /// * A record_name is provided without any way to identify the scope (e.g. a scope_id, a session_id as an address, or
        /// a record_addr).
        /// * Two or more of scope_id, session_id as an address, and record_addr are provided and don't all refer to the same
        /// scope.
        /// * A record_addr (or scope_id and record_name) is provided with a session_id and that session does not contain such
        /// a record.
        /// * A record_addr and record_name are both provided, but reference different records.
        ///
        /// By default, the scope and records are not included.
        /// Set include_scope and/or include_records to true to include the scope and/or records.
        async fn sessions(
            &self,
            request: tonic::Request<super::SessionsRequest>,
        ) -> Result<tonic::Response<super::SessionsResponse>, tonic::Status>;
        /// SessionsAll retrieves all sessions.
        async fn sessions_all(
            &self,
            request: tonic::Request<super::SessionsAllRequest>,
        ) -> Result<tonic::Response<super::SessionsAllResponse>, tonic::Status>;
        /// Records searches for records.
        ///
        /// The record_addr, if provided, must be a bech32 record address, e.g.
        /// record1q2ge0zaztu65tx5x5llv5xc9ztsw42dq2jdvmdazuwzcaddhh8gmu3mcze3. The scope-id can either be scope uuid, e.g.
        /// 91978ba2-5f35-459a-86a7-feca1b0512e0 or a scope address, e.g. scope1qzge0zaztu65tx5x5llv5xc9ztsqxlkwel. Similarly,
        /// the session_id can either be a uuid or session address, e.g.
        /// session1qxge0zaztu65tx5x5llv5xc9zts9sqlch3sxwn44j50jzgt8rshvqyfrjcr. The name is the name of the record you're
        /// interested in.
        ///
        /// * If only a record_addr is provided, that single record will be returned.
        /// * If only a scope_id is provided, all records in that scope will be returned.
        /// * If only a session_id (or scope_id/session_id), all records in that session will be returned.
        /// * If a name is provided with a scope_id and/or session_id, that single record will be returned.
        ///
        /// A bad request is returned if:
        /// * The session_id is a uuid and no scope_id is provided.
        /// * There are two or more of record_addr, session_id, and scope_id, and they don't all refer to the same scope.
        /// * A name is provided, but not a scope_id and/or a session_id.
        /// * A name and record_addr are provided and the name doesn't match the record_addr.
        ///
        /// By default, the scope and sessions are not included.
        /// Set include_scope and/or include_sessions to true to include the scope and/or sessions.
        async fn records(
            &self,
            request: tonic::Request<super::RecordsRequest>,
        ) -> Result<tonic::Response<super::RecordsResponse>, tonic::Status>;
        /// RecordsAll retrieves all records.
        async fn records_all(
            &self,
            request: tonic::Request<super::RecordsAllRequest>,
        ) -> Result<tonic::Response<super::RecordsAllResponse>, tonic::Status>;
        /// Ownership returns the scope identifiers that list the given address as either a data or value owner.
        async fn ownership(
            &self,
            request: tonic::Request<super::OwnershipRequest>,
        ) -> Result<tonic::Response<super::OwnershipResponse>, tonic::Status>;
        /// ValueOwnership returns the scope identifiers that list the given address as the value owner.
        async fn value_ownership(
            &self,
            request: tonic::Request<super::ValueOwnershipRequest>,
        ) -> Result<tonic::Response<super::ValueOwnershipResponse>, tonic::Status>;
        /// ScopeSpecification returns a scope specification for the given specification id.
        ///
        /// The specification_id can either be a uuid, e.g. dc83ea70-eacd-40fe-9adf-1cf6148bf8a2 or a bech32 scope
        /// specification address, e.g. scopespec1qnwg86nsatx5pl56muw0v9ytlz3qu3jx6m.
        async fn scope_specification(
            &self,
            request: tonic::Request<super::ScopeSpecificationRequest>,
        ) -> Result<tonic::Response<super::ScopeSpecificationResponse>, tonic::Status>;
        /// ScopeSpecificationsAll retrieves all scope specifications.
        async fn scope_specifications_all(
            &self,
            request: tonic::Request<super::ScopeSpecificationsAllRequest>,
        ) -> Result<tonic::Response<super::ScopeSpecificationsAllResponse>, tonic::Status>;
        /// ContractSpecification returns a contract specification for the given specification id.
        ///
        /// The specification_id can either be a uuid, e.g. def6bc0a-c9dd-4874-948f-5206e6060a84, a bech32 contract
        /// specification address, e.g. contractspec1q000d0q2e8w5say53afqdesxp2zqzkr4fn, or a bech32 record specification
        /// address, e.g. recspec1qh00d0q2e8w5say53afqdesxp2zw42dq2jdvmdazuwzcaddhh8gmuqhez44. If it is a record specification
        /// address, then the contract specification that contains that record specification is looked up.
        ///
        /// By default, the record specifications for this contract specification are not included.
        /// Set include_record_specs to true to include them in the result.
        async fn contract_specification(
            &self,
            request: tonic::Request<super::ContractSpecificationRequest>,
        ) -> Result<tonic::Response<super::ContractSpecificationResponse>, tonic::Status>;
        /// ContractSpecificationsAll retrieves all contract specifications.
        async fn contract_specifications_all(
            &self,
            request: tonic::Request<super::ContractSpecificationsAllRequest>,
        ) -> Result<tonic::Response<super::ContractSpecificationsAllResponse>, tonic::Status>;
        /// RecordSpecificationsForContractSpecification returns the record specifications for the given input.
        ///
        /// The specification_id can either be a uuid, e.g. def6bc0a-c9dd-4874-948f-5206e6060a84, a bech32 contract
        /// specification address, e.g. contractspec1q000d0q2e8w5say53afqdesxp2zqzkr4fn, or a bech32 record specification
        /// address, e.g. recspec1qh00d0q2e8w5say53afqdesxp2zw42dq2jdvmdazuwzcaddhh8gmuqhez44. If it is a record specification
        /// address, then the contract specification that contains that record specification is used.
        async fn record_specifications_for_contract_specification(
            &self,
            request: tonic::Request<super::RecordSpecificationsForContractSpecificationRequest>,
        ) -> Result<
            tonic::Response<super::RecordSpecificationsForContractSpecificationResponse>,
            tonic::Status,
        >;
        /// RecordSpecification returns a record specification for the given input.
        async fn record_specification(
            &self,
            request: tonic::Request<super::RecordSpecificationRequest>,
        ) -> Result<tonic::Response<super::RecordSpecificationResponse>, tonic::Status>;
        /// RecordSpecificationsAll retrieves all record specifications.
        async fn record_specifications_all(
            &self,
            request: tonic::Request<super::RecordSpecificationsAllRequest>,
        ) -> Result<tonic::Response<super::RecordSpecificationsAllResponse>, tonic::Status>;
        /// OSLocatorParams returns all parameters for the object store locator sub module.
        async fn os_locator_params(
            &self,
            request: tonic::Request<super::OsLocatorParamsRequest>,
        ) -> Result<tonic::Response<super::OsLocatorParamsResponse>, tonic::Status>;
        /// OSLocator returns an ObjectStoreLocator by its owner's address.
        async fn os_locator(
            &self,
            request: tonic::Request<super::OsLocatorRequest>,
        ) -> Result<tonic::Response<super::OsLocatorResponse>, tonic::Status>;
        /// OSLocatorsByURI returns all ObjectStoreLocator entries for a locator uri.
        async fn os_locators_by_uri(
            &self,
            request: tonic::Request<super::OsLocatorsByUriRequest>,
        ) -> Result<tonic::Response<super::OsLocatorsByUriResponse>, tonic::Status>;
        /// OSLocatorsByScope returns all ObjectStoreLocator entries for a for all signer's present in the specified scope.
        async fn os_locators_by_scope(
            &self,
            request: tonic::Request<super::OsLocatorsByScopeRequest>,
        ) -> Result<tonic::Response<super::OsLocatorsByScopeResponse>, tonic::Status>;
        /// OSAllLocators returns all ObjectStoreLocator entries.
        async fn os_all_locators(
            &self,
            request: tonic::Request<super::OsAllLocatorsRequest>,
        ) -> Result<tonic::Response<super::OsAllLocatorsResponse>, tonic::Status>;
    }
    /// Query defines the Metadata Query service.
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
                "/provenance.metadata.v1.Query/Params" => {
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
                "/provenance.metadata.v1.Query/Scope" => {
                    #[allow(non_camel_case_types)]
                    struct ScopeSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ScopeRequest> for ScopeSvc<T> {
                        type Response = super::ScopeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScopeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).scope(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScopeSvc(inner);
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
                "/provenance.metadata.v1.Query/ScopesAll" => {
                    #[allow(non_camel_case_types)]
                    struct ScopesAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ScopesAllRequest> for ScopesAllSvc<T> {
                        type Response = super::ScopesAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScopesAllRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).scopes_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScopesAllSvc(inner);
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
                "/provenance.metadata.v1.Query/Sessions" => {
                    #[allow(non_camel_case_types)]
                    struct SessionsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::SessionsRequest> for SessionsSvc<T> {
                        type Response = super::SessionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SessionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).sessions(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SessionsSvc(inner);
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
                "/provenance.metadata.v1.Query/SessionsAll" => {
                    #[allow(non_camel_case_types)]
                    struct SessionsAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::SessionsAllRequest> for SessionsAllSvc<T> {
                        type Response = super::SessionsAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SessionsAllRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).sessions_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SessionsAllSvc(inner);
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
                "/provenance.metadata.v1.Query/Records" => {
                    #[allow(non_camel_case_types)]
                    struct RecordsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::RecordsRequest> for RecordsSvc<T> {
                        type Response = super::RecordsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecordsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).records(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecordsSvc(inner);
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
                "/provenance.metadata.v1.Query/RecordsAll" => {
                    #[allow(non_camel_case_types)]
                    struct RecordsAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::RecordsAllRequest> for RecordsAllSvc<T> {
                        type Response = super::RecordsAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecordsAllRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).records_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecordsAllSvc(inner);
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
                "/provenance.metadata.v1.Query/Ownership" => {
                    #[allow(non_camel_case_types)]
                    struct OwnershipSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::OwnershipRequest> for OwnershipSvc<T> {
                        type Response = super::OwnershipResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OwnershipRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).ownership(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OwnershipSvc(inner);
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
                "/provenance.metadata.v1.Query/ValueOwnership" => {
                    #[allow(non_camel_case_types)]
                    struct ValueOwnershipSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ValueOwnershipRequest> for ValueOwnershipSvc<T> {
                        type Response = super::ValueOwnershipResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValueOwnershipRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).value_ownership(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ValueOwnershipSvc(inner);
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
                "/provenance.metadata.v1.Query/ScopeSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct ScopeSpecificationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ScopeSpecificationRequest>
                        for ScopeSpecificationSvc<T>
                    {
                        type Response = super::ScopeSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScopeSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).scope_specification(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScopeSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Query/ScopeSpecificationsAll" => {
                    #[allow(non_camel_case_types)]
                    struct ScopeSpecificationsAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ScopeSpecificationsAllRequest>
                        for ScopeSpecificationsAllSvc<T>
                    {
                        type Response = super::ScopeSpecificationsAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScopeSpecificationsAllRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).scope_specifications_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScopeSpecificationsAllSvc(inner);
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
                "/provenance.metadata.v1.Query/ContractSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct ContractSpecificationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::ContractSpecificationRequest>
                        for ContractSpecificationSvc<T>
                    {
                        type Response = super::ContractSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContractSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).contract_specification(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ContractSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Query/ContractSpecificationsAll" => {
                    #[allow(non_camel_case_types)]
                    struct ContractSpecificationsAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::ContractSpecificationsAllRequest>
                        for ContractSpecificationsAllSvc<T>
                    {
                        type Response = super::ContractSpecificationsAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ContractSpecificationsAllRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).contract_specifications_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ContractSpecificationsAllSvc(inner);
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
                "/provenance.metadata.v1.Query/RecordSpecificationsForContractSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct RecordSpecificationsForContractSpecificationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::RecordSpecificationsForContractSpecificationRequest,
                        > for RecordSpecificationsForContractSpecificationSvc<T>
                    {
                        type Response = super::RecordSpecificationsForContractSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::RecordSpecificationsForContractSpecificationRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .record_specifications_for_contract_specification(request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecordSpecificationsForContractSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Query/RecordSpecification" => {
                    #[allow(non_camel_case_types)]
                    struct RecordSpecificationSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::RecordSpecificationRequest>
                        for RecordSpecificationSvc<T>
                    {
                        type Response = super::RecordSpecificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecordSpecificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).record_specification(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecordSpecificationSvc(inner);
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
                "/provenance.metadata.v1.Query/RecordSpecificationsAll" => {
                    #[allow(non_camel_case_types)]
                    struct RecordSpecificationsAllSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::RecordSpecificationsAllRequest>
                        for RecordSpecificationsAllSvc<T>
                    {
                        type Response = super::RecordSpecificationsAllResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecordSpecificationsAllRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).record_specifications_all(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecordSpecificationsAllSvc(inner);
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
                "/provenance.metadata.v1.Query/OSLocatorParams" => {
                    #[allow(non_camel_case_types)]
                    struct OSLocatorParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::OsLocatorParamsRequest>
                        for OSLocatorParamsSvc<T>
                    {
                        type Response = super::OsLocatorParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OsLocatorParamsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).os_locator_params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OSLocatorParamsSvc(inner);
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
                "/provenance.metadata.v1.Query/OSLocator" => {
                    #[allow(non_camel_case_types)]
                    struct OSLocatorSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::OsLocatorRequest> for OSLocatorSvc<T> {
                        type Response = super::OsLocatorResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OsLocatorRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).os_locator(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OSLocatorSvc(inner);
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
                "/provenance.metadata.v1.Query/OSLocatorsByURI" => {
                    #[allow(non_camel_case_types)]
                    struct OSLocatorsByURISvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::OsLocatorsByUriRequest>
                        for OSLocatorsByURISvc<T>
                    {
                        type Response = super::OsLocatorsByUriResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OsLocatorsByUriRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).os_locators_by_uri(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OSLocatorsByURISvc(inner);
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
                "/provenance.metadata.v1.Query/OSLocatorsByScope" => {
                    #[allow(non_camel_case_types)]
                    struct OSLocatorsByScopeSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::OsLocatorsByScopeRequest>
                        for OSLocatorsByScopeSvc<T>
                    {
                        type Response = super::OsLocatorsByScopeResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OsLocatorsByScopeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).os_locators_by_scope(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OSLocatorsByScopeSvc(inner);
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
                "/provenance.metadata.v1.Query/OSAllLocators" => {
                    #[allow(non_camel_case_types)]
                    struct OSAllLocatorsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::OsAllLocatorsRequest> for OSAllLocatorsSvc<T> {
                        type Response = super::OsAllLocatorsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OsAllLocatorsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).os_all_locators(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OSAllLocatorsSvc(inner);
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
        const NAME: &'static str = "provenance.metadata.v1.Query";
    }
}
/// GenesisState defines the account module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// A collection of metadata scopes and specs to create on start
    #[prost(message, repeated, tag = "2")]
    pub scopes: ::prost::alloc::vec::Vec<Scope>,
    #[prost(message, repeated, tag = "3")]
    pub sessions: ::prost::alloc::vec::Vec<Session>,
    #[prost(message, repeated, tag = "4")]
    pub records: ::prost::alloc::vec::Vec<Record>,
    #[prost(message, repeated, tag = "5")]
    pub scope_specifications: ::prost::alloc::vec::Vec<ScopeSpecification>,
    #[prost(message, repeated, tag = "6")]
    pub contract_specifications: ::prost::alloc::vec::Vec<ContractSpecification>,
    #[prost(message, repeated, tag = "7")]
    pub record_specifications: ::prost::alloc::vec::Vec<RecordSpecification>,
    #[prost(message, optional, tag = "8")]
    pub o_s_locator_params: ::core::option::Option<OsLocatorParams>,
    #[prost(message, repeated, tag = "9")]
    pub object_store_locators: ::prost::alloc::vec::Vec<ObjectStoreLocator>,
}
