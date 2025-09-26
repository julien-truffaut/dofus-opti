use anyhow::Result;

use dofusopti::dofus_db_client::fetch_amulets;
use dofusopti::dofus_db_parser::parse_gear;

#[tokio::main]
async fn main() -> Result<()> {

    let result = fetch_amulets(0).await?;

    for data in result.data {
        let gear = parse_gear(data);
        println!("Fetched gear from dofus db: {:?}", gear);
    }

    Ok(())

}


