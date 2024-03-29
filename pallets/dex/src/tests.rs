use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Dispatch a signed extrinsic.
		assert_ok!(Dex::do_something(RuntimeOrigin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		assert_eq!(Dex::something(), Some(42));
		// Assert that the correct event was deposited
		System::assert_last_event(Event::SomethingStored { something: 42, who: 1 }.into());
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(Dex::cause_error(RuntimeOrigin::signed(1)), Error::<Test>::NoneValue);
	});
}

type NativeBalance = <Test as crate::Config>::NativeBalance;

#[test]
fn basic_balance_stuff() {
	new_test_ext().execute_with(|| {
		let alice = 0;
		let bob = 1;

		use frame_support::traits::fungible::*;

		assert_eq!(NativeBalance::total_issuance(), 0);
		assert_ok!(NativeBalance::mint_into(&alice, 100));

		assert_eq!(NativeBalance::total_issuance(), 100);
		assert_eq!(NativeBalance::free_balance(&alice), 100);
		assert_eq!(NativeBalance::total_balance(&bob), 0);
	});
}
