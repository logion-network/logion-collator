// This is free and unencumbered software released into the public domain.
//
// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.
//
// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// For more information, please refer to <http://unlicense.org>

mod xcm_config;

// Substrate and Polkadot dependencies
use cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
use cumulus_primitives_core::{AggregateMessageOrigin, ParaId};
use frame_support::{
	derive_impl,
	dispatch::DispatchClass,
	parameter_types,
	traits::{ConstBool, ConstU32, ConstU64, ConstU8, EitherOfDiverse, TransformOrigin, OnUnbalanced},
	weights::{ConstantMultiplier, Weight},
	PalletId,
};
use frame_support::traits::{Contains, Currency, Imbalance};
use frame_system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot,
};
use pallet_xcm::{EnsureXcm, IsVoiceOfBody};
use parachains_common::message_queue::{NarrowOriginToSibling, ParaIdToSibling};
use polkadot_runtime_common::{
	xcm_sender::NoPriceForMessageDelivery, BlockHashCount,
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_runtime::{Perbill, Percent, Perquintill, FixedPointNumber, traits::{Bounded, One }};
use sp_version::RuntimeVersion;
use xcm::latest::prelude::BodyId;

// Local module imports
use super::{weights::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight}, AccountId, Aura, Balance, Balances, Block, BlockNumber, CollatorSelection, Hash, MessageQueue, Nonce, PalletInfo, ParachainSystem, Runtime, RuntimeCall, RuntimeEvent, RuntimeFreezeReason, RuntimeHoldReason, RuntimeOrigin, RuntimeTask, Session, SessionKeys, System, XcmpQueue, AVERAGE_ON_INITIALIZE_RATIO, BLOCK_PROCESSING_VELOCITY, EXISTENTIAL_DEPOSIT, HOURS, MAXIMUM_BLOCK_WEIGHT, NORMAL_DISPATCH_RATIO, RELAY_CHAIN_SLOT_DURATION_MILLIS, SLOT_DURATION, UNINCLUDED_SEGMENT_CAPACITY, VERSION, LoAuthorityList, CommunityTreasuryPalletId, LogionTreasuryPalletId};
use xcm_config::{RelayLocation, XcmOriginToTransactDispatchOrigin};

// Logion imports
use logion_shared::{DistributionKey, RewardDistributor as RewardDistributorTrait};
use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment, CurrencyAdapter};
use sp_runtime::traits::AccountIdConversion;
use crate::weights;
use crate::{LGNT, NANO_LGNT, MILLI_LGNT};

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;

	// This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
	//  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
	// `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
	// the lazy contract deletion.
	pub RuntimeBlockLength: BlockLength =
		BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
	pub const SS58Prefix: u16 = 2021;
}

pub struct BaseCallFilter;
impl Contains<RuntimeCall> for BaseCallFilter {
	fn contains(call: &RuntimeCall) -> bool {
		match call {
			RuntimeCall::Recovery(pallet_recovery::Call::create_recovery{ .. }) => false,
			RuntimeCall::Multisig(pallet_multisig::Call::approve_as_multi{ .. }) => false,
			RuntimeCall::Multisig(pallet_multisig::Call::as_multi{ .. }) => false,
			_ => true
		}
	}
}

/// The default types are being injected by [`derive_impl`](`frame_support::derive_impl`) from
/// [`ParaChainDefaultConfig`](`struct@frame_system::config_preludes::ParaChainDefaultConfig`),
/// but overridden as needed.
#[derive_impl(frame_system::config_preludes::ParaChainDefaultConfig as frame_system::DefaultConfig)]
impl frame_system::Config for Runtime {
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = BaseCallFilter;
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The index type for storing how many extrinsics an account has signed.
	type Nonce = Nonce;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The block type.
	type Block = Block;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// Runtime version.
	type Version = Version;
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = RuntimeBlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = RuntimeBlockLength;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The action to take on a Runtime Upgrade
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
	/// The maximum number of consumers allowed on a single account.
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
	type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type EventHandler = (CollatorSelection,);
}

parameter_types! {
	pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
	type FreezeIdentifier = [u8; 8];
	type MaxFreezes = ();
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
}

