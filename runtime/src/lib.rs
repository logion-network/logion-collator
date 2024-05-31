#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod apis;
mod configs;
mod weights;

use codec::{Decode, Encode, MaxEncodedLen};
use sp_core::{H160, H256};
use sp_runtime::{create_runtime_str, generic, impl_opaque_keys, traits::{
	AccountIdConversion, BlakeTwo256, ConvertInto, IdentifyAccount,
	IdentityLookup, Verify,
}, MultiSignature};

use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;

use frame_support::{
	construct_runtime,
	parameter_types,
	traits::{
		ConstU32,
		tokens::{PayFromAccount, UnityAssetBalanceConversion},
		WithdrawReasons,
	},
	weights::{
		constants::WEIGHT_REF_TIME_PER_SECOND, Weight,
	},
	PalletId,
};
use frame_system::{	EnsureRoot };
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
pub use sp_runtime::{MultiAddress, Perbill, Permill, Percent};
use scale_info::TypeInfo;

#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;

// Logion imports
use logion_shared::{CreateRecoveryCallFactory, MultisigApproveAsMultiCallFactory, MultisigAsMultiCallFactory};
use crate::configs::{InflationDistributionKey, RewardDistributor, InflationAmount, OtherLocLegalFeeDistributionKey, IdentityLocLegalFeeDistributionKey, RecurentFeeDistributionKey, ValueFeeDistributionKey, CertificateFeeDistributionKey, CertificateFee, FileStorageFeeDistributionKey, FileStorageByteFee, FileStorageEntryFee};

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// LOC ID, compatible with UUIDs
pub type LocId = u128;

/// Ethereum Address
pub type EthereumAddress = H160;

/// Sponsorship ID, compatible with UUIDs
pub type SponsorshipId = u128;

/// A given token's total supply type
pub type TokenIssuance = u64;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
	cumulus_primitives_storage_weight_reclaim::StorageWeightReclaim<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic =
	generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra>;

/// All migrations of the runtime, aside from the ones declared in the pallets.
///
/// This can be a tuple of types, each implementing `OnRuntimeUpgrade`.
#[allow(unused_parens)]
type Migrations = (
);

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	Migrations,
>;

pub type SS58Prefix = crate::configs::SS58Prefix;

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;
	use sp_runtime::{
		traits::{Hash as HashT},
	};

	pub use sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
	/// Opaque block hash type.
	pub type Hash = <BlakeTwo256 as HashT>::Output;
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}

#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("logion"),
	impl_name: create_runtime_str!("logion"),
	authoring_version: 1,
	spec_version: 000_004_000,
	impl_version: 0,
	apis: apis::RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 1,
};

/// This determines the average expected block time that we are targeting.
/// Blocks will be produced at a minimum duration defined by `SLOT_DURATION`.
/// `SLOT_DURATION` is picked up by `pallet_timestamp` which is in turn picked
/// up by `pallet_aura` to implement `fn slot_duration()`.
///
/// Change this to adjust the block time.
pub const MILLISECS_PER_BLOCK: u64 = 12000;

// NOTE: Currently it is not possible to change the slot duration after the chain has started.
//       Attempting to do so will brick block production.
pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000_000_000;
pub const MILLIUNIT: Balance = 1_000_000_000_000_000;
pub const MICROUNIT: Balance = 1_000_000_000_000;

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

pub const NANO_LGNT: Balance = 1_000_000_000;
pub const MICRO_LGNT: Balance = 1_000 * NANO_LGNT;
pub const MILLI_LGNT: Balance = 1_000 * MICRO_LGNT;
pub const LGNT: Balance = 1_000 * MILLI_LGNT;

/// We allow for 0.5 of a second of compute with a 12 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
	WEIGHT_REF_TIME_PER_SECOND.saturating_div(2),
	cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64,
);

/// Maximum number of blocks simultaneously accepted by the Runtime, not yet included
/// into the relay chain.
const UNINCLUDED_SEGMENT_CAPACITY: u32 = 1;
/// How many parachain blocks are processed by the relay chain per parent. Limits the
/// number of blocks authored per slot.
const BLOCK_PROCESSING_VELOCITY: u32 = 1;
/// Relay chain slot duration, in milliseconds.
const RELAY_CHAIN_SLOT_DURATION_MILLIS: u32 = 6000;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

