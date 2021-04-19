mod autogenerated_code {
    // Expose proto data types from included third-party/external proto files.
    pub use mc_api::external;
    pub use mc_attest_api::attest;
    pub use mc_fog_api::{report, report_grpc};
    pub use protobuf::well_known_types::Empty;

    // Needed due to how to the auto-generated code references the Empty message.
    pub mod empty {
        pub use protobuf::well_known_types::Empty;
    }

    // Include the auto-generated code.
    include!(concat!(env!("OUT_DIR"), "/protos-auto-gen/mod.rs"));
}

pub use autogenerated_code::*;

use fog_uri::{IngestPeerUri, UriParseError};
use mc_util_grpc::BasicCredentials;
use std::{collections::BTreeSet, str::FromStr};

// For tests, we need to implement Eq on view::QueryRequest
// They implement PartialEq but not Eq for some reason
impl Eq for autogenerated_code::view::QueryRequest {}
impl Eq for autogenerated_code::view::QueryRequestAAD {}
impl Eq for autogenerated_code::kex_rng::KexRngPubkey {}
impl Eq for autogenerated_code::kex_rng::StoredRng {}

// Extra functions for IngestSummary to avoid repetition
impl ingest_common::IngestSummary {
    pub fn get_sorted_peers(&self) -> Result<BTreeSet<IngestPeerUri>, UriParseError> {
        self.peers
            .iter()
            .map(|x| IngestPeerUri::from_str(x))
            .collect()
    }
}

// Implement the EnclaveGrpcChannel trait on attested service types.
// If we don't do this in this crate, then newtype wrappers must be used,
// because of orphan rules
use fog_enclave_connection::EnclaveGrpcChannel;

impl EnclaveGrpcChannel for view_grpc::FogViewApiClient {
    fn auth(
        &mut self,
        msg: &attest::AuthMessage,
        creds: &BasicCredentials,
    ) -> Result<attest::AuthMessage, grpcio::Error> {
        <Self>::auth_opt(self, msg, creds.call_option()?)
    }
    fn enclave_request(
        &mut self,
        msg: &attest::Message,
        creds: &BasicCredentials,
    ) -> Result<attest::Message, grpcio::Error> {
        <Self>::query_opt(self, msg, creds.call_option()?)
    }
}

impl EnclaveGrpcChannel for ledger_grpc::FogKeyImageApiClient {
    fn auth(
        &mut self,
        msg: &attest::AuthMessage,
        creds: &BasicCredentials,
    ) -> Result<attest::AuthMessage, grpcio::Error> {
        <Self>::auth_opt(self, msg, creds.call_option()?)
    }
    fn enclave_request(
        &mut self,
        msg: &attest::Message,
        creds: &BasicCredentials,
    ) -> Result<attest::Message, grpcio::Error> {
        self.check_key_images_opt(msg, creds.call_option()?)
    }
}

impl EnclaveGrpcChannel for ledger_grpc::FogMerkleProofApiClient {
    fn auth(
        &mut self,
        msg: &attest::AuthMessage,
        creds: &BasicCredentials,
    ) -> Result<attest::AuthMessage, grpcio::Error> {
        <Self>::auth_opt(self, msg, creds.call_option()?)
    }
    fn enclave_request(
        &mut self,
        msg: &attest::Message,
        creds: &BasicCredentials,
    ) -> Result<attest::Message, grpcio::Error> {
        self.get_outputs_opt(msg, creds.call_option()?)
    }
}
