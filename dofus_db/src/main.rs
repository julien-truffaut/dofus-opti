use anyhow::{Ok, Result};

use dofus_opti_dofus_db::{client::fetch_all_gears, model::DofusDbObject};
use dofus_opti_dofus_db::file;
use dofus_opti_dofus_db::parser::parse_gear;
use dofus_opti_core::model::*;

use clap::Parser;

use futures::{stream, StreamExt};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Import data from DofusDB and save locally
    #[arg(short = 'i', long = "import")]
    import: bool,

    /// Parse local DofusDB JSON files into Rust models and export them
    #[arg(short = 'e', long = "export")]
    export: bool,
}

const DOFUS_DB_EXPORT_PATH: &str = "dofus_db/data";

#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();

    if args.import {
        println!("📥 Importing data from DofusDB...");
        stream::iter(ALL_GEAR_TYPES)
        .for_each_concurrent(5, |gear_type| async move {
            if let Err(e) = import_gears(gear_type).await {
                eprintln!("❌ Failed to import {gear_type}: {e}");
            } else {
                println!("✅ Finished importing {gear_type}");
            }
        })
        .await;
    }

    if args.export {
        println!("📤 Exporting DofusDB data to model format...");
        stream::iter(ALL_GEAR_TYPES)
        .for_each_concurrent(5, |gear_type| async move {
            if let Err(e) = export_gears(gear_type).await {
                eprintln!("❌ Failed to export {gear_type}: {e}");
            } else {
                println!("✅ Finished exporting {gear_type}");
            }
        })
        .await;
    }

    Ok(())

}

async fn import_gears(gear_type: &GearType) -> Result<()> {
    let result = fetch_all_gears(gear_type).await?;
    println!("Imported {} {} from dofus db", result.len(), gear_type);
    file::save_dofus_db_jsons(DOFUS_DB_EXPORT_PATH, gear_type, &result)
}

async fn export_gears(gear_type: &GearType) -> Result<()> {
    let json_gears = file::read_dofus_db_jsons(DOFUS_DB_EXPORT_PATH, gear_type)?;
    let number_of_json = json_gears.len();
    let dofus_db_objects: Vec<DofusDbObject> = json_gears
        .into_iter()
        .map(|json| serde_json::from_value(json))
        .collect::<Result<_, _>>()?;
        
    let mut gears = Vec::new();
     
    for object in dofus_db_objects {
        let object_name = object.name.en.clone();
        match parse_gear(object) {
            std::result::Result::Ok(gear) => gears.push(gear),
            Err(e) => eprintln!("❌ Failed to parse gear: {e} from {}", object_name),
        }
    }

    println!("Successfuly parsed {}/{} {gear_type}", gears.len(), number_of_json);

    Ok(())
}