use crate::*;

#[near_bindgen]
impl DiscordRoles {
    pub fn get_guild(&self, guild_id: String) -> Vec<Role> {
        let roles = match self.guilds.get(&guild_id) {
            Some(v) => {
                let mut vec: Vec<Role> = Vec::new();
                for hash in v.keys() {
                    let role = self.roles.get(&hash).unwrap();
                    vec.push(role);
                }
                vec
            },
            None => Vec::new(),
        };
        roles
    }
    
    pub fn get_field(&self, field_key: String, field_value: String) -> Vec<Role> {
        let roles = match self.key_fields.get(&(field_key, field_value)) {
            Some(v) => {
                let mut vec: Vec<Role> = Vec::new();
                for hash in v.keys() {
                    let role = self.roles.get(&hash).unwrap();
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

    pub fn get_token_list(&self) -> Vec<(String, String)> {
        self.key_fields.keys().collect()
    }
}

