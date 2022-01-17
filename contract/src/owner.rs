
use crate::*;

#[near_bindgen]
impl DiscordRoles {
    pub fn set_public_key(&mut self, pk: String) {
        assert!(self.owner_id == env::predecessor_account_id(), "owner only");
        self.public_key = pk;
    }

}