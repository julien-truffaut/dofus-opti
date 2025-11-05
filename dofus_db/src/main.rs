use anyhow::{Ok, Result};

use dofus_opti_dofus_db::{client::fetch_all_gears, model::DofusDbObject};
use dofus_opti_dofus_db::file::write_dofus_db_jsons;
use dofus_opti_dofus_db::parser::parse_gear;
use dofus_opti_core::model::*;
use dofus_opti_core::file::{read_gears, write_gears};

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

const IMPORT_PATH: &str = "dofus_db/data";
const EXPORT_PATH: &str = "core/data";


#[tokio::main]
async fn main() -> Result<()> {

    let args = Args::parse();

    if args.import {
        println!("Importing data from DofusDB...");
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
        println!("Exporting DofusDB data to our own model...");
        ALL_GEAR_TYPES
            .into_iter()
            .for_each(|gear_type| {
                if let Err(e) = export_gears(gear_type) {
                    eprintln!("❌ Failed to export {gear_type}: {e}");
                } else {
                    println!("✅ Finished exporting {gear_type}");
                }
            });
    }

    Ok(())

}

async fn import_gears(gear_type: &GearType) -> Result<()> {
    let result = fetch_all_gears(gear_type).await?;
    println!("Imported {} {} from dofus db", result.len(), gear_type);
    write_dofus_db_jsons(IMPORT_PATH, gear_type, &result)
}

fn export_gears(gear_type: &GearType) -> Result<()> {
    let dofus_db_objects: Vec<DofusDbObject> = read_gears(IMPORT_PATH, gear_type)?;
    let number_of_objects = dofus_db_objects.len();
    let mut gears = Vec::new();
     
    for object in dofus_db_objects {
        let object_name = object.name.en.clone();
        match parse_gear(object) {
            std::result::Result::Ok(gear) => gears.push(gear),
            Err(e) => eprintln!("❌ Failed to parse gear: {e} from {}", object_name),
        }
    }

    println!("Successfuly parsed {}/{} {gear_type}", gears.len(), number_of_objects);

    write_gears(EXPORT_PATH, gear_type, &gears)?;

    Ok(())
}