use crate::utils::get_hash;
use crate::*;

#[near_bindgen]
impl DiscordRoles {
    pub(crate) fn internal_set_role(&mut self, guild_id: String, role_id: String, fields: HashMap<String, String>, key_field: (String, String)) {
        let hash = get_hash(guild_id.clone(), role_id.clone(), fields.clone(), key_field.clone());
        let mut guild = match self.guilds.get(&guild_id) {
            Some(v) => v,
            None => UnorderedMap::new(guild_id.as_bytes().to_vec())
        };
        
        guild.insert(&hash, &true);
        self.guilds.insert(&guild_id, &guild);

        let mut _key_field = match self.key_fields.get(&key_field) {
            Some(v) => v,
            None => UnorderedMap::new((key_field.0.clone() + &key_field.1.clone()).as_bytes().to_vec())
        };
        _key_field.insert(&hash, &true);
        self.key_fields.insert(&key_field, &_key_field);

        let role = Role {
            guild_id: guild_id.clone(),
            role_id: role_id.clone(),
            fields,
            key_field
        };
        self.roles.insert(&hash, &role);
    }
}