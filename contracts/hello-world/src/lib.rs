#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol};

#[contracttype]
#[derive(Clone)]
pub struct Rental {
    pub owner: Address,
    pub renter: Option<Address>,
    pub price_per_day: i128,
    pub rented_until: u64,
}

#[contract]
pub struct NFTRentalContract;

#[contractimpl]
impl NFTRentalContract {

    // Store NFT rental info
    pub fn list_nft(
        env: Env,
        nft_id: Symbol,
        owner: Address,
        price_per_day: i128,
    ) {
        owner.require_auth();

        let rental = Rental {
            owner: owner.clone(),
            renter: None,
            price_per_day,
            rented_until: 0,
        };

        env.storage().instance().set(&nft_id, &rental);
    }

    // Rent NFT
    pub fn rent_nft(
        env: Env,
        nft_id: Symbol,
        renter: Address,
        duration_days: u64,
    ) {
        renter.require_auth();

        let mut rental: Rental = env
            .storage()
            .instance()
            .get(&nft_id)
            .expect("NFT not listed");

        // Ensure not already rented
        if rental.rented_until > env.ledger().timestamp() {
            panic!("Already rented");
        }

        rental.renter = Some(renter);
        rental.rented_until =
            env.ledger().timestamp() + (duration_days * 86400);

        env.storage().instance().set(&nft_id, &rental);
    }

    // Get rental details
    pub fn get_rental(env: Env, nft_id: Symbol) -> Rental {
        env.storage()
            .instance()
            .get(&nft_id)
            .expect("NFT not found")
    }
}