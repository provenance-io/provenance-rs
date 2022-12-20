/// RewardProgram
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardProgram {
    /// An integer to uniquely identify the reward program.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// Name to help identify the Reward Program.(MaxTitleLength=140)
    #[prost(string, tag = "2")]
    pub title: ::prost::alloc::string::String,
    /// Short summary describing the Reward Program.(MaxDescriptionLength=10000)
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// address that provides funds for the total reward pool.
    #[prost(string, tag = "4")]
    pub distribute_from_address: ::prost::alloc::string::String,
    /// The total amount of funding given to the RewardProgram.
    #[prost(message, optional, tag = "5")]
    pub total_reward_pool: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// The remaining funds available to distribute after n claim periods have passed.
    #[prost(message, optional, tag = "6")]
    pub remaining_pool_balance:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// The total amount of all funds claimed by participants for all past claim periods.
    #[prost(message, optional, tag = "7")]
    pub claimed_amount: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Maximum reward per claim period per address.
    #[prost(message, optional, tag = "8")]
    pub max_reward_by_address:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Minimum amount of coins for a program to rollover.
    #[prost(message, optional, tag = "9")]
    pub minimum_rollover_amount:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Number of seconds that a claim period lasts.
    #[prost(uint64, tag = "10")]
    pub claim_period_seconds: u64,
    /// Time that a RewardProgram should start and switch to STARTED state.
    #[prost(message, optional, tag = "11")]
    pub program_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time that a RewardProgram is expected to end, based on data when it was setup.
    #[prost(message, optional, tag = "12")]
    pub expected_program_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time that a RewardProgram MUST end.
    #[prost(message, optional, tag = "13")]
    pub program_end_time_max: ::core::option::Option<::prost_types::Timestamp>,
    /// Used internally to calculate and track the current claim period's ending time.
    #[prost(message, optional, tag = "14")]
    pub claim_period_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Time the RewardProgram switched to FINISHED state. Initially set as empty.
    #[prost(message, optional, tag = "15")]
    pub actual_program_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Number of claim periods this program will run for.
    #[prost(uint64, tag = "16")]
    pub claim_periods: u64,
    /// Current claim period of the RewardProgram. Uses 1-based indexing.
    #[prost(uint64, tag = "17")]
    pub current_claim_period: u64,
    /// maximum number of claim periods a reward program can rollover.
    #[prost(uint64, tag = "18")]
    pub max_rollover_claim_periods: u64,
    /// Current state of the RewardProgram.
    #[prost(enumeration = "reward_program::State", tag = "19")]
    pub state: i32,
    /// Grace period after a RewardProgram FINISHED. It is the number of seconds until a RewardProgram enters the EXPIRED
    /// state.
    #[prost(uint64, tag = "20")]
    pub expiration_offset: u64,
    /// Actions that count towards the reward.
    #[prost(message, repeated, tag = "21")]
    pub qualifying_actions: ::prost::alloc::vec::Vec<QualifyingAction>,
}
/// Nested message and enum types in `RewardProgram`.
pub mod reward_program {
    /// State is the state of the reward program
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// undefined program state
        Unspecified = 0,
        /// pending state of reward program
        Pending = 1,
        /// started state of reward program
        Started = 2,
        /// finished state of reward program
        Finished = 3,
        /// expired state of reward program
        Expired = 4,
    }
    impl State {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                State::Unspecified => "STATE_UNSPECIFIED",
                State::Pending => "STATE_PENDING",
                State::Started => "STATE_STARTED",
                State::Finished => "STATE_FINISHED",
                State::Expired => "STATE_EXPIRED",
            }
        }
    }
}
/// ClaimPeriodRewardDistribution, this is updated at the end of every claim period.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimPeriodRewardDistribution {
    /// The claim period id.
    #[prost(uint64, tag = "1")]
    pub claim_period_id: u64,
    /// The id of the reward program that this reward belongs to.
    #[prost(uint64, tag = "2")]
    pub reward_program_id: u64,
    /// The sum of all the granted rewards for this claim period.
    #[prost(message, optional, tag = "3")]
    pub total_rewards_pool_for_claim_period:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// The final allocated rewards for this claim period.
    #[prost(message, optional, tag = "4")]
    pub rewards_pool: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// The total number of granted shares for this claim period.
    #[prost(int64, tag = "5")]
    pub total_shares: i64,
    /// A flag representing if the claim period for this reward has ended.
    #[prost(bool, tag = "6")]
    pub claim_period_ended: bool,
}
/// RewardAccountState contains state at the claim period level for a specific address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardAccountState {
    /// The id of the reward program that this share belongs to.
    #[prost(uint64, tag = "1")]
    pub reward_program_id: u64,
    /// The id of the claim period that the share belongs to.
    #[prost(uint64, tag = "2")]
    pub claim_period_id: u64,
    /// Owner of the reward account state.
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    /// The number of actions performed by this account, mapped by action type.
    #[prost(message, repeated, tag = "4")]
    pub action_counter: ::prost::alloc::vec::Vec<ActionCounter>,
    /// The amount of granted shares for the address in the reward program's claim period.
    #[prost(uint64, tag = "5")]
    pub shares_earned: u64,
    /// The status of the claim.
    #[prost(enumeration = "reward_account_state::ClaimStatus", tag = "6")]
    pub claim_status: i32,
}
/// Nested message and enum types in `RewardAccountState`.
pub mod reward_account_state {
    /// ClaimStatus is the state a claim is in
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ClaimStatus {
        /// undefined state
        Unspecified = 0,
        /// unclaimable status
        Unclaimable = 1,
        /// unclaimable claimable
        Claimable = 2,
        /// unclaimable claimed
        Claimed = 3,
        /// unclaimable expired
        Expired = 4,
    }
    impl ClaimStatus {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ClaimStatus::Unspecified => "CLAIM_STATUS_UNSPECIFIED",
                ClaimStatus::Unclaimable => "CLAIM_STATUS_UNCLAIMABLE",
                ClaimStatus::Claimable => "CLAIM_STATUS_CLAIMABLE",
                ClaimStatus::Claimed => "CLAIM_STATUS_CLAIMED",
                ClaimStatus::Expired => "CLAIM_STATUS_EXPIRED",
            }
        }
    }
}
/// QualifyingAction can be one of many action types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QualifyingAction {
    /// type of action to process
    #[prost(oneof = "qualifying_action::Type", tags = "1, 2, 3")]
    pub r#type: ::core::option::Option<qualifying_action::Type>,
}
/// Nested message and enum types in `QualifyingAction`.
pub mod qualifying_action {
    /// type of action to process
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(message, tag = "1")]
        Delegate(super::ActionDelegate),
        #[prost(message, tag = "2")]
        Transfer(super::ActionTransfer),
        #[prost(message, tag = "3")]
        Vote(super::ActionVote),
    }
}
/// QualifyingActions contains a list of QualifyingActions.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QualifyingActions {
    /// The actions that count towards the reward.
    #[prost(message, repeated, tag = "1")]
    pub qualifying_actions: ::prost::alloc::vec::Vec<QualifyingAction>,
}
/// ActionDelegate represents the delegate action and its required eligibility criteria.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionDelegate {
    /// Minimum number of successful delegates.
    #[prost(uint64, tag = "1")]
    pub minimum_actions: u64,
    /// Maximum number of successful delegates.
    #[prost(uint64, tag = "2")]
    pub maximum_actions: u64,
    /// Minimum amount that the user must have currently delegated on the validator.
    #[prost(message, optional, tag = "3")]
    pub minimum_delegation_amount:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Maximum amount that the user must have currently delegated on the validator.
    #[prost(message, optional, tag = "4")]
    pub maximum_delegation_amount:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Minimum percentile that can be below the validator's power ranking.
    #[prost(string, tag = "5")]
    pub minimum_active_stake_percentile: ::prost::alloc::string::String,
    /// Maximum percentile that can be below the validator's power ranking.
    #[prost(string, tag = "6")]
    pub maximum_active_stake_percentile: ::prost::alloc::string::String,
}
/// ActionTransfer represents the transfer action and its required eligibility criteria.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionTransfer {
    /// Minimum number of successful transfers.
    #[prost(uint64, tag = "1")]
    pub minimum_actions: u64,
    /// Maximum number of successful transfers.
    #[prost(uint64, tag = "2")]
    pub maximum_actions: u64,
    /// Minimum delegation amount the account must have across all validators, for the transfer action to be counted.
    #[prost(message, optional, tag = "3")]
    pub minimum_delegation_amount:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// ActionVote represents the voting action and its required eligibility criteria.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionVote {
    /// Minimum number of successful votes.
    #[prost(uint64, tag = "1")]
    pub minimum_actions: u64,
    /// Maximum number of successful votes.
    #[prost(uint64, tag = "2")]
    pub maximum_actions: u64,
    /// Minimum delegation amount the account must have across all validators, for the vote action to be counted.
    #[prost(message, optional, tag = "3")]
    pub minimum_delegation_amount:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// Positive multiplier that is applied to the shares awarded by the vote action when conditions
    /// are met(for now the only condition is the current vote is a validator vote). A value of zero will behave the same as one
    #[prost(uint64, tag = "4")]
    pub validator_multiplier: u64,
}
/// ActionCounter is a key-value pair that maps action type to the number of times it was performed.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ActionCounter {
    /// The type of action performed.
    #[prost(string, tag = "1")]
    pub action_type: ::prost::alloc::string::String,
    /// The number of times this action has been performed
    #[prost(uint64, tag = "2")]
    pub number_of_actions: u64,
}
/// MsgCreateRewardProgramRequest is the request type for creating a reward program RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateRewardProgramRequest {
    /// title for the reward program.
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// description for the reward program.
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// provider address for the reward program funds and signer of message.
    #[prost(string, tag = "3")]
    pub distribute_from_address: ::prost::alloc::string::String,
    /// total reward pool for the reward program.
    #[prost(message, optional, tag = "4")]
    pub total_reward_pool: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// maximum amount of funds an address can be rewarded per claim period.
    #[prost(message, optional, tag = "5")]
    pub max_reward_per_claim_address:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// start time of the reward program.
    #[prost(message, optional, tag = "6")]
    pub program_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// number of claim periods the reward program runs for.
    #[prost(uint64, tag = "7")]
    pub claim_periods: u64,
    /// number of days a claim period will exist.
    #[prost(uint64, tag = "8")]
    pub claim_period_days: u64,
    /// maximum number of claim periods a reward program can rollover.
    #[prost(uint64, tag = "9")]
    pub max_rollover_claim_periods: u64,
    /// number of days before a reward program will expire after it has ended.
    #[prost(uint64, tag = "10")]
    pub expire_days: u64,
    /// actions that count towards the reward.
    #[prost(message, repeated, tag = "11")]
    pub qualifying_actions: ::prost::alloc::vec::Vec<QualifyingAction>,
}
/// MsgCreateRewardProgramResponse is the response type for creating a reward program RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateRewardProgramResponse {
    /// reward program id that is generated on creation.
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
/// MsgEndRewardProgramRequest is the request type for ending a reward program RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEndRewardProgramRequest {
    /// reward program id to end.
    #[prost(uint64, tag = "1")]
    pub reward_program_id: u64,
    /// owner of the reward program that funds were distributed from.
    #[prost(string, tag = "2")]
    pub program_owner_address: ::prost::alloc::string::String,
}
/// MsgEndRewardProgramResponse is the response type for ending a reward program RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgEndRewardProgramResponse {}
/// MsgClaimRewardsRequest is the request type for claiming reward from reward program RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimRewardsRequest {
    /// reward program id to claim rewards.
    #[prost(uint64, tag = "1")]
    pub reward_program_id: u64,
    /// reward address and signer of msg to send claimed rewards to.
    #[prost(string, tag = "2")]
    pub reward_address: ::prost::alloc::string::String,
}
/// MsgClaimRewardsResponse is the response type for claiming reward from reward program RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimRewardsResponse {
    /// details about acquired rewards from reward program.
    #[prost(message, optional, tag = "1")]
    pub claim_details: ::core::option::Option<RewardProgramClaimDetail>,
}
/// MsgClaimRewardsResponse is the request type for claiming rewards from all reward programs RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimAllRewardsRequest {
    /// reward address and signer of msg to send claimed rewards to.
    #[prost(string, tag = "1")]
    pub reward_address: ::prost::alloc::string::String,
}
/// MsgClaimRewardsResponse is the response type for claiming rewards from all reward programs RPC
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimAllRewardsResponse {
    /// total rewards claimed for all eligible claim periods in all programs.
    #[prost(message, repeated, tag = "1")]
    pub total_reward_claim: ::prost::alloc::vec::Vec<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// details about acquired rewards from a reward program.
    #[prost(message, repeated, tag = "2")]
    pub claim_details: ::prost::alloc::vec::Vec<RewardProgramClaimDetail>,
}
/// ClaimedRewardPeriodDetail is information regarding an addresses' shares and reward for a claim period.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimedRewardPeriodDetail {
    /// claim period id
    #[prost(uint64, tag = "1")]
    pub claim_period_id: u64,
    /// total shares accumulated for claim period
    #[prost(uint64, tag = "2")]
    pub total_shares: u64,
    /// total rewards for claim period
    #[prost(message, optional, tag = "3")]
    pub claim_period_reward: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
}
/// RewardProgramClaimDetail is the response object regarding an address's shares and reward for a reward program.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardProgramClaimDetail {
    /// reward program id.
    #[prost(uint64, tag = "1")]
    pub reward_program_id: u64,
    /// total rewards claimed for all eligible claim periods in program.
    #[prost(message, optional, tag = "2")]
    pub total_reward_claim: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// claim period details.
    #[prost(message, repeated, tag = "3")]
    pub claimed_reward_period_details: ::prost::alloc::vec::Vec<ClaimedRewardPeriodDetail>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg
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
        /// CreateRewardProgram is the RPC endpoint for creating a rewards program
        pub async fn create_reward_program(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateRewardProgramRequest>,
        ) -> Result<tonic::Response<super::MsgCreateRewardProgramResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.reward.v1.Msg/CreateRewardProgram",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// EndRewardProgram is the RPC endpoint for ending a rewards program
        pub async fn end_reward_program(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgEndRewardProgramRequest>,
        ) -> Result<tonic::Response<super::MsgEndRewardProgramResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.reward.v1.Msg/EndRewardProgram");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ClaimRewards is the RPC endpoint for claiming rewards belonging to completed claim periods of a reward program
        pub async fn claim_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgClaimRewardsRequest>,
        ) -> Result<tonic::Response<super::MsgClaimRewardsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.reward.v1.Msg/ClaimRewards");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ClaimAllRewards is the RPC endpoint for claiming rewards for completed claim periods of every reward program for
        /// the signer of the tx.
        pub async fn claim_all_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgClaimAllRewardsRequest>,
        ) -> Result<tonic::Response<super::MsgClaimAllRewardsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.reward.v1.Msg/ClaimAllRewards");
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
        /// CreateRewardProgram is the RPC endpoint for creating a rewards program
        async fn create_reward_program(
            &self,
            request: tonic::Request<super::MsgCreateRewardProgramRequest>,
        ) -> Result<tonic::Response<super::MsgCreateRewardProgramResponse>, tonic::Status>;
        /// EndRewardProgram is the RPC endpoint for ending a rewards program
        async fn end_reward_program(
            &self,
            request: tonic::Request<super::MsgEndRewardProgramRequest>,
        ) -> Result<tonic::Response<super::MsgEndRewardProgramResponse>, tonic::Status>;
        /// ClaimRewards is the RPC endpoint for claiming rewards belonging to completed claim periods of a reward program
        async fn claim_rewards(
            &self,
            request: tonic::Request<super::MsgClaimRewardsRequest>,
        ) -> Result<tonic::Response<super::MsgClaimRewardsResponse>, tonic::Status>;
        /// ClaimAllRewards is the RPC endpoint for claiming rewards for completed claim periods of every reward program for
        /// the signer of the tx.
        async fn claim_all_rewards(
            &self,
            request: tonic::Request<super::MsgClaimAllRewardsRequest>,
        ) -> Result<tonic::Response<super::MsgClaimAllRewardsResponse>, tonic::Status>;
    }
    /// Msg
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
                "/provenance.reward.v1.Msg/CreateRewardProgram" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRewardProgramSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgCreateRewardProgramRequest>
                        for CreateRewardProgramSvc<T>
                    {
                        type Response = super::MsgCreateRewardProgramResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgCreateRewardProgramRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_reward_program(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateRewardProgramSvc(inner);
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
                "/provenance.reward.v1.Msg/EndRewardProgram" => {
                    #[allow(non_camel_case_types)]
                    struct EndRewardProgramSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgEndRewardProgramRequest>
                        for EndRewardProgramSvc<T>
                    {
                        type Response = super::MsgEndRewardProgramResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgEndRewardProgramRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).end_reward_program(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EndRewardProgramSvc(inner);
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
                "/provenance.reward.v1.Msg/ClaimRewards" => {
                    #[allow(non_camel_case_types)]
                    struct ClaimRewardsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgClaimRewardsRequest> for ClaimRewardsSvc<T> {
                        type Response = super::MsgClaimRewardsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgClaimRewardsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).claim_rewards(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClaimRewardsSvc(inner);
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
                "/provenance.reward.v1.Msg/ClaimAllRewards" => {
                    #[allow(non_camel_case_types)]
                    struct ClaimAllRewardsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgClaimAllRewardsRequest>
                        for ClaimAllRewardsSvc<T>
                    {
                        type Response = super::MsgClaimAllRewardsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgClaimAllRewardsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).claim_all_rewards(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClaimAllRewardsSvc(inner);
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
        const NAME: &'static str = "provenance.reward.v1.Msg";
    }
}
/// QueryRewardProgramByIDRequest queries for the Reward Program with an identifier of id
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardProgramByIdRequest {
    /// The id of the reward program to query.
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
/// QueryRewardProgramByIDResponse contains the requested RewardProgram
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardProgramByIdResponse {
    /// The reward program object that was queried for.
    #[prost(message, optional, tag = "1")]
    pub reward_program: ::core::option::Option<RewardProgram>,
}
/// QueryRewardProgramsRequest queries for all reward programs matching the query_type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardProgramsRequest {
    /// A filter on the types of reward programs.
    #[prost(enumeration = "query_reward_programs_request::QueryType", tag = "1")]
    pub query_type: i32,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// Nested message and enum types in `QueryRewardProgramsRequest`.
pub mod query_reward_programs_request {
    /// QueryType is the state of reward program to query
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum QueryType {
        /// unspecified type
        Unspecified = 0,
        /// all reward programs states
        All = 1,
        /// pending reward program state=
        Pending = 2,
        /// active reward program state
        Active = 3,
        /// pending and active reward program states
        Outstanding = 4,
        /// finished reward program state
        Finished = 5,
    }
    impl QueryType {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                QueryType::Unspecified => "QUERY_TYPE_UNSPECIFIED",
                QueryType::All => "QUERY_TYPE_ALL",
                QueryType::Pending => "QUERY_TYPE_PENDING",
                QueryType::Active => "QUERY_TYPE_ACTIVE",
                QueryType::Outstanding => "QUERY_TYPE_OUTSTANDING",
                QueryType::Finished => "QUERY_TYPE_FINISHED",
            }
        }
    }
}
/// QueryRewardProgramsResponse contains the list of RewardPrograms matching the query
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardProgramsResponse {
    /// List of RewardProgram objects matching the query_type.
    #[prost(message, repeated, tag = "1")]
    pub reward_programs: ::prost::alloc::vec::Vec<RewardProgram>,
    /// pagination defines an optional pagination for the response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryClaimPeriodRewardDistributionsRequest queries for all the ClaimPeriodRewardDistributions with pagination.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimPeriodRewardDistributionsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryClaimPeriodRewardDistributionsResponse returns the list of paginated ClaimPeriodRewardDistributions
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimPeriodRewardDistributionsResponse {
    /// List of all ClaimPeriodRewardDistribution objects queried for.
    #[prost(message, repeated, tag = "1")]
    pub claim_period_reward_distributions: ::prost::alloc::vec::Vec<ClaimPeriodRewardDistribution>,
    /// pagination defines an optional pagination for the response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryClaimPeriodRewardDistributionsByIDRequest queries for a single ClaimPeriodRewardDistribution
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimPeriodRewardDistributionsByIdRequest {
    /// The reward program that the claim period reward distribution belongs to.
    #[prost(uint64, tag = "1")]
    pub reward_id: u64,
    /// The claim period that the claim period reward distribution was created for.
    #[prost(uint64, tag = "2")]
    pub claim_period_id: u64,
}
/// QueryClaimPeriodRewardDistributionsByIDResponse returns the requested ClaimPeriodRewardDistribution
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClaimPeriodRewardDistributionsByIdResponse {
    /// The ClaimPeriodRewardDistribution object that was queried for.
    #[prost(message, optional, tag = "1")]
    pub claim_period_reward_distribution: ::core::option::Option<ClaimPeriodRewardDistribution>,
}
/// QueryRewardDistributionsByAddressRequest queries for reward claims by address that match the claim_status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardDistributionsByAddressRequest {
    /// The address that the claim belongs to.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// The status that the reward account must have.
    #[prost(enumeration = "reward_account_state::ClaimStatus", tag = "2")]
    pub claim_status: i32,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryRewardDistributionsByAddressResponse returns the reward claims for an address that match the claim_status.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardDistributionsByAddressResponse {
    /// The address that the reward account belongs to.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// List of RewardAccounts queried for.
    #[prost(message, repeated, tag = "2")]
    pub reward_account_state: ::prost::alloc::vec::Vec<RewardAccountResponse>,
    /// pagination defines an optional pagination for the response.
    #[prost(message, optional, tag = "99")]
    pub pagination:
        ::core::option::Option<cosmos_sdk_proto::cosmos::base::query::v1beta1::PageResponse>,
}
/// RewardAccountResponse is an address' reward claim for a reward program's claim period.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardAccountResponse {
    /// The id of the reward program that this claim belongs to.
    #[prost(uint64, tag = "1")]
    pub reward_program_id: u64,
    /// total rewards claimed for all eligible claim periods in program.
    #[prost(message, optional, tag = "2")]
    pub total_reward_claim: ::core::option::Option<cosmos_sdk_proto::cosmos::base::v1beta1::Coin>,
    /// The status of the claim.
    #[prost(enumeration = "reward_account_state::ClaimStatus", tag = "3")]
    pub claim_status: i32,
    /// The claim period that the claim belongs to.
    #[prost(uint64, tag = "4")]
    pub claim_id: u64,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC querier service for reward module.
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
        /// RewardProgramByID returns a reward program matching the ID.
        pub async fn reward_program_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRewardProgramByIdRequest>,
        ) -> Result<tonic::Response<super::QueryRewardProgramByIdResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.reward.v1.Query/RewardProgramByID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RewardPrograms returns a list of reward programs matching the query type.
        pub async fn reward_programs(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRewardProgramsRequest>,
        ) -> Result<tonic::Response<super::QueryRewardProgramsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/provenance.reward.v1.Query/RewardPrograms");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ClaimPeriodRewardDistributions returns a list of claim period reward distributions matching the claim_status.
        pub async fn claim_period_reward_distributions(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClaimPeriodRewardDistributionsRequest>,
        ) -> Result<
            tonic::Response<super::QueryClaimPeriodRewardDistributionsResponse>,
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
                "/provenance.reward.v1.Query/ClaimPeriodRewardDistributions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// ClaimPeriodRewardDistributionsByID returns a claim period reward distribution matching the ID.
        pub async fn claim_period_reward_distributions_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClaimPeriodRewardDistributionsByIdRequest>,
        ) -> Result<
            tonic::Response<super::QueryClaimPeriodRewardDistributionsByIdResponse>,
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
                "/provenance.reward.v1.Query/ClaimPeriodRewardDistributionsByID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// RewardDistributionsByAddress returns a list of reward claims belonging to the account and matching the claim
        /// status.
        pub async fn reward_distributions_by_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRewardDistributionsByAddressRequest>,
        ) -> Result<tonic::Response<super::QueryRewardDistributionsByAddressResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/provenance.reward.v1.Query/RewardDistributionsByAddress",
            );
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
        /// RewardProgramByID returns a reward program matching the ID.
        async fn reward_program_by_id(
            &self,
            request: tonic::Request<super::QueryRewardProgramByIdRequest>,
        ) -> Result<tonic::Response<super::QueryRewardProgramByIdResponse>, tonic::Status>;
        /// RewardPrograms returns a list of reward programs matching the query type.
        async fn reward_programs(
            &self,
            request: tonic::Request<super::QueryRewardProgramsRequest>,
        ) -> Result<tonic::Response<super::QueryRewardProgramsResponse>, tonic::Status>;
        /// ClaimPeriodRewardDistributions returns a list of claim period reward distributions matching the claim_status.
        async fn claim_period_reward_distributions(
            &self,
            request: tonic::Request<super::QueryClaimPeriodRewardDistributionsRequest>,
        ) -> Result<
            tonic::Response<super::QueryClaimPeriodRewardDistributionsResponse>,
            tonic::Status,
        >;
        /// ClaimPeriodRewardDistributionsByID returns a claim period reward distribution matching the ID.
        async fn claim_period_reward_distributions_by_id(
            &self,
            request: tonic::Request<super::QueryClaimPeriodRewardDistributionsByIdRequest>,
        ) -> Result<
            tonic::Response<super::QueryClaimPeriodRewardDistributionsByIdResponse>,
            tonic::Status,
        >;
        /// RewardDistributionsByAddress returns a list of reward claims belonging to the account and matching the claim
        /// status.
        async fn reward_distributions_by_address(
            &self,
            request: tonic::Request<super::QueryRewardDistributionsByAddressRequest>,
        ) -> Result<tonic::Response<super::QueryRewardDistributionsByAddressResponse>, tonic::Status>;
    }
    /// Query defines the gRPC querier service for reward module.
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
                "/provenance.reward.v1.Query/RewardProgramByID" => {
                    #[allow(non_camel_case_types)]
                    struct RewardProgramByIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryRewardProgramByIdRequest>
                        for RewardProgramByIDSvc<T>
                    {
                        type Response = super::QueryRewardProgramByIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRewardProgramByIdRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).reward_program_by_id(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RewardProgramByIDSvc(inner);
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
                "/provenance.reward.v1.Query/RewardPrograms" => {
                    #[allow(non_camel_case_types)]
                    struct RewardProgramsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryRewardProgramsRequest>
                        for RewardProgramsSvc<T>
                    {
                        type Response = super::QueryRewardProgramsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRewardProgramsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).reward_programs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RewardProgramsSvc(inner);
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
                "/provenance.reward.v1.Query/ClaimPeriodRewardDistributions" => {
                    #[allow(non_camel_case_types)]
                    struct ClaimPeriodRewardDistributionsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryClaimPeriodRewardDistributionsRequest,
                        > for ClaimPeriodRewardDistributionsSvc<T>
                    {
                        type Response = super::QueryClaimPeriodRewardDistributionsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryClaimPeriodRewardDistributionsRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).claim_period_reward_distributions(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClaimPeriodRewardDistributionsSvc(inner);
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
                "/provenance.reward.v1.Query/ClaimPeriodRewardDistributionsByID" => {
                    #[allow(non_camel_case_types)]
                    struct ClaimPeriodRewardDistributionsByIDSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<
                            super::QueryClaimPeriodRewardDistributionsByIdRequest,
                        > for ClaimPeriodRewardDistributionsByIDSvc<T>
                    {
                        type Response = super::QueryClaimPeriodRewardDistributionsByIdResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryClaimPeriodRewardDistributionsByIdRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner)
                                    .claim_period_reward_distributions_by_id(request)
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
                        let method = ClaimPeriodRewardDistributionsByIDSvc(inner);
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
                "/provenance.reward.v1.Query/RewardDistributionsByAddress" => {
                    #[allow(non_camel_case_types)]
                    struct RewardDistributionsByAddressSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryRewardDistributionsByAddressRequest>
                        for RewardDistributionsByAddressSvc<T>
                    {
                        type Response = super::QueryRewardDistributionsByAddressResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::QueryRewardDistributionsByAddressRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).reward_distributions_by_address(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RewardDistributionsByAddressSvc(inner);
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
        const NAME: &'static str = "provenance.reward.v1.Query";
    }
}
/// GenesisState defines the reward module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// Reward program id is the next auto incremented id to be assigned to the next created reward program
    #[prost(uint64, tag = "1")]
    pub reward_program_id: u64,
    /// Reward programs to initially start with.
    #[prost(message, repeated, tag = "2")]
    pub reward_programs: ::prost::alloc::vec::Vec<RewardProgram>,
    /// Claim period reward distributions to initially start with.
    #[prost(message, repeated, tag = "3")]
    pub claim_period_reward_distributions: ::prost::alloc::vec::Vec<ClaimPeriodRewardDistribution>,
    /// Reward account states to initially start with.
    #[prost(message, repeated, tag = "4")]
    pub reward_account_states: ::prost::alloc::vec::Vec<RewardAccountState>,
}
