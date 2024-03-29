use std::borrow::Borrow;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{Vector, LookupMap};
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Role {
    pub name: String,
    pub owner_account_id: String,
    pub public_key: Vector<u8>,
    pub encrypted_private_keys: LookupMap<String, Vector<u8>>,
}

impl Default for Role {
    fn default() -> Self {
        Self {
            name: String::new(),
            owner_account_id: String::new(),
            public_key: Vector::new(b"p".to_vec()),
            encrypted_private_keys: LookupMap::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl Role {
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_owner_account_id(&self) -> String {
        return self.owner_account_id.clone();
    }

    pub fn get_public_key(&self) -> Vec<u8> {
        return self.public_key.to_vec();
    }

    pub fn get_encrypted_private_key(&self) -> Option<Vec<u8>> {
        let account_id = env::current_account_id();

        return self.encrypted_private_keys.get(&account_id).map(|v| v.to_vec());
    }

    pub fn add_encrypted_private_key(
        &mut self,
        account_id: String,
        encrypted_private_key: Vec<u8>,
    ) {
        let mut vector = Vector::<u8>::new(b"p");

        for byte in encrypted_private_key {
            vector.push(&byte);
        }

        self.encrypted_private_keys.insert(&account_id, &vector);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

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
    fn given_role_when_get_owner_account_id_then_return_owner_account_id() {
        // Arrange
        let context = get_context(vec![], false);
        testing_env!(context);

        let contract = Role {
            owner_account_id: "test".to_string(),
            ..Default::default()
        };

        // Act
        let name = contract.get_owner_account_id();

        // Assert
        assert_eq!("test".to_string(), name);
    }

    #[test]
    fn given_role_when_get_public_key_then_return_public_key() {
        // Arrange
        let context = get_context(vec![], false);
        testing_env!(context);

        let mock_key = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut mock_vector = Vector::<u8>::new(b"p");

        for byte in mock_key.clone() {
            mock_vector.push(&byte);
        }

        let contract = Role {
            public_key: mock_vector,
            ..Default::default()
        };

        // Act
        let public_key = contract.get_public_key();

        // Assert
        assert_eq!(mock_key.clone(), public_key);
    }

    #[test]
    fn given_account_when_get_encrypted_private_key_then_return_encrypted_private_key() {
        // Arrange
        let context = get_context(vec![], false);
        testing_env!(context);

        let mock_key = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut mock_vector = Vector::<u8>::new(b"p");

        for byte in mock_key.clone() {
            mock_vector.push(&byte);
        }

        let mut map = LookupMap::<String, Vector<u8>>::new(b"r".to_vec());
        map.insert(&"alice.testnet".to_string(), &mock_vector);

        let contract = Role {
            encrypted_private_keys: map,
            ..Default::default()
        };

        // Act
        let encrypted_private_key = contract.get_encrypted_private_key().unwrap();

        // Assert
        assert_eq!(mock_key.clone(), encrypted_private_key);
    }

    #[test]
    fn given_no_account_when_get_encrypted_private_key_then_return_none() {
        // Arrange
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = Role::default();

        // Act
        let encrypted_private_key = contract.get_encrypted_private_key();

        // Assert
        assert_eq!(None, encrypted_private_key);
    }

    #[test]
    fn given_signer_when_add_encrypted_private_key_for_signer_then_encrypted_private_key_is_added()
    {
        // Arrange
        let context = get_context(vec![], false);
        testing_env!(context);

        let mock_private_key = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut contract = Role::default();

        // Act
        contract.add_encrypted_private_key("alice.testnet".to_string(), mock_private_key.clone());

        // Assert
        let encrypted_private_key = contract.get_encrypted_private_key().unwrap();
        assert_eq!(mock_private_key, encrypted_private_key);
    }
}