parameter_types! {
    pub const InclusionFeesDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(35),
        community_treasury_percent: Percent::from_percent(30),
        logion_treasury_percent: Percent::from_percent(35),
        loc_owner_percent: Percent::from_percent(0),
    };

	// Inflation: I=0,05 (5%)
	// Total supply: N=10^9
	// Block rate: B=12 (Number of seconds between 2 blocks)
	// The reward can be calculated as follows: N * (I / (3600 * 24 * 365 / B))
	// We thus mint 19 LGNT every block
    pub const InflationAmount: Balance = 19 * LGNT;
    pub const InflationDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(35),
        community_treasury_percent: Percent::from_percent(30),
        logion_treasury_percent: Percent::from_percent(35),
        loc_owner_percent: Percent::from_percent(0),
    };

	pub const FileStorageByteFee: Balance = 2000 * NANO_LGNT; // 2.0 LGNT per MB -> 0.000002 LGNT per B
	pub const FileStorageEntryFee: Balance = 0;
	pub const FileStorageFeeDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(80),
        community_treasury_percent: Percent::from_percent(20),
        logion_treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };

	pub const CertificateFee: Balance = 40 * MILLI_LGNT; // 0.04 LGNT per token
    pub const CertificateFeeDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(20),
        community_treasury_percent: Percent::from_percent(80),
        logion_treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };

	pub const ValueFeeDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(0),
        community_treasury_percent: Percent::from_percent(0),
        logion_treasury_percent: Percent::from_percent(100),
        loc_owner_percent: Percent::from_percent(0),
    };

    pub const RecurentFeeDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(0),
        community_treasury_percent: Percent::from_percent(0),
        logion_treasury_percent: Percent::from_percent(95),
        loc_owner_percent: Percent::from_percent(5),
    };

    pub const IdentityLocLegalFeeDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(0),
        community_treasury_percent: Percent::from_percent(0),
        logion_treasury_percent: Percent::from_percent(100),
        loc_owner_percent: Percent::from_percent(0),
    };

    pub const OtherLocLegalFeeDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(0),
        community_treasury_percent: Percent::from_percent(0),
        logion_treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(100),
    };
}
pub type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

pub struct RewardDistributor;
impl logion_shared::RewardDistributor<NegativeImbalance, Balance, AccountId, RuntimeOrigin, LoAuthorityList>
for RewardDistributor
{
	fn payout_community_treasury(reward: NegativeImbalance) {
		if reward != NegativeImbalance::zero() {
			Balances::resolve_creating(&CommunityTreasuryPalletId::get().into_account_truncating(), reward);
		}
	}

	fn payout_logion_treasury(reward: NegativeImbalance) {
		if reward != NegativeImbalance::zero() {
			Balances::resolve_creating(&LogionTreasuryPalletId::get().into_account_truncating(), reward);
		}
	}

	fn payout_to(reward: NegativeImbalance, account: &AccountId) {
		if reward != NegativeImbalance::zero() {
			Balances::resolve_creating(account, reward);
		}
	}
}

pub struct DealWithInclusionFees;
impl OnUnbalanced<NegativeImbalance> for DealWithInclusionFees {

	fn on_nonzero_unbalanced(fees: NegativeImbalance) {

		RewardDistributor::distribute(fees, InclusionFeesDistributionKey::get());
	}
}

parameter_types! {
	pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
	pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(75, 1000_000);
	pub MinimumMultiplier: Multiplier = Multiplier::one();
	pub MaximumMultiplier: Multiplier = Bounded::max_value();

	// The multiplier is set such as inclusion fees are ~2 LGNT on average.
	// Spreadsheet in /docs/inclusion_fees.ods contains the model that lead
	// to this result.
	//
	// This value will probably have to be adjusted once we have more
	// usage statistics available.
	pub const WeightToFeeMultiplier: Balance = 5_089_484_898;
}

/// This instance of TargetedFeeAdjustment is basically the same as
/// `polkadot_runtime_common::SlowAdjustingFeeUpdate`, with MinimumMultiplier = 1.
pub type SlowAdjustingFeeUpdate<R> = TargetedFeeAdjustment<
	R,
	TargetBlockFullness,
	AdjustmentVariable,
	MinimumMultiplier,
	MaximumMultiplier,
