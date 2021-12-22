use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

use super::*;


#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim), 
			Some((1, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofsAlreadyClaimed
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::revoke(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), None);
	})
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_noop!(
			PoeModule::revoke(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotFound
		);
	})
}

#[test]
fn transform_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_ok!(PoeModule::transform(Origin::signed(1), claim.clone(), 2));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((2, frame_system::Pallet::<Test>::block_number()))
		);
	})
}

#[test]
fn transform_failed_when_claim_not_found() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_noop!(
			PoeModule::transform(Origin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotFound
		);
	})
}

#[test]
fn transform_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
		assert_noop!(
			PoeModule::transform(Origin::signed(2), claim.clone(), 3),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn create_claim_failed_when_claim_over_max_length() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1, 2, 3];
		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::OverMaxLength
		);
	})
}

