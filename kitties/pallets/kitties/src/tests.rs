use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, assert_err, traits::{OnInitialize}};
// use frame_system as System;
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

#[test]
fn create_kitty_should_work() {
    new_test_ext().execute_with(|| {
        setup_blocks(1);
        let user_account: u64 = 1;
        let owner = Origin::signed(user_account);
        assert_ok!(SubstrateKitties::create_kitty(owner));
        assert_eq!(SubstrateKitties::kitty_cnt(), 1);
    })
}

#[test]
fn create_kitty_failed_not_enough_money() {
    new_test_ext().execute_with(|| {
        setup_blocks(2);
        let user_account: u64 = 2;
        let owner = Origin::signed(user_account);
        assert_err!(
            SubstrateKitties::create_kitty(owner.clone()),
            <Error<Test>>::NotEnoughBalance
        );
    })
}

#[test]
fn transfer_should_work() {
    new_test_ext().execute_with(|| {
        setup_blocks(3);
        let user_account: u64 = 1;
        let owner = Origin::signed(user_account);
        let kitty_id = SubstrateKitties::mint(&user_account, None, None).unwrap();
        assert_ok!(SubstrateKitties::create_kitty(owner.clone()));
        assert_ok!(SubstrateKitties::transfer(owner.clone(), kitty_id, 2));
    })
}

#[test]
fn transfer_failed_not_kitty_owner() {
    new_test_ext().execute_with(|| {
        setup_blocks(1);
        let user_account: u64 = 1;
        let not_owner = Origin::signed(2);
        let kitty_id = SubstrateKitties::mint(&user_account, None, None).unwrap();
        assert_err!(
            SubstrateKitties::transfer(not_owner, kitty_id, 2),
            Error::<Test>::NotKittyOwner
        );
    })
}

#[test]
fn buy_kitty_should_work() {
    new_test_ext().execute_with(|| {
        setup_blocks(5);
        let user_account: u64 = 1;
        let owner = Origin::signed(user_account);
        let not_owner = Origin::signed(2);
        let kitty_id = SubstrateKitties::mint(&user_account, None, None).unwrap();
        assert_ok!(SubstrateKitties::create_kitty(owner.clone()));
        assert_ok!(SubstrateKitties::sell_kitty(owner.clone(), kitty_id, Some(10)));
        assert_ok!(SubstrateKitties::buy_kitty(not_owner, kitty_id, 10));
    });
}

#[test]
fn buy_kitty_failed_NotEnoughBalance() {
    new_test_ext().execute_with(|| {
        setup_blocks(5);
        let user_account: u64 = 1;
        let owner = Origin::signed(user_account);
        let not_owner = Origin::signed(2);
        let kitty_id = SubstrateKitties::mint(&user_account, None, None).unwrap();
        assert_ok!(SubstrateKitties::create_kitty(owner.clone()));
        assert_ok!(SubstrateKitties::sell_kitty(owner.clone(), kitty_id, Some(2000)));
        assert_err!(SubstrateKitties::buy_kitty(not_owner, kitty_id, 2000), Error::<Test>::NotEnoughBalance);
    });
}

#[test]
fn buy_kitty_failed_BuyerIsKittyOwner() {
    new_test_ext().execute_with(|| {
        setup_blocks(5);
        let user_account: u64 = 1;
        let owner = Origin::signed(user_account);
        let kitty_id = SubstrateKitties::mint(&user_account, None, None).unwrap();
        assert_ok!(SubstrateKitties::create_kitty(owner.clone()));
        assert_ok!(SubstrateKitties::sell_kitty(owner.clone(), kitty_id, Some(2000)));
        assert_err!(SubstrateKitties::buy_kitty(owner.clone(), kitty_id, 2000), Error::<Test>::BuyerIsKittyOwner);
    });
}

#[test]
fn sell_kitty_failed_not_kitty_owner() {
    new_test_ext().execute_with(|| {
        setup_blocks(7);
        let user_account: u64 = 1;
        let owner = Origin::signed(user_account);
        let not_owner = Origin::signed(2);
        let kitty_id = SubstrateKitties::mint(&user_account, None, None).unwrap();
        assert_ok!(SubstrateKitties::create_kitty(owner.clone()));
        assert_err!(
            SubstrateKitties::sell_kitty(not_owner, kitty_id, Some(2000)),
            Error::<Test>::NotKittyOwner
        );
    });
}

#[test]
fn sell_kitty_should_work() {
    new_test_ext().execute_with(|| {
        setup_blocks(7);
        let user_account: u64 = 1;
        let owner = Origin::signed(user_account);
        let kitty_id = SubstrateKitties::mint(&user_account, None, None).unwrap();
        assert_ok!(SubstrateKitties::create_kitty(owner.clone()));
        assert_ok!(SubstrateKitties::sell_kitty(owner.clone(), kitty_id, Some(2000)));
    });
}



