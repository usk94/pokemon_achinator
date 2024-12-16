use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    pub id: u32,
    pub name: String,
    pub base_experience: u32,
    pub height: u32,
    pub is_default: bool,
    pub order: u32,
    pub weight: u32,
    pub abilities: Vec<PokemonAbility>,
}

#[derive(Deserialize, Debug)]
pub struct PokemonAbility {
    pub is_hidden: bool,
    pub slot: u32,
    pub ability: NamedAPIResource,
}

#[derive(Deserialize, Debug)]
pub struct VersionGameIndex {
    pub game_index: u32,
    pub version: NamedAPIResource,
}

#[derive(Deserialize, Debug)]
pub struct PokemonHeldItem {
    pub rarity: u32,
    pub version: NamedAPIResource,
}

#[derive(Deserialize, Debug)]
pub struct PokemonMove {
    pub r#move: NamedAPIResource,
    pub version_group_details: Vec<MoveVersionGroupDetail>,
}

#[derive(Deserialize, Debug)]
pub struct MoveVersionGroupDetail {
    pub level_learned_at: u32,
    pub version_group: NamedAPIResource,
    pub move_learn_method: NamedAPIResource,
}

#[derive(Deserialize, Debug)]
pub struct PokemonSprites {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
    pub front_female: Option<String>,
    pub front_shiny_female: Option<String>,
    pub back_default: Option<String>,
    pub back_shiny: Option<String>,
    pub back_female: Option<String>,
    pub back_shiny_female: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PokemonStat {
    pub stat: NamedAPIResource,
    pub effort: u32,
    pub base_stat: u32,
}

#[derive(Deserialize, Debug)]
pub struct PokemonType {
    pub slot: u32,
    pub r#type: NamedAPIResource,
}

#[derive(Deserialize, Debug)]
pub struct NamedAPIResource {
    pub name: String,
    pub url: String,
}
