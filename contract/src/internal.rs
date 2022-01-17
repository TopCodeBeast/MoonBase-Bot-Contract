use crate::*;

#[near_bindgen]
impl DiscordRoles {
    pub(crate) fn internal_set_role(&mut self, guild_id: String, role_id: String, token_id: String, amount: U128) {
        let mut guild = match self.guilds.get(&guild_id) {
            Some(v) => v,
            None => UnorderedMap::new(guild_id.as_bytes().to_vec())
        };
        
        guild.insert(&role_id, &true);
        self.guilds.insert(&guild_id, &guild);

        let mut token = match self.tokens.get(&token_id) {
            Some(v) => v,
            None => UnorderedMap::new(token_id.as_bytes().to_vec())
        };
        token.insert(&role_id, &true);
        self.tokens.insert(&token_id, &token);


        let role = Role {
            guild_id: guild_id.clone(),
            role_id: role_id.clone(),
            token_id,
            amount
        };
        self.roles.insert(&role_id, &role);
    }
}