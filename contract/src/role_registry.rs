use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::collections::Vector;
use near_sdk::{env, near_bindgen};

use crate::role::Role;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RoleRegistry {
    pub roles: Vector<Role>,
}

impl Default for RoleRegistry {
    fn default() -> Self {
        Self {
            roles: Vector::new(b"r".to_vec()),
        }
    }
}

#[near_bindgen]
impl RoleRegistry {
    pub fn get_roles(&self) -> Vec<Role> {
        return self.roles.to_vec();
    }

    pub fn create_role(&mut self, name: String, public_key: String) {
        let account_id = env::signer_account_id();

        let role = Role {
            name,
            owner_account_id: account_id.clone(),
            public_key,
            encrypted_private_keys: LookupMap::new(b"r".to_vec()),
        };

        self.roles.push(&role);
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
    fn given_existing_role_when_get_roles_then_return_roles() {
        // Arrange
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = RoleRegistry::default();
        contract.create_role("test_name".to_string(), "test_public_key".to_string());

        // Act
        let roles = contract.get_roles();

        // Assert
        let role = roles.get(0).unwrap();
        assert_eq!("test_name".to_string(), role.name);
        assert_eq!("test_public_key".to_string(), role.public_key);
    }

    #[test]
    fn given_default_registry_when_create_role_then_role_is_created() {
        // Arrange
        let context = get_context(vec![], false);
        testing_env!(context);

        let mut contract = RoleRegistry::default();

        // Act
        contract.create_role("test_name".to_string(), "test_public_key".to_string());

        // Assert
        let roles = contract.get_roles();
        let role = roles.get(0).unwrap();
        assert_eq!("test_name".to_string(), role.name);
        assert_eq!("test_public_key".to_string(), role.public_key);
    }
}
