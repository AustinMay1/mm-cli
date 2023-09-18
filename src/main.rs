use monstermash::Players;

const BASE_URL: &str = "https://api.wiseoldman.net/v2/groups/1500/gained?metric=barrows_chests&period=week"; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
    let total = Players::get_totals(BASE_URL).await?;
    println!("{total}");
    Ok(())
}
