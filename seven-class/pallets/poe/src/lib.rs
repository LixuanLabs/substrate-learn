#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use sp_std::vec::Vec;

    #[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        #[pallet::constant]
        type ClaimMaxLimit: Get<u64>;
	}

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T> (_);

    #[pallet::storage]
    #[pallet::getter(fn proofs)]
    pub type Proofs<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, (T::AccountId, T::BlockNumber)>; // 存储拥有人和当前块

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)] // 辅助函数，触发函数
    // #[pallet::metadata(T::AccountId = "AccountId")]
    pub enum Event<T: Config>  {
        ClaimCreated(T::AccountId, Vec<u8>),
        ClaimRevoke(T::AccountId, Vec<u8>),
        Transform(T::AccountId, Vec<u8>, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        ProofsAlreadyClaimed,
        ClaimNotFound,
        NotClaimOwner,
        OverMaxLength,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(0)]
        pub fn create_claim(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            ensure!(claim.len() < T::ClaimMaxLimit::get().try_into().unwrap(), Error::<T>::OverMaxLength);

            ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofsAlreadyClaimed);

            Proofs::<T>::insert(&claim, (sender.clone(), frame_system::Pallet::<T>::block_number()));

            Self::deposit_event(Event::ClaimCreated(sender, claim));

            Ok(().into())
        }

        #[pallet::weight(0)]
        pub fn revoke(origin: OriginFor<T>, claim: Vec<u8>) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotFound)?;

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            Proofs::<T>::remove(&claim);

            Self::deposit_event(Event::ClaimRevoke(sender, claim));

            Ok(().into())

        }

        #[pallet::weight(0)]
        pub fn transform(origin: OriginFor<T>, claim: Vec<u8>, acceptor: T::AccountId) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotFound)?;

            ensure!(owner == sender, Error::<T>::NotClaimOwner);

            // Proofs::<T>::mutate(&claim, |v| { 
            //     *v = Some((acceptor.clone(), frame_system::Pallet::<T>::block_number()));
            // });

            Proofs::<T>::insert(&claim, (acceptor, frame_system::Pallet::<T>::block_number()));

            Ok(().into())
        }
    }
}