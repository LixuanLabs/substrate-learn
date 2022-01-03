use crate as pallet_kitties;
use frame_support::{parameter_types, traits::{OnInitialize}};
use frame_system as system;
use sp_runtime::{
	generic,
	traits::{BlakeTwo256, IdentityLookup},
};
use core::convert::TryFrom;
// use pallet_balances::*;
// use pallet_randomness_collective_flip;
use sp_runtime::{
    traits::{Header},
};


fn setup_blocks(blocks: u64) {
    let mut parent_hash = System::parent_hash();

    for i in 1..(blocks + 1) {
        System::initialize(&i, &parent_hash, &Default::default(), frame_system::InitKind::Full);
        RandomnessCollectiveFlip::on_initialize(i);

        let header = System::finalize();
        parent_hash = header.hash();
        System::set_block_number(*header.number());
    }
}

/// An index to a block.
pub type BlockNumber = u64;
pub type Index = u64;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u64;
pub type Hash = sp_core::H256;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		SubstrateKitties: pallet_kitties::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
	}
);

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

parameter_types! {              // <- add this macro
    // One can own at most 9,999 Kitties
    pub const MaxKittyOwned: u32 = 9999;
	pub const PledgeCount: u64 = 2000;
	pub const ExistentialDeposit: u64 = 50;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

impl pallet_randomness_collective_flip::Config for Test {}

impl pallet_kitties::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type KittyRandomness = RandomnessCollectiveFlip;
	type MaxKittyOwned = MaxKittyOwned;
	type PledgeCount = PledgeCount;

}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	// system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(1, 2000),
			(2, 1000)
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();
	// setup_blocks(1);
	pallet_kitties::GenesisConfig::<Test> {
		kitties: vec![]
	};
	t.into()
}
