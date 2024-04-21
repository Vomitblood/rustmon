#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Slug {
    pub eng: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Forms {
    // ignoring actual details in the forms and just capturing form names
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Generation {
    pub forms: std::collections::HashMap<String, Forms>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PokemonRaw {
    pub idx: String,
    pub slug: Slug,
    #[serde(rename = "gen-8")]
    pub gen_8: Generation,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PokemonRawCollection {
    #[serde(flatten)]
    pub entries: std::collections::HashMap<String, PokemonRaw>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Pokemon {
    pub pokedex: String,
    pub name: String,
    pub forms: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct PokemonCollection {
    pub pokemons: Vec<Pokemon>,
}
