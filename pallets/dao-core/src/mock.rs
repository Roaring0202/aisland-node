use crate as pallet_dao_core;
use frame_support::{
	parameter_types,
	traits::{AsEnsureOriginWithArg, ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, Randomness},
};
use sp_core::H256;

use sp_runtime::{
	BuildStorage,
	traits::{BlakeTwo256, IdentityLookup},
};

type Block = frame_system::mocking::MockBlock<Test>;

pub(crate) type Balance = u128;
/// Index of a transaction in the chain.
pub type Nonce = u32;
// Account ID
pub type AccountId = u64;

pub const PLANCK: Balance = 1;
pub const MILLIDOT: Balance = PLANCK * 10_000_000;

pub const fn deposit(items: u32, bytes: u32) -> Balance {
	items as Balance * 150 * MILLIDOT + bytes as Balance * 60 * MILLIDOT
}

parameter_types! {
	pub const ExistentialDeposit: Balance = 1;
}

frame_support::construct_runtime!(
	pub struct Test {
		System: frame_system,
		Timestamp: pallet_timestamp,
		Balances: pallet_balances,
		Assets: pallet_dao_assets,
		DaoCore: pallet_dao_core,
		Contracts: pallet_contracts,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type Block = Block;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeCall = RuntimeCall;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Nonce = Nonce;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<2>;
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<0>;
	type WeightInfo = ();
}

impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type MaxHolds = ();
}


fn schedule<T: pallet_contracts::Config>() -> pallet_contracts::Schedule<T> {
	pallet_contracts::Schedule {
		limits: pallet_contracts::Limits {
			runtime_memory: 1024 * 1024 * 1024,
			..Default::default()
		},
		..Default::default()
	}
}

parameter_types! {
	pub const DepositPerItem: Balance = 0;
	pub const DepositPerByte: Balance = 0;
	pub Schedule: pallet_contracts::Schedule<Test> = schedule::<Test>();
	pub const DefaultDepositLimit: Balance = 0;
}

// Pallet contracts promises to never use this, but needs this type anyway
// Therefore we provide it, but panic when called.
pub struct FakeRandom;
impl<Output, BlockNumber> frame_support::traits::Randomness<Output, BlockNumber> for FakeRandom {
	fn random(_: &[u8]) -> (Output, BlockNumber) {
		panic!("Pallet contracts promised not to call me");
	}

	fn random_seed() -> (Output, BlockNumber) {
		panic!("Pallet contracts promised not to call me");
	}
}

impl pallet_contracts::Config for Test {
	type Time = Timestamp;
	type Randomness = FakeRandom;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	/// The safest default is to allow no calls at all.
	///
	/// Runtimes should whitelist dispatchables that are allowed to be called from contracts
	/// and make sure they are stable. Dispatchables exposed to contracts are not allowed to
	/// change because that would break already deployed contracts. The `RuntimeCall` structure
	/// itself is not allowed to change the indices of existing pallets, too.
	type CallFilter = frame_support::traits::Nothing;
	type DepositPerItem = DepositPerItem;
	type DepositPerByte = DepositPerByte;
	type CallStack = [pallet_contracts::Frame<Self>; 31];
	type WeightPrice = (); //pallet_transaction_payment::Pallet<Self>;
	type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
	type ChainExtension = ();
	type Schedule = Schedule;
	type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
	type MaxCodeLen = ConstU32<{ 128 * 1024 }>;
	type DefaultDepositLimit = DefaultDepositLimit;
	type MaxStorageKeyLen = ConstU32<128>;
	type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
	type UnsafeUnstableInterface = ConstBool<false>;
}

parameter_types! {
	// we're not really using this, as reservation is via DAO, but whatever
	pub const ApprovalDeposit: Balance = 1;
	pub const AssetsStringLimit: u32 = 50;
}

impl pallet_dao_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<Self::AccountId>>;
	type ApprovalDeposit = ApprovalDeposit;
	type RemoveItemsLimit = ConstU32<1000>;
	type StringLimit = AssetsStringLimit;
	type HistoryHorizon = ConstU32<4200>;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

impl pallet_dao_core::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MinLength = ConstU32<3>;
	type MaxLengthId = ConstU32<8>;
	type MaxLengthName = ConstU32<16>;
	type MaxLengthMetadata = ConstU32<256>;
	type Currency = Balances;
	type DaoDeposit = ConstU128<10>;
	type TokenUnits = ConstU8<10>;
	type AssetId = u32;
	type WeightInfo = ();
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<1000>;
	type WeightInfo = ();
}

parameter_types! {
	pub const DepositPerItem: Balance = deposit(1, 0);
	pub const DepositPerByte: Balance = deposit(0, 1);
	pub const DefaultDepositLimit: Balance = deposit(1024, 1024 * 1024);
}

pub struct FakeRandom;
impl<Output, BlockNumber> Randomness<Output, BlockNumber> for FakeRandom {
	fn random(_: &[u8]) -> (Output, BlockNumber) {
		panic!("Pallet contracts promised not to call me");
	}

	fn random_seed() -> (Output, BlockNumber) {
		panic!("Pallet contracts promised not to call me");
	}
}

impl pallet_contracts::Config for Test {
	type Time = Timestamp;
	type Randomness = FakeRandom;
	type Currency = Balances;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	/// The safest default is to allow no calls at all.
	///
	/// Runtimes should whitelist dispatchables that are allowed to be called from contracts
	/// and make sure they are stable. Dispatchables exposed to contracts are not allowed to
	/// change because that would break already deployed contracts. The `RuntimeCall` structure
	/// itself is not allowed to change the indices of existing pallets, too.
	type CallFilter = frame_support::traits::Nothing;
	type DepositPerItem = DepositPerItem;
	type DepositPerByte = DepositPerByte;
	type CallStack = [pallet_contracts::Frame<Self>; 31];
	type WeightPrice = ();
	type WeightInfo = pallet_contracts::weights::SubstrateWeight<Self>;
	type ChainExtension = ();
	type Schedule = ();
	type AddressGenerator = pallet_contracts::DefaultAddressGenerator;
	type MaxCodeLen = ConstU32<{ 128 * 1024 }>;
	type DefaultDepositLimit = DefaultDepositLimit;
	type MaxStorageKeyLen = ConstU32<128>;
	type MaxDebugBufferLen = ConstU32<{ 2 * 1024 * 1024 }>;
	type UnsafeUnstableInterface = ConstBool<false>;
	type Migrations = ();
}

// Build genesis storage according to the mock runtime.
pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	pallet_balances::GenesisConfig::<Test> { balances: vec![(1, 1000)] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
