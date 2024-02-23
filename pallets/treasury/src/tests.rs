use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(Treasury::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(Treasury::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(Treasury::cause_error(RuntimeOrigin::signed(1)), Error::<Test>::NoneValue);
	});
}

type Assets = <Test as crate::Config>::Fungibles;

#[test]
fn mint_asset_works() {
	new_test_ext().execute_with(|| {
		let alice = 0;
		let asset_id = 1337;
		let amount = 420;

		assert_ok!(Treasury::gimme_asset(RuntimeOrigin::signed(alice), asset_id, amount));
		assert_eq!(Assets::balance(asset_id, alice), amount);
	});
}

type NativeBalance = <Test as crate::Config>::NativeBalance;
use frame_support::traits::tokens::WithdrawConsequence;
use sp_runtime::TokenError;

#[test]
fn freeze_and_hold_works() {
	new_test_ext().execute_with(|| {
		let alice = 0;

		use frame_support::traits::fungible::*;

		assert_eq!(NativeBalance::total_issuance(), 0);
		assert_ok!(NativeBalance::mint_into(&alice, 100));

		assert_eq!(NativeBalance::total_issuance(), 100);
		assert_eq!(NativeBalance::free_balance(&alice), 100);

		assert_ok!(NativeBalance::set_freeze(&(), &alice, 50));
		assert_eq!(NativeBalance::can_withdraw(&alice, 50), WithdrawConsequence::Success);
		assert_eq!(NativeBalance::can_withdraw(&alice, 60), WithdrawConsequence::Frozen);
		assert_ok!(NativeBalance::set_freeze(&(), &alice, 50000));
		assert_eq!(NativeBalance::can_withdraw(&alice, 1), WithdrawConsequence::Frozen);

		assert_ok!(NativeBalance::hold(&(), &alice, 50));
		assert_noop!(NativeBalance::hold(&(), &alice, 60), TokenError::FundsUnavailable);

		assert_ok!(NativeBalance::thaw(&(), &alice));

		assert_eq!(NativeBalance::can_withdraw(&alice, 1), WithdrawConsequence::Success);
		assert_eq!(NativeBalance::can_withdraw(&alice, 60), WithdrawConsequence::BalanceLow);
	});
}
