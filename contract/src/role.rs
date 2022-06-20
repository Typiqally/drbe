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
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // part of writing unit tests is setting up a mock context
    // in this example, this is only needed for env::log in the contract
    // this is also a useful list to peek at when wondering what's available in env::*
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    #[test]
    fn given_role_when_get_name_then_return_name() {
        // Arrange
        let context = get_context(vec![], false);
        testing_env!(context);

        let contract = Role {
            name: "test".to_string(),
            ..Default::default()
        };

        // Act
        let name = contract.get_name();

        // Assert
        assert_eq!("test".to_string(), name);
    }

    #[test]
    fn given_role_when_get_public_key_then_return_public_key() {
        // Arrange
        let context = get_context(vec![], false);
        testing_env!(context);

        let contract = Role {
            public_key: "test".to_string(),
            ..Default::default()
        };

        // Act
        let public_key = contract.get_public_key();

        // Assert
        assert_eq!("test".to_string(), public_key);
    }

    #[test]
    fn given_account_when_get_encrypted_private_key_then_return_encrypted_private_key() {
        // Arrange
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut map = LookupMap::<String, String>::new(b"r".to_vec());
        map.insert(&"francis.near".to_string(), &"test".to_string());

        let contract = Role {
            encrypted_private_keys: map,
            ..Default::default()
        };

        // Act
        let encrypted_private_key = contract
            .get_encrypted_private_key("francis.near".to_string())
            .unwrap();

        // Assert
        assert_eq!("test".to_string(), encrypted_private_key);
    }

    #[test]
    fn given_no_account_when_get_encrypted_private_key_then_return_none() {
        // Arrange
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Role::default();

        // Act
        let encrypted_private_key = contract.get_encrypted_private_key("francis.near".to_string());

        // Assert
        assert_eq!(None, encrypted_private_key);
    }
}
