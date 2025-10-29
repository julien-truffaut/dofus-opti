use anyhow::Result;

use dofusopti::dofus_db_client::fetch_all_gears;
use dofusopti::dofus_db_file::save_gears;
use dofusopti::models::*;

use futures::{stream, StreamExt};

const DOFUS_DB_EXPORT_PATH: &str = "dofus_db/data";

#[tokio::main]
async fn main() -> Result<()> {

    stream::iter(ALL_GEAR_TYPES)
        .for_each_concurrent(5, |gear_type| async move {
            if let Err(e) = fetch_and_save_gears(gear_type).await {
                eprintln!("❌ Failed to save {gear_type}: {e}");
            } else {
                println!("✅ Finished saving {gear_type}");
            }
        })
        .await;

    // let object = read_object_from_file(format!("{DOFUS_DB_EXPORT_PATH}/amulet_albueran_warrior_amulet.json")).unwrap();
    // let gear = parse_gear(object);

    // println!("Gear from the filesystem: {:?}", gear);

    Ok(())

}

async fn fetch_and_save_gears(gear_type: &GearType) -> Result<()> {
    let result = fetch_all_gears(gear_type).await?;
    println!("Imported {} {} from dofus db", result.len(), gear_type);
    save_gears(DOFUS_DB_EXPORT_PATH, gear_type, &result)
}