>;

pub type WeightToFee = ConstantMultiplier<Balance, WeightToFeeMultiplier>;

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = CurrencyAdapter<Balances, DealWithInclusionFees>;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightToFee = WeightToFee;
	type LengthToFee = ConstantMultiplier<Balance, WeightToFeeMultiplier>;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
}

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = weights::pallet_sudo::WeightInfo<Runtime>;
}

parameter_types! {
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const RelayOrigin: AggregateMessageOrigin = AggregateMessageOrigin::Parent;
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type OnSystemEvent = ();
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type OutboundXcmpMessageSource = XcmpQueue;
	type DmpQueue = frame_support::traits::EnqueueWithOrigin<MessageQueue, RelayOrigin>;
	type ReservedDmpWeight = ReservedDmpWeight;
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
	type ConsensusHook = cumulus_pallet_aura_ext::FixedVelocityConsensusHook<
		Runtime,
		RELAY_CHAIN_SLOT_DURATION_MILLIS,
		BLOCK_PROCESSING_VELOCITY,
		UNINCLUDED_SEGMENT_CAPACITY,
	>;
}

impl parachain_info::Config for Runtime {}

parameter_types! {
	pub MessageQueueServiceWeight: Weight = Perbill::from_percent(35) * RuntimeBlockWeights::get().max_block;
}

impl pallet_message_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type MessageProcessor = pallet_message_queue::mock_helpers::NoopMessageProcessor<
		cumulus_primitives_core::AggregateMessageOrigin,
	>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type MessageProcessor = xcm_builder::ProcessXcmMessage<
		AggregateMessageOrigin,
		xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
		RuntimeCall,
	>;
	type Size = u32;
	// The XCMP queue pallet is only ever able to handle the `Sibling(ParaId)` origin:
	type QueueChangeHandler = NarrowOriginToSibling<XcmpQueue>;
	type QueuePausedQuery = NarrowOriginToSibling<XcmpQueue>;
	type HeapSize = sp_core::ConstU32<{ 64 * 1024 }>;
	type MaxStale = sp_core::ConstU32<8>;
	type ServiceWeight = MessageQueueServiceWeight;
	type IdleMaxServiceWeight = ();
}

impl cumulus_pallet_aura_ext::Config for Runtime {}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = ();
	// Enqueue XCMP messages from siblings for later processing.
	type XcmpQueue = TransformOrigin<MessageQueue, AggregateMessageOrigin, ParaId, ParaIdToSibling>;
	type MaxInboundSuspended = sp_core::ConstU32<1_000>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = ();
	type PriceForSiblingDelivery = NoPriceForMessageDelivery<ParaId>;
}

parameter_types! {
	pub const Period: u32 = 6 * HOURS;
	pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	// we don't have stash and controller, thus we don't need the convert as well.
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type SessionManager = CollatorSelection;
	// Essentially just Aura, but let's be pedantic.
	type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>; // No benchmark available
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = ConstU32<100_000>;
	type AllowMultipleBlocksPerSlot = ConstBool<false>;
	type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Self>;
}

parameter_types! {
	pub const PotId: PalletId = PalletId(*b"PotStake");
	pub const MaxCandidates: u32 = 1000;
	pub const SessionLength: BlockNumber = 6 * HOURS;
	pub const MaxInvulnerables: u32 = 100;
	// StakingAdmin pluralistic body.
	pub const StakingAdminBodyId: BodyId = BodyId::Defense;
}

/// We allow root and the StakingAdmin to execute privileged collator selection operations.
pub type CollatorSelectionUpdateOrigin = EitherOfDiverse<
	EnsureRoot<AccountId>,
	EnsureXcm<IsVoiceOfBody<RelayLocation, StakingAdminBodyId>>,
>;

impl pallet_collator_selection::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type UpdateOrigin = CollatorSelectionUpdateOrigin;
	type PotId = PotId;
	type MaxCandidates = MaxCandidates;
	type MinEligibleCollators = ConstU32<4>;
	type MaxInvulnerables = MaxInvulnerables;
	// should be a multiple of session or things will get inconsistent
	type KickThreshold = Period;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ValidatorRegistration = Session;
	type WeightInfo = ();
}
