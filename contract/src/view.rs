use crate::*;

#[near_bindgen]
impl DiscordRoles {
    pub fn get_guild(&self, guild_id: String) -> Vec<Role> {
        let roles = match self.guilds.get(&guild_id) {
            Some(v) => {
                let mut vec: Vec<Role> = Vec::new();
                for role_id in v.keys() {
                    let role = self.roles.get(&role_id).unwrap();
                    vec.push(role);
                }
                vec
            },
            None => Vec::new(),
        };
        roles
    }
    
    pub fn get_token(&self, token_id: AccountId) -> Vec<Role> {
        let roles = match self.tokens.get(&token_id) {
            Some(v) => {
                let mut vec: Vec<Role> = Vec::new();
                for role_id in v.keys() {
                    let role = self.roles.get(&role_id).unwrap();
                    vec.push(role);
                }
                vec
            },
            None => Vec::new(),
        };
        roles
    }
    
    pub fn get_role(&self, role_id: String) -> Option<Role> {
        self.roles.get(&role_id)
    }

    pub fn get_token_list(&self) -> Vec<AccountId> {
        self.tokens.keys().collect()
    }
}

