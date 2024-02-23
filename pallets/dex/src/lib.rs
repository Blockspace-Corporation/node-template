#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::fungibles;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::traits::fungible;

pub type AssetIdOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
	<T as frame_system::Config>::AccountId,
>>::AssetId;

pub type BalanceOf<T> = <<T as Config>::NativeBalance as fungible::Inspect<
	<T as frame_system::Config>::AccountId,
>>::Balance;

pub type AssetBalanceOf<T> = <<T as Config>::Fungibles as fungibles::Inspect<
	<T as frame_system::Config>::AccountId,
>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use crate::*;
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungible::{self, *},
			fungibles::{self},
		},
	};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Type to access the Balances Pallet.
		type NativeBalance: fungible::Inspect<Self::AccountId>
			+ fungible::Mutate<Self::AccountId>
			+ fungible::hold::Inspect<Self::AccountId>
			+ fungible::hold::Mutate<Self::AccountId>
			+ fungible::freeze::Inspect<Self::AccountId>
			+ fungible::freeze::Mutate<Self::AccountId>;

		/// Type to access the Assets Pallet.
		type Fungibles: fungibles::Inspect<Self::AccountId, Balance = BalanceOf<Self>, AssetId = u32>
			+ fungibles::Mutate<Self::AccountId>
			+ fungibles::Create<Self::AccountId>;
	}

	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct AssetPair<T: Config> {
		pub asset_a: AssetIdOf<T>,
		pub asset_b: AssetIdOf<T>,
	}

	#[pallet::storage]
	pub type Pools<T> = StorageMap<_, Blake2_128Concat, AssetPair<T>, bool>;

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u32, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::default())]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored { something, who });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(Weight::default())]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(2)]
		#[pallet::weight(Weight::default())]
		pub fn gimme_money(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			T::NativeBalance::mint_into(&who, 6969420u32.into())?;

			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(3)]
		#[pallet::weight(Weight::default())]
		pub fn gimme_specific_money(origin: OriginFor<T>, amount: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			T::NativeBalance::mint_into(&who, amount)?;

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(Weight::default())]
		pub fn give_them_specific_money(
			origin: OriginFor<T>,
			who: T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			T::NativeBalance::mint_into(&who, amount)?;

			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(Weight::default())]
		pub fn create_pool(
			origin: OriginFor<T>,
			asset_a: u32,
			asset_b: AssetIdOf<T>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			let pair = AssetPair { asset_a, asset_b };

			Pools::<T>::insert(pair, true);

			Ok(())
		}

		#[pallet::call_index(6)]
		#[pallet::weight(Weight::default())]
		pub fn check_equal(
			origin: OriginFor<T>,
			native: BalanceOf<T>,
			asset: AssetBalanceOf<T>,
		) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			ensure!(native == asset, "they are not equal");
			Ok(())
		}
	}
}