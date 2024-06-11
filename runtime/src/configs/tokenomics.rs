use frame_support::parameter_types;
use frame_support::traits::{Currency, Imbalance, OnUnbalanced};
use frame_support::weights::ConstantMultiplier;
use logion_shared::{DistributionKey, RewardDistributor as RewardDistributorTrait};
use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};
use sp_runtime::{FixedPointNumber, Percent, Perquintill};
use sp_runtime::traits::{AccountIdConversion, Bounded, One};

use crate::{AccountId, Balance, Balances, LoAuthorityList, RuntimeOrigin};
use crate::configs::logion_config::{CommunityTreasuryPalletId, LogionTreasuryPalletId};

pub const NANO_LGNT: Balance = 1_000_000_000;
pub const MICRO_LGNT: Balance = 1_000 * NANO_LGNT;
pub const MILLI_LGNT: Balance = 1_000 * MICRO_LGNT;
pub const LGNT: Balance = 1_000 * MILLI_LGNT;

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
