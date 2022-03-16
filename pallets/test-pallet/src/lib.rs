#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
    use frame_system::pallet_prelude::{ OriginFor};
	use sp_std::{prelude::*, vec::Vec};

	use frame_system::{ensure_signed};
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_did::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

	#[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

	#[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// An organization has been created. [creator, organization_id]
        CreatedAttribute(T::AccountId, Vec<u8>),
    }

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Increase the value associated with a particular key
		#[pallet::weight(0)]
        pub fn create_test(
            origin: OriginFor<T>,
            name: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;
            Self::create_did(&who, name.clone())?;
            Self::deposit_event(Event::CreatedAttribute(who, name));
            Ok(().into())
        }
	}

	#[allow(dead_code)]
    impl<T: Config> Pallet<T> {
        pub fn create_did(owner: &T::AccountId, name: Vec<u8>) -> DispatchResult {
            // let mut orgs = <Pallet<T>>::organizations();
            // ensure!(!orgs.contains(&owner), Error::<T>::OrganizationExists);
            // orgs.push(owner.clone());
            // <Organizations<T>>::put(orgs);

            // DID add attribute
            <pallet_did::Pallet<T>>::create_attribute(&owner, &owner, b"Org", &name, None)?;
            Ok(())
        }
	}
}

