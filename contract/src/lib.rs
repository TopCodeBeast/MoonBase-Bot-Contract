
use std::collections::HashMap;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, Base58CryptoHash, Base58PublicKey};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::serde_json::{json, self};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, PanicOnDefault, bs58};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use upgrade::Upgrade;
use utils::{refund_extra_storage_deposit, get_hash};
use utils::verify;
//use wasm_sign::{Config, PKey};

setup_alloc!();
pub mod internal;
pub mod utils;
pub mod view;
pub mod upgrade;
pub mod sha512;
pub mod curve25519;
pub mod ed25519;
pub mod error;
pub mod owner;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct DiscordRoles {
    owner_id: AccountId,
    public_key: String,
    roles: LookupMap<String, Role>,
    guilds: LookupMap<String, UnorderedMap<String, bool>>,
    key_fields: UnorderedMap<(String, String), UnorderedMap<String, bool>>,
    upgrade: Upgrade
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct Role {
    guild_id: String,
    role_id: String,
    fields: HashMap<String, String>,
    key_field: (String, String),
}




#[near_bindgen]
impl DiscordRoles {
    #[init]
    pub fn new(pk: String) -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            public_key: pk,
            roles: LookupMap::new(b"r".to_vec()),
            guilds: LookupMap::new(b"g".to_vec()),
            key_fields: UnorderedMap::new(b"k".to_vec()),
            upgrade: Upgrade::new(env::predecessor_account_id(), 0)
        }
      }

    #[payable]
    pub fn set_roles(&mut self, args: String, sign: String) {
        let sign: Vec<u8> = bs58::decode(sign).into_vec().unwrap();
        let pk: Vec<u8> = bs58::decode(self.public_key.clone()).into_vec().unwrap();
        verify(args.as_bytes().to_vec(), sign.into(), pk.into());
        
        let roles: Vec<Role> = serde_json::from_str(&args).unwrap();
        let initial_storage_usage = env::storage_usage();
        for role in roles.iter() {
            self.internal_set_role(role.guild_id.clone(), role.role_id.clone(), role.fields.clone(), role.key_field.clone());
        }
        refund_extra_storage_deposit(
            env::storage_usage() - initial_storage_usage,
            0,
        );
    }

    pub fn del_role(&mut self, args: String, sign: String) {
        let sign: Vec<u8> = bs58::decode(sign).into_vec().unwrap();
        let pk: Vec<u8> = bs58::decode(self.public_key.clone()).into_vec().unwrap();
        let role_ids: Vec<Role> = serde_json::from_str(&args).unwrap();
        verify(args.as_bytes().to_vec(), sign.into(), pk.into());

        for role in role_ids {
            let hash = get_hash(role.guild_id, role.role_id, role.fields, role.key_field);
            let role = self.roles.get(&hash).unwrap();

            let mut guild = self.guilds.get(&role.guild_id).unwrap();
            guild.remove(&hash);
            self.guilds.insert(&role.guild_id, &guild);

            let mut key_field = self.key_fields.get(&role.key_field).unwrap();
            key_field.remove(&hash);
            self.key_fields.insert(&role.key_field, &key_field);

            self.roles.remove(&hash);
        }
        
    }

}