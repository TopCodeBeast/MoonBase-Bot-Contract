use crate::*;

#[near_bindgen]
impl DiscordRoles {
    pub(crate) fn internal_set_role(&mut self, guild_id: String, role_id: String, fields: HashMap<String, String>, key_field: (String, String)) {
        let mut guild = match self.guilds.get(&guild_id) {
            Some(v) => v,
            None => UnorderedMap::new(guild_id.as_bytes().to_vec())
        };
        
        guild.insert(&role_id, &true);
        self.guilds.insert(&guild_id, &guild);

        let mut _key_field = match self.key_fields.get(&key_field) {
            Some(v) => v,
            None => UnorderedMap::new((key_field.0.clone() + &key_field.1.clone()).as_bytes().to_vec())
        };
        _key_field.insert(&role_id, &true);
        self.key_fields.insert(&key_field, &_key_field);


        let role = Role {
            guild_id: guild_id.clone(),
            role_id: role_id.clone(),
            fields,
            key_field
        };
        self.roles.insert(&role_id, &role);
    }
}