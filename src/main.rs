pub mod pokemon;
use pokemon::Pokemon;
use serde_json::from_str;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://pokeapi.co/api/v2/pokemon/1";
    let response = reqwest::get(url).await?.text().await?;

    let pokemon: Pokemon = from_str(&response)?;
    println!("{:#?}", pokemon);
    Ok(())
}