parameter_types! {
	#[derive(Debug, Eq, Clone, PartialEq, TypeInfo)]
	pub const MaxBaseUrlLen: u32 = 2000;
	pub const MaxWellKnownNodes: u32 = 100;
	#[derive(Debug, Eq, Clone, PartialEq, TypeInfo, PartialOrd, Ord)]
	pub const MaxPeerIdLength: u32 = 128;
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, Copy, MaxEncodedLen)]
pub enum Region {
	Europe,
}

impl sp_std::str::FromStr for Region {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"Europe" => Ok(Region::Europe),
			_ => Err(()),
		}
	}
}

impl Default for Region {

	fn default() -> Self {
		Self::Europe
	}
}

impl pallet_lo_authority_list::Config for Runtime {
	type AddOrigin = EnsureRoot<AccountId>;
	type RemoveOrigin = EnsureRoot<AccountId>;
	type UpdateOrigin = EnsureRoot<AccountId>;
	type Region = Region;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_lo_authority_list::WeightInfo<Runtime>;
	type MaxBaseUrlLen = MaxBaseUrlLen;
	type MaxNodes = MaxWellKnownNodes;
	type MaxPeerIdLength = MaxPeerIdLength;
}

parameter_types! {
	pub const MaxAccountLocs: u32 = 200;
	#[derive(TypeInfo)]
	pub const MaxLocMetadata: u32 = 50;
	#[derive(TypeInfo)]
	pub const MaxLocFiles: u32 = 50;
	#[derive(TypeInfo)]
	pub const MaxLocLinks: u32 = 50;
	pub const MaxCollectionItemFiles: u32 = 10;
	pub const MaxCollectionItemTCs: u32 = 10;
	pub const MaxTokensRecordFiles: u32 = 10;
}

pub struct SHA256;
impl Hasher<H256> for SHA256 {

	fn hash(data: &Vec<u8>) -> H256 {
		let bytes = sha2_256(data);
		H256(bytes)
	}
}

impl pallet_logion_loc::Config for Runtime {
	type LocId = LocId;
	type RuntimeEvent = RuntimeEvent;
	type Hash = Hash;
	type Hasher = SHA256;
	type IsLegalOfficer = LoAuthorityList;
	type CollectionItemId = Hash;
	type TokensRecordId = Hash;
	type MaxAccountLocs = MaxAccountLocs;
	type MaxLocMetadata = MaxLocMetadata;
	type MaxLocFiles = MaxLocFiles;
	type MaxLocLinks = MaxLocLinks;
	type MaxCollectionItemFiles = MaxCollectionItemFiles;
	type MaxCollectionItemTCs = MaxCollectionItemTCs;
	type MaxTokensRecordFiles = MaxTokensRecordFiles;
	type WeightInfo = weights::pallet_logion_loc::WeightInfo<Runtime>;
	type Currency = Balances;
	type FileStorageByteFee = FileStorageByteFee;
	type FileStorageEntryFee = FileStorageEntryFee;
	type RewardDistributor = RewardDistributor;
	type FileStorageFeeDistributionKey = FileStorageFeeDistributionKey;
	type EthereumAddress = EthereumAddress;
	type SponsorshipId = SponsorshipId;
	type CertificateFee = CertificateFee;
	type CertificateFeeDistributionKey = CertificateFeeDistributionKey;
	type TokenIssuance = TokenIssuance;
	type ValueFeeDistributionKey = ValueFeeDistributionKey;
	type CollectionItemFeeDistributionKey = RecurentFeeDistributionKey;
	type TokensRecordFeeDistributionKey = RecurentFeeDistributionKey;
	type IdentityLocLegalFeeDistributionKey = IdentityLocLegalFeeDistributionKey;
	type TransactionLocLegalFeeDistributionKey = OtherLocLegalFeeDistributionKey;
	type CollectionLocLegalFeeDistributionKey = OtherLocLegalFeeDistributionKey;
	#[cfg(feature = "runtime-benchmarks")]
	type LocIdFactory = ();
	#[cfg(feature = "runtime-benchmarks")]
	type CollectionItemIdFactory = ();
	#[cfg(feature = "runtime-benchmarks")]
	type TokensRecordIdFactory = ();
	#[cfg(feature = "runtime-benchmarks")]
	type EthereumAddressFactory = ();
	#[cfg(feature = "runtime-benchmarks")]
	type SponsorshipIdFactory = ();
}

parameter_types! {
	pub const RecoveryConfigDepositBase: u64 = 10;
	pub const RecoveryFriendDepositFactor: u64 = 1;
	pub const MaxFriends: u16 = 3;
	pub const RecoveryDeposit: u64 = 10;
}

