use cumulus_primitives_core::ParaId;
use logion_runtime as runtime;
use runtime::{AccountId, AuraId, Signature, EXISTENTIAL_DEPOSIT, Balance, SS58Prefix};
use runtime::configs::tokenomics::{LGNT};
use pallet_lo_authority_list::GenesisHostData;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public, OpaquePeerId};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::str::FromStr;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<(), Extensions>;

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
pub fn template_session_keys(keys: AuraId) -> runtime::SessionKeys {
	runtime::SessionKeys { aura: keys }
}

const DEFAULT_TEST_BALANCE: Balance = 100_000_000_000_000_000_000_000;

pub fn development_config() -> ChainSpec {
    ChainSpec::builder(
        runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: test_parachain_id().into(),
        },
    )
    .with_name("Local Logion")
    .with_id("local_logion")
    .with_protocol_id("logion")
    .with_chain_type(ChainType::Development)
    .with_genesis_config_patch(logion_genesis(
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
		vec![ // Initial set of Logion Legal Officers
			  (
				  get_account_id_from_seed::<sr25519::Public>("Alice"),
				  GenesisHostData {
					  node_id: Some(OpaquePeerId(bs58::decode("12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2").into_vec().unwrap())),
					  base_url: None,
					  region: "Europe".into(),
				  }
			  ),
			  (
				  get_account_id_from_seed::<sr25519::Public>("Bob"),
				  GenesisHostData {
					  node_id: Some(OpaquePeerId(bs58::decode("12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust").into_vec().unwrap())),
					  base_url: None,
					  region: "Europe".into(),
				  }
			  ),
			  (
				  get_account_id_from_seed::<sr25519::Public>("Charlie"),
				  GenesisHostData {
					  node_id: Some(OpaquePeerId(bs58::decode("12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ").into_vec().unwrap())),
					  base_url: None,
					  region: "Europe".into(),
				  }
			  ),
		],
    ))
    .with_properties(default_properties("LGNT"))
    .build()
}

pub fn logion_dev_config() -> ChainSpec {
	testnet_config(
		"Logion DEV",
		"logion_dev",
		"LGNTD",
		"5FneAsYfxt1kDMR36j9bpYKx6HcqSs94x8YbCV9EsxXDHeKJ",
	)
}

fn testnet_config(name: &str, id: &str, symbol: &str, root_ref: &str) -> ChainSpec {
    let root = String::from(root_ref);
    ChainSpec::builder(
        logion_runtime::WASM_BINARY
            .expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "rococo_local_testnet".into(),
            para_id: test_parachain_id().into(),
        },
    )
        .with_name(name)
        .with_id(id)
        .with_chain_type(ChainType::Live)
        .with_genesis_config_patch(logion_genesis(
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
                (
                    AccountId::from_str(&root).unwrap(),
                    1_000_000_000 * LGNT,
                ),
            ],
            test_parachain_id(),
            AccountId::from_str(&root).unwrap(),
			vec![ // Initial set of Logion Legal Officers
				  (
					  get_account_id_from_seed::<sr25519::Public>("Alice"),
					  GenesisHostData {
						  node_id: Some(OpaquePeerId(bs58::decode("12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2").into_vec().unwrap())),
						  base_url: None,
						  region: "Europe".into(),
					  }
				  ),
				  (
					  get_account_id_from_seed::<sr25519::Public>("Bob"),
					  GenesisHostData {
						  node_id: Some(OpaquePeerId(bs58::decode("12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust").into_vec().unwrap())),
						  base_url: None,
						  region: "Europe".into(),
					  }
				  ),
				  (
					  get_account_id_from_seed::<sr25519::Public>("Charlie"),
					  GenesisHostData {
						  node_id: Some(OpaquePeerId(bs58::decode("12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ").into_vec().unwrap())),
						  base_url: None,
						  region: "Europe".into(),
					  }
				  ),
			],
        ))
        .with_protocol_id(id)
        .with_properties(default_properties(symbol))
        .build()
}

pub fn logion_test_config() -> ChainSpec {
	testnet_config(
		"Logion TEST",
		"logion_test",
		"LGNTT",
		"5FvTEcjmz4CpyCJCV1q4SeAwq4hn7HpoFd9ymUsivLGt9Y6T",
	)
}

pub fn local_config() -> ChainSpec {
	ChainSpec::builder(
		logion_runtime::WASM_BINARY
			.expect("WASM binary was not built, please build it!"),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: test_parachain_id().into(),
		},
	)
	.with_name("Local Logion")
	.with_id("local_logion")
	.with_protocol_id("logion")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_patch(logion_genesis(
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
		vec![ // Initial set of Logion Legal Officers
			  (
				  get_account_id_from_seed::<sr25519::Public>("Alice"),
				  GenesisHostData {
					  node_id: Some(OpaquePeerId(bs58::decode("12D3KooWBmAwcd4PJNJvfV89HwE48nwkRmAgo8Vy3uQEyNNHBox2").into_vec().unwrap())),
					  base_url: None,
					  region: "Europe".into(),
				  }
			  ),
			  (
				  get_account_id_from_seed::<sr25519::Public>("Bob"),
				  GenesisHostData {
					  node_id: Some(OpaquePeerId(bs58::decode("12D3KooWQYV9dGMFoRzNStwpXztXaBUjtPqi6aU76ZgUriHhKust").into_vec().unwrap())),
					  base_url: None,
					  region: "Europe".into(),
				  }
			  ),
			  (
				  get_account_id_from_seed::<sr25519::Public>("Charlie"),
				  GenesisHostData {
					  node_id: Some(OpaquePeerId(bs58::decode("12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ").into_vec().unwrap())),
					  base_url: None,
					  region: "Europe".into(),
				  }
			  ),
		],
	))
	.with_properties(default_properties("LGNT"))
	.build()
}

fn logion_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<(AccountId, Balance)>,
	id: ParaId,
	root_key: AccountId,
	legal_officers: Vec<(AccountId, GenesisHostData)>,
) -> serde_json::Value {
	serde_json::json!({
		"balances": {
			"balances": endowed_accounts.iter().cloned().map(|k| (k.0, k.1)).collect::<Vec<_>>(),
		},
		"parachainInfo": {
			"parachainId": id,
		},
		"collatorSelection": {
			"invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
			"candidacyBond": EXISTENTIAL_DEPOSIT * 16,
		},
		"session": {
			"keys": invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                 // account id
						acc,                         // validator id
						template_session_keys(aura), // session keys
					)
				})
				.collect::<Vec<_>>(),
		},
		"polkadotXcm": {
			"safeXcmVersion": Some(SAFE_XCM_VERSION),
		},
		"sudo": {
			"key": Some(root_key),
		},
		"loAuthorityList": {
			"legalOfficers": legal_officers,
		},
	})
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
