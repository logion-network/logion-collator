use cumulus_primitives_core::ParaId;
use logion_runtime::{AccountId, AuraId, Signature, EXISTENTIAL_DEPOSIT, Balance, LGNT, SS58Prefix};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::str::FromStr;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
	sc_service::GenericChainSpec<logion_runtime::RuntimeGenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> logion_runtime::SessionKeys {
	logion_runtime::SessionKeys { aura: keys }
}

pub fn logion_config() -> ChainSpec {
	const ROOT_PUBLIC_SR25519: &str = "5GzrECvUzFng58zzFvqVJvpE2MRnjU9vgh4iJMhqrLBSRRAv";

	const NODE01_PUBLIC_SR25519: &str = "5DjzFDhFidvGCuuy6i8Lsi4XyruYjxTTkJKb1o7XzVdMNPVb";
	const NODE02_PUBLIC_SR25519: &str = "5DoD9n61SssFiWQDTD7bz1eX3KCxZJ6trVj2GsDwMi2PqP85";
	const NODE03_PUBLIC_SR25519: &str = "5CJTSSJ4v1RAauZpeqTeddyui4wESZZqPor33wum9aKuQXZC";
	const NODE04_PUBLIC_SR25519: &str = "5EF6NVgMfRRFMRnNEByNJsQJfD1fokamB9kq2J7SLRVraJrg";
	const NODE05_PUBLIC_SR25519: &str = "5G7Gtz7iLn3z5PkqfweQJp5jJdV3u8ix7qWcGS4bs38EH1W3";
	const NODE06_PUBLIC_SR25519: &str = "5EZRCd7FybQKthaD2XuV21RAdU5LqPoveiSdrz9Z6JCstoSH";
	const NODE07_PUBLIC_SR25519: &str = "5DqwojnfMTfZvERe9SJr3e1ApfaAY8Lye8Tch6WfnmxkfJfw";
	const NODE08_PUBLIC_SR25519: &str = "5GRie9PZxqzAmPoJAgiLjzgxzFi7LW2ez1TNzzWdUN6yh8Jd";
	const NODE09_PUBLIC_SR25519: &str = "5CSsbWDRbV5eYuWZsSrFcfkrEnGAjhbmyGJjjpRkjQ7s5dCd";
	const NODE10_PUBLIC_SR25519: &str = "5FYe8QZfCUZVh6BeuAziATXNcowbZuSngqrguGahscdbhhnz";
	const NODE11_PUBLIC_SR25519: &str = "5DRbgvZC3LEeJmRe893Q3UEwP2H1DPv5x8ofFgcxihCLu3oL";
	const NODE12_PUBLIC_SR25519: &str = "5F6h3kuXnhpwkVzDKRd65jrSu53UecKNRdHcgCGFiAbAPWMt";

	ChainSpec::from_genesis(
		"logion network",
		"logion",
		ChainType::Live,
		move || {
			build_genesis_config(
				vec![
					(
						AccountId::from_str(NODE01_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE01_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE02_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE02_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE03_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE03_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE04_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE04_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE05_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE05_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE06_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE06_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE07_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE07_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE08_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE08_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE09_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE09_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE10_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE10_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE11_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE11_PUBLIC_SR25519).unwrap()),
					),
					(
						AccountId::from_str(NODE12_PUBLIC_SR25519).unwrap(),
						AuraId::from(sr25519::Public::from_str(NODE12_PUBLIC_SR25519).unwrap()),
					),
				],
				vec![
					(
						AccountId::from_str(ROOT_PUBLIC_SR25519).unwrap(),
						1_000_000_000 * LGNT,
					),
				],
				main_para_id().into(),
				AccountId::from_str(ROOT_PUBLIC_SR25519).unwrap(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("logion"),
		// Fork ID
		None,
		// Properties
		Some(default_properties("LGNT")),
		// Extensions
		Extensions {
			relay_chain: "polkadot".into(),
			para_id: main_para_id(),
		},
	)
}

fn main_para_id() -> u32 {
	3341
}

const DEFAULT_TEST_BALANCE: Balance = 1 << 60;

pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Local Logion",
		// ID
		"local_logion",
		ChainType::Development,
		move || {
			build_genesis_config(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
				],
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
						DEFAULT_TEST_BALANCE
					),
				],
				test_parachain_id(),
				get_account_id_from_seed::<sr25519::Public>("Alice"),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("logion"),
		// Fork ID
		None,
		// Properties
		Some(default_properties("LGNTD")),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: test_parachain_id().into(),
		},
	)
}

pub fn logion_dev_config() -> ChainSpec {
	testnet_config(
		"Logion DEV",
		"logion_dev",
		"LGNTD"
	)
}

fn testnet_config(name: &str, id: &str, symbol: &str) -> ChainSpec {
	ChainSpec::from_genesis(
		name,
		id,
		ChainType::Live,
		move || {
			build_genesis_config(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						get_collator_keys_from_seed("Charlie"),
					),
				],
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
						DEFAULT_TEST_BALANCE
					),
				],
				test_parachain_id(),
				get_account_id_from_seed::<sr25519::Public>("Alice"),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("logion"),
		// Fork ID
		None,
		// Properties
		Some(default_properties(symbol)),
		// Extensions
		Extensions {
			relay_chain: "rococo_local_testnet".into(),
			para_id: test_parachain_id().into(),
		},
	)
}

pub fn logion_test_config() -> ChainSpec {
	testnet_config(
		"Logion TEST",
		"logion_test",
		"LGNTT"
	)
}

pub fn local_config() -> ChainSpec {
	ChainSpec::from_genesis(
		// Name
		"Local Logion",
		// ID
		"local_logion",
		ChainType::Local,
		move || {
			build_genesis_config(
				// initial collators.
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed("Alice"),
					),
				],
				vec![
					(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
						DEFAULT_TEST_BALANCE
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
						DEFAULT_TEST_BALANCE
					),
				],
				test_parachain_id(),
				get_account_id_from_seed::<sr25519::Public>("Alice"),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some("logion"),
		// Fork ID
		None,
		// Properties
		Some(default_properties("LGNT")),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: test_parachain_id().into(),
		},
	)
}

fn build_genesis_config(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<(AccountId, Balance)>,
	id: ParaId,
	root_key: AccountId,
) -> logion_runtime::RuntimeGenesisConfig {
	logion_runtime::RuntimeGenesisConfig {
		system: logion_runtime::SystemConfig {
			code: logion_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			..Default::default()
		},
		balances: logion_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k.0, k.1)).collect(),
		},
		parachain_info: logion_runtime::ParachainInfoConfig {
			parachain_id: id,
			..Default::default()
		},
		collator_selection: logion_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: logion_runtime::SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						template_session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: logion_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
			..Default::default()
		},
		sudo: logion_runtime::SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		community_treasury: Default::default(),
		logion_treasury: Default::default(),
		vesting: Default::default(),
	}
}

fn default_properties(symbol: &str) -> sc_service::Properties {
	let mut props : sc_service::Properties = sc_service::Properties::new();
	props.insert("tokenSymbol".into(), symbol.into());
	props.insert("tokenDecimals".into(), 18.into());
	props.insert("ss58Format".into(), SS58Prefix::get().into());
	return props;
}

fn test_parachain_id() -> ParaId {
	2000.into()
}