impl pallet_recovery::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type ConfigDepositBase = RecoveryConfigDepositBase;
	type FriendDepositFactor = RecoveryFriendDepositFactor;
	type MaxFriends = MaxFriends;
	type RecoveryDeposit = RecoveryDeposit;
	type WeightInfo = weights::pallet_recovery::WeightInfo<Runtime>;
}

pub struct PalletRecoveryCreateRecoveryCallFactory;
impl CreateRecoveryCallFactory<RuntimeOrigin, AccountId, BlockNumber> for PalletRecoveryCreateRecoveryCallFactory {
	type Call = RuntimeCall;

	fn build_create_recovery_call(legal_officers: Vec<AccountId>, threshold: u16, delay_period: BlockNumber) -> RuntimeCall {
		RuntimeCall::Recovery(pallet_recovery::Call::create_recovery{ friends: legal_officers, threshold, delay_period })
	}
}

#[cfg(feature = "runtime-benchmarks")]
use pallet_verified_recovery::benchmarking::{
	SetupBenchmark,
};
#[cfg(feature = "runtime-benchmarks")]
pub struct VerifiedRecoverySetupBenchmark;
#[cfg(feature = "runtime-benchmarks")]
impl SetupBenchmark<AccountId> for VerifiedRecoverySetupBenchmark {

	fn setup() -> (AccountId, Vec<AccountId>) {
		let requester: AccountId = [0u8;32].into();
		Balances::make_free_balance_be(&requester, Balance::max_value());

		let loc_id1: LocId = 0;
		let legal_officer_id1 = LoAuthorityList::legal_officers()[0].clone();
		Self::setup_loc(loc_id1, &requester, &legal_officer_id1);

		let loc_id2: LocId = 1;
		let legal_officer_id2 = LoAuthorityList::legal_officers()[1].clone();
		Self::setup_loc(loc_id2, &requester, &legal_officer_id2);
		(
			requester,
			Vec::from([
				legal_officer_id1,
				legal_officer_id2,
			])
		)
	}
}
#[cfg(feature = "runtime-benchmarks")]
impl VerifiedRecoverySetupBenchmark {
	fn setup_loc(loc_id: LocId, requester: &AccountId, legal_officer_id: &AccountId) {
		let _ = LogionLoc::create_polkadot_identity_loc(
			RuntimeOrigin::signed(requester.clone()),
			loc_id,
			legal_officer_id.clone(),
			0u32.into(),
			ItemsParams::empty(),
		);
		let _ = LogionLoc::close(
			RuntimeOrigin::signed(legal_officer_id.clone()),
			loc_id,
			None,
			false,
		);
	}
}

impl pallet_verified_recovery::Config for Runtime {
	type LocId = LocId;
	type CreateRecoveryCallFactory = PalletRecoveryCreateRecoveryCallFactory;
	type LocQuery = LogionLoc;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_verified_recovery::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type SetupBenchmark = VerifiedRecoverySetupBenchmark;
}

parameter_types! {
	pub const MultiSigDepositBase: Balance = 500;
	pub const MultiSigDepositFactor: Balance = 100;
	pub const MaxSignatories: u16 = 20;
}

impl pallet_multisig::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type Currency = Balances;
	type DepositBase = MultiSigDepositBase;
	type DepositFactor = MultiSigDepositFactor;
	type MaxSignatories = MaxSignatories;
	type WeightInfo = weights::pallet_multisig::WeightInfo<Runtime>;
}

pub struct PalletMultisigApproveAsMultiCallFactory;
impl MultisigApproveAsMultiCallFactory<RuntimeOrigin, AccountId, Timepoint<BlockNumber>> for PalletMultisigApproveAsMultiCallFactory {
	type Call = RuntimeCall;

	fn build_approve_as_multi_call(
		threshold: u16,
		other_signatories: Vec<AccountId>,
		maybe_timepoint: Option<Timepoint<BlockNumber>>,
		call_hash: [u8; 32],
		max_weight: Weight
	) -> RuntimeCall {
		RuntimeCall::Multisig(pallet_multisig::Call::approve_as_multi{ threshold, other_signatories, maybe_timepoint, call_hash, max_weight })
	}
}

pub struct PalletMultisigAsMultiCallFactory;
impl MultisigAsMultiCallFactory<RuntimeOrigin, AccountId, Timepoint<BlockNumber>> for PalletMultisigAsMultiCallFactory {
	type Call = RuntimeCall;

