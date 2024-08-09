pub use prost;
pub use prost_types::Any;

pub const PROVENANCE_SDK_VERSION: &str = include_str!("prost/provenance-sdk/PROVENANCE_SDK_COMMIT");

pub const PROVENANCE_MAINNET_ACCOUNT_PREFIX: &str = "pb";
pub const PROVENANCE_TESTNET_ACCOUNT_PREFIX: &str = "tp";

pub mod provenance {
    pub mod attribute {
        pub mod v1 {
            include!("prost/provenance-sdk/provenance.attribute.v1.rs");
        }
    }

    pub mod marker {
        pub mod v1 {
            include!("prost/provenance-sdk/provenance.marker.v1.rs");
        }
    }

    pub mod metadata {
        pub mod v1 {
            include!("prost/provenance-sdk/provenance.metadata.v1.rs");

            pub mod p8e {
                include!("prost/provenance-sdk/provenance.metadata.v1.p8e.rs");
            }
        }
    }

    pub mod msgfees {
        pub mod v1 {
            include!("prost/provenance-sdk/provenance.msgfees.v1.rs");
        }
    }

    pub mod name {
        pub mod v1 {
            include!("prost/provenance-sdk/provenance.name.v1.rs");
        }
    }
}
