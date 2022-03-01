mod model;
mod utils;
use crate::{
    utils::{
        AccountId,
        ONE_NEAR,
        assert_self,
        assert_single_promise_success,
    },
    model::{
        Crowdfund,
        Donation
    }
};
use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}, Promise};
#[allow(unused_imports)]
use near_sdk::{env, PromiseIndex, near_bindgen};
near_sdk::setup_alloc!();   

#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    owner: AccountId,
    crowdfunds: Vec<Crowdfund>,
    donations: Vec<Donation>,
}

#[near_bindgen]

impl Contract {
    #[init]
    pub fn init(owner:AccountId)->Self{
        let crowdfunds: Vec<Crowdfund> = Vec::new();
        let donations: Vec<Donation> = Vec::new();
        Contract {
            owner,
            crowdfunds,
            donations
        }
    }
}
