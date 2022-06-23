use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::collections::Vector;
use near_sdk::serde::{Serialize, Serializer};
use near_sdk::{env, near_bindgen};
use near_sdk::serde::ser::SerializeStruct;

use crate::role::Role;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct RoleRegistry {
    pub roles: Vector<Role>,
}

impl Serialize for Role {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut state = serializer.serialize_struct("Role", 4)?;
        state.serialize_field("name", &self.name);
        state.serialize_field("owner_account_id", &self.owner_account_id);
        state.serialize_field("public_key", &self.public_key.to_vec());
        state.end()
    }
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

    pub fn create_role(&mut self, name: String, public_key: Vec<u8>) {
        let account_id = env::current_account_id();
        let mut role = Role {
            name,
            owner_account_id: account_id.clone(),
            public_key: Vector::new(b"p".to_vec()),
            encrypted_private_keys: LookupMap::new(b"r".to_vec()),
        };

        for byte in public_key {
            role.public_key.push(&byte);
        }

        self.roles.push(&role);
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
    fn given_existing_role_when_get_roles_then_return_roles() {
        // Arrange
        let context = get_context(vec![], false);
        testing_env!(context);

        let mock_public_key = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut contract = RoleRegistry::default();
        contract.create_role("test_name".to_string(), mock_public_key.clone());

        // Act
        let roles = contract.get_roles();

        // Assert
        let role = roles.get(0).unwrap();
        assert_eq!("test_name".to_string(), role.name);
        assert_eq!(mock_public_key.clone(), role.public_key.to_vec());
    }

    #[test]
    fn given_no_existing_roles_when_get_roles_then_return_empty_vec() {
        // Arrange
        let context = get_context(vec![], true);
        testing_env!(context);

        let contract = RoleRegistry::default();

        // Act
        let roles = contract.get_roles();

        // Assert
        assert_eq!(0, roles.len());
    }

    #[test]
    fn given_default_registry_when_create_role_then_role_is_created() {
        // Arrange
        let context = get_context(vec![], false);
        testing_env!(context);

        let mock_public_key = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut contract = RoleRegistry::default();

        // Act
        contract.create_role("test_name".to_string(), mock_public_key.clone());

        // Assert
        let roles = contract.get_roles();
        let role = roles.get(0).unwrap();
        assert_eq!("test_name".to_string(), role.name);
        assert_eq!(mock_public_key.clone(), role.public_key.to_vec());
    }
}
