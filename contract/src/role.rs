use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen};

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

#[near_bindgen]
impl Role {
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_public_key(&self) -> String {
        return self.public_key.clone();
    }

    pub fn get_encrypted_private_key(&self, account_id: String) -> Option<String> {
        return self.encrypted_private_keys.get(&account_id);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {}
