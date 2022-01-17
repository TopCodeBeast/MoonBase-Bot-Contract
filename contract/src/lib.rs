
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128, Base58CryptoHash, Base58PublicKey};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::serde_json::{json, self};
use near_sdk::{env, near_bindgen, setup_alloc, AccountId, PanicOnDefault, bs58};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use upgrade::Upgrade;
use utils::refund_extra_storage_deposit;
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
    tokens: UnorderedMap<AccountId, UnorderedMap<String, bool>>,
    upgrade: Upgrade
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Debug)]
pub struct Role {
    guild_id: String,
    role_id: String,
    token_id: AccountId,
    amount: U128,
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
            tokens: UnorderedMap::new(b"t".to_vec()),
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
            self.internal_set_role(role.guild_id.clone(), role.role_id.clone(), role.token_id.clone(), role.amount);
        }
        refund_extra_storage_deposit(
            env::storage_usage() - initial_storage_usage,
            0,
        );
    }

    pub fn del_role(&mut self, args: String, sign: String) {
        let sign: Vec<u8> = bs58::decode(sign).into_vec().unwrap();
        let pk: Vec<u8> = bs58::decode(self.public_key.clone()).into_vec().unwrap();
        let role_id: String = serde_json::from_str(&args).unwrap();
        verify(args.as_bytes().to_vec(), sign.into(), pk.into());

        let role = self.roles.get(&role_id).unwrap();

        let mut guild = self.guilds.get(&role.guild_id).unwrap();
        guild.remove(&role_id);
        self.guilds.insert(&role.guild_id, &guild);

        let mut token = self.tokens.get(&role.token_id).unwrap();
        token.remove(&role_id);
        self.tokens.insert(&role.token_id, &token);

        self.roles.remove(&role_id);
    }

}