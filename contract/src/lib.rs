use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Role {
    name: String,
    owner_account_id: String,
    public_key: String,
    encrypted_private_keys: LookupMap<String, String>,
}

impl Default for Role {
    fn default() -> Self {
        Self {
            name: String::new(),
            owner_account_id: String::new(),
            public_key: String::new(),
            encrypted_private_keys: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {}
