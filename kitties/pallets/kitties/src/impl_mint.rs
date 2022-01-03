use super::*;
use codec::Encode;
use frame_support::sp_runtime::traits::Hash;
use sp_io::hashing::blake2_128;
use frame_support::{dispatch::DispatchResult, traits::{Currency, ReservableCurrency, ExistenceRequirement, Randomness}};

impl<T: Config> Pallet<T> {
    fn gen_gender() -> Gender {
        let random = T::KittyRandomness::random(&b"gender"[..]).0;
        match random.as_ref()[0] % 2 {
            0 => Gender::Male,
            _ => Gender::Female,
        }
    }
    fn gen_dna() -> [u8; 16] {
        let payload = (
            T::KittyRandomness::random(&b"dna"[..]).0,
            <frame_system::Pallet<T>>::block_number(),
        );
        payload.using_encoded(blake2_128)
    }
    pub fn mint(owner: &T::AccountId, dna: Option<[u8; 16]>, gender: Option<Gender>) -> Result<T::Hash, Error<T>> {
        let kitty = Kitty::<T> {
            dna: dna.unwrap_or_else(Self::gen_dna),
            price: None,
            gender: gender.unwrap_or_else(Self::gen_gender),
            owner: owner.clone(),
        };
        let kitty_id = T::Hashing::hash_of(&kitty);
        let new_cnt = Self::kitty_cnt().checked_add(1).ok_or(<Error<T>>::KittyCntOverflow)?;

        <KittiesOwned<T>>::try_mutate(&owner, |kitty_vec| {
            kitty_vec.try_push(kitty_id)
        }).map_err(|_| <Error<T>>::ExceedMaxKittyOwned)?;

        <Kitties<T>>::insert(kitty_id, kitty);
        <KittyCnt<T>>::put(new_cnt);
        Ok(kitty_id)
    }
    pub fn breed_dna(parent1: &T::Hash, parent2: &T::Hash) -> Result<[u8; 16], Error<T>> {
        let dna1 = Self::kitties(parent1).ok_or(<Error<T>>::KittyNotExist)?.dna;
        let dna2 = Self::kitties(parent2).ok_or(<Error<T>>::KittyNotExist)?.dna;
  
        let mut new_dna = Self::gen_dna();
        for i in 0..new_dna.len() {
          new_dna[i] = (new_dna[i] & dna1[i]) | (!new_dna[i] & dna2[i]);
        }
        Ok(new_dna)
    }
}