	fn build_as_multi_call(
		threshold: u16,
		other_signatories: Vec<AccountId>,
		maybe_timepoint: Option<Timepoint<BlockNumber>>,
		call: Box<Self::Call>,
		max_weight: Weight,
	) -> RuntimeCall {
		RuntimeCall::Multisig(pallet_multisig::Call::as_multi{ threshold, other_signatories, maybe_timepoint, call, max_weight })
	}
}

impl pallet_logion_vault::Config for Runtime {
	type RuntimeCall = RuntimeCall;
	type MultisigApproveAsMultiCallFactory = PalletMultisigApproveAsMultiCallFactory;
	type MultisigAsMultiCallFactory = PalletMultisigAsMultiCallFactory;
	type IsLegalOfficer = LoAuthorityList;
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_multisig::WeightInfo<Runtime>;
}

#[cfg(feature = "runtime-benchmarks")]
use pallet_logion_vote::benchmarking::{
	LocSetup,
};
#[cfg(feature = "runtime-benchmarks")]
use logion_shared::IsLegalOfficer;
use pallet_logion_loc::Hasher;
#[cfg(feature = "runtime-benchmarks")]
use pallet_logion_loc::ItemsParams;
use pallet_multisig::Timepoint;
use sp_io::hashing::sha2_256;

#[cfg(feature = "runtime-benchmarks")]
pub struct VoteLocSetup;
#[cfg(feature = "runtime-benchmarks")]
impl LocSetup<LocId, AccountId> for VoteLocSetup {

	fn setup_vote_loc() -> (LocId, AccountId) {
		let loc_id: LocId = 0;
		let requester: AccountId = [0u8;32].into();
		Balances::make_free_balance_be(&requester, Balance::max_value());
		let legal_officer_id = LoAuthorityList::legal_officers()[0].clone();
		let _ = LogionLoc::create_polkadot_identity_loc(
			RuntimeOrigin::signed(requester),
			loc_id,
			legal_officer_id.clone(),
			0u32.into(),
			ItemsParams::empty(),
		);
		let _ = LogionLoc::close(
			RuntimeOrigin::signed(legal_officer_id.clone()),
			loc_id,
			None,
			false,
		);
		(loc_id, legal_officer_id)
	}
}

parameter_types! {
	#[derive(Debug, PartialEq, TypeInfo)]
	pub const MaxBallots: u32 = 12;
}

impl pallet_logion_vote::Config for Runtime {
	type LocId = LocId;
	type RuntimeEvent = RuntimeEvent;
	type IsLegalOfficer = LoAuthorityList;
	type LocValidity = LogionLoc;
	type LocQuery = LogionLoc;
	type LegalOfficerCreation = LoAuthorityList;
	type MaxBallots = MaxBallots;
	type WeightInfo = weights::pallet_logion_vote::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type LocSetup = VoteLocSetup;
}

parameter_types! {
	pub const LogionTreasuryPalletId: PalletId = PalletId(*b"lg/lgtrs");
    pub LogionTreasuryAccountId: AccountId = LogionTreasuryPalletId::get().into_account_truncating();
    pub const CommunityTreasuryPalletId: PalletId = PalletId(*b"lg/cmtrs");
    pub CommunityTreasuryAccountId: AccountId = CommunityTreasuryPalletId::get().into_account_truncating();

    pub const ProposalBond: Permill = Permill::from_percent(5);
    pub const ProposalBondMinimum: Balance = 100 * LGNT;
    pub const SpendPeriod: BlockNumber = 1 * DAYS;
	pub const SpendPayoutPeriod: BlockNumber = 30 * DAYS;
}

type LogionTreasuryType = pallet_treasury::Instance1;
impl pallet_treasury::Config<LogionTreasuryType> for Runtime {
	type Currency = Balances;
	type ApproveOrigin = EnsureRoot<AccountId>;
	type RejectOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type OnSlash = LogionTreasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ();
	type SpendPeriod = SpendPeriod;
	type Burn = ();
	type PalletId = LogionTreasuryPalletId;
	type BurnDestination = ();
	type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>; // Benchmark broken
	type SpendFunds = ();
	type MaxApprovals = ConstU32<100>;
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
	type AssetKind = ();
	type Beneficiary = AccountId;
	type BeneficiaryLookup = IdentityLookup<AccountId>;
	type Paymaster = PayFromAccount<Balances, LogionTreasuryAccountId>;
	type BalanceConverter = UnityAssetBalanceConversion;
	type PayoutPeriod = SpendPayoutPeriod;
}

type CommunityTreasuryType = pallet_treasury::Instance2;
impl pallet_treasury::Config<CommunityTreasuryType> for Runtime {
	type Currency = Balances;
	type ApproveOrigin = EnsureRoot<AccountId>;
	type RejectOrigin = EnsureRoot<AccountId>;
	type RuntimeEvent = RuntimeEvent;
	type OnSlash = CommunityTreasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ();
	type SpendPeriod = SpendPeriod;
	type Burn = ();
	type PalletId = CommunityTreasuryPalletId;
	type BurnDestination = ();
	type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>; // Benchmark broken
	type SpendFunds = ();
	type MaxApprovals = ConstU32<100>;
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
	type AssetKind = ();
	type Beneficiary = AccountId;
	type BeneficiaryLookup = IdentityLookup<AccountId>;
	type Paymaster = PayFromAccount<Balances, CommunityTreasuryAccountId>;
	type BalanceConverter = UnityAssetBalanceConversion;
	type PayoutPeriod = SpendPayoutPeriod;
}

impl pallet_block_reward::Config for Runtime {
	type Currency = Balances;
	type RewardAmount = InflationAmount;
	type RewardDistributor = RewardDistributor;
	type DistributionKey = InflationDistributionKey;
	type IsLegalOfficer = LoAuthorityList;
}

impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = weights::pallet_utility::WeightInfo<Runtime>;
}

parameter_types! {
	pub const MinVestedTransfer: Balance = 1 * LGNT;
	pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons =
		WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
}

impl pallet_vesting::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BlockNumberToBalance = ConvertInto;
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
	type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
	type BlockNumberProvider = System;
	const MAX_VESTING_SCHEDULES: u32 = 28;
}

// Create the runtime by composing the FRAME pallets that were previously configured.
construct_runtime!(
	pub enum Runtime {
		// System support stuff.
		System: frame_system = 0,
		ParachainSystem: cumulus_pallet_parachain_system = 1,
		Timestamp: pallet_timestamp = 2,
		ParachainInfo: parachain_info = 3,

		// Monetary stuff.
		Balances: pallet_balances = 10,
		TransactionPayment: pallet_transaction_payment = 11,

		// Governance
		Sudo: pallet_sudo = 15,
		LogionTreasury: pallet_treasury::<Instance1> = 16,
		CommunityTreasury: pallet_treasury::<Instance2> = 17,

		// Collator support. The order of these 4 are important and shall not change.
		Authorship: pallet_authorship = 20,
		CollatorSelection: pallet_collator_selection = 21,
		Session: pallet_session = 22,
		Aura: pallet_aura = 23,
		AuraExt: cumulus_pallet_aura_ext = 24,

		// XCM helpers.
		XcmpQueue: cumulus_pallet_xcmp_queue = 30,
		PolkadotXcm: pallet_xcm = 31,
		CumulusXcm: cumulus_pallet_xcm = 32,
		MessageQueue: pallet_message_queue = 33,

		// Misc helpers.
		Utility: pallet_utility = 40,

		// Vesting.
		Vesting: pallet_vesting = 50,

		// Logion.
		LogionLoc: pallet_logion_loc = 60,
		BlockReward: pallet_block_reward = 61,
		LoAuthorityList: pallet_lo_authority_list = 62,
		Vote: pallet_logion_vote = 63,
		Recovery: pallet_recovery = 64,
		VerifiedRecovery: pallet_verified_recovery = 65,
		Multisig:  pallet_multisig = 66,
		Vault: pallet_logion_vault = 67,

	}
);

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	frame_benchmarking::define_benchmarks!(
		[frame_benchmarking, BaselineBench::<Runtime>]
		[frame_system, SystemBench::<Runtime>]
		[pallet_balances, Balances]
		[pallet_session, SessionBench::<Runtime>]
		[pallet_timestamp, Timestamp]
		[pallet_collator_selection, CollatorSelection]
		[cumulus_pallet_xcmp_queue, XcmpQueue]
		[pallet_lo_authority_list, LoAuthorityList]
		[pallet_logion_loc, LogionLoc]
		[pallet_logion_vote, Vote]
		[pallet_multisig, Multisig]
		[pallet_recovery, Recovery]
		[pallet_sudo, Sudo]
		[pallet_timestamp, Timestamp]
		[pallet_validator_set, ValidatorSet]
		[pallet_verified_recovery, VerifiedRecovery]
		[pallet_utility, Utility]
	);
}

cumulus_pallet_parachain_system::register_validate_block! {
	Runtime = Runtime,
	BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
}
