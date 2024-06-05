use codec::{Decode, Encode, MaxEncodedLen};
use cumulus_primitives_core::Weight;
use frame_support::{PalletId, parameter_types};
use frame_support::pallet_prelude::{ConstU32, TypeInfo};
use frame_support::traits::tokens::{PayFromAccount, UnityAssetBalanceConversion};
use frame_support::traits::WithdrawReasons;
use frame_system::EnsureRoot;
use logion_shared::{CreateRecoveryCallFactory, MultisigApproveAsMultiCallFactory, MultisigAsMultiCallFactory};
#[cfg(feature = "runtime-benchmarks")]
use logion_shared::IsLegalOfficer;
use pallet_logion_loc::Hasher;
#[cfg(feature = "runtime-benchmarks")]
use pallet_logion_loc::ItemsParams;
#[cfg(feature = "runtime-benchmarks")]
use pallet_logion_vote::benchmarking::LocSetup;
use pallet_multisig::Timepoint;
#[cfg(feature = "runtime-benchmarks")]
use pallet_verified_recovery::benchmarking::SetupBenchmark;
use sp_core::H256;
use sp_io::hashing::sha2_256;
use sp_runtime::Permill;
use sp_runtime::traits::{AccountIdConversion, ConvertInto, IdentityLookup};
use sp_std::prelude::*;
use crate::{DAYS, LGNT};
use crate::{AccountId, Balance, Balances, BlockNumber, CommunityTreasury, EthereumAddress, Hash, LoAuthorityList, LocId, LogionLoc, LogionTreasury, OriginCaller, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin, SponsorshipId, System, TokenIssuance, weights};
use crate::configs::{CertificateFee, CertificateFeeDistributionKey, FileStorageByteFee, FileStorageEntryFee, FileStorageFeeDistributionKey, IdentityLocLegalFeeDistributionKey, InflationAmount, InflationDistributionKey, OtherLocLegalFeeDistributionKey, RecurentFeeDistributionKey, RewardDistributor, ValueFeeDistributionKey};

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
