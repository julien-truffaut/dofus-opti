use anyhow::Result;

use dofus_opti_dofus_db::client::fetch_all_gears;
use dofus_opti_dofus_db::file::save_gears;
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
            if let Err(e) = fetch_and_save_gears(gear_type).await {
                eprintln!("❌ Failed to save {gear_type}: {e}");
            } else {
                println!("✅ Finished saving {gear_type}");
            }
        })
        .await;
    }

    if args.export {
        println!("📤 Exporting DofusDB data to model format...");
    }

    Ok(())

}

async fn fetch_and_save_gears(gear_type: &GearType) -> Result<()> {
    let result = fetch_all_gears(gear_type).await?;
    println!("Imported {} {} from dofus db", result.len(), gear_type);
    save_gears(DOFUS_DB_EXPORT_PATH, gear_type, &result)
}