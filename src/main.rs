use anyhow::Result;

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use dofusopti::dofus_db_models::DofusDbObject;
use dofusopti::dofus_db_client::fetch_amulets;
use dofusopti::dofus_db_parser::parse_gear;
use dofusopti::models::GearType;

const DOFUS_DB_EXPORT_PATH: &str = "dofus_db/data";

#[tokio::main]
async fn main() -> Result<()> {

    let result = fetch_amulets(0).await?;

    import_dofus_db_data(&result.data, GearType::Amulet).unwrap();

    let object = read_object_from_file(format!("{DOFUS_DB_EXPORT_PATH}/amulet_albueran_warrior_amulet.json")).unwrap();
    let gear = parse_gear(object);

    println!("Gear from the filesystem: {:?}", gear);

    Ok(())

}


fn import_dofus_db_data(objects: &Vec<serde_json::Value>, gear_type: GearType) -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = Path::new(DOFUS_DB_EXPORT_PATH);
    fs::create_dir_all(out_dir)?;

    for (i, object) in objects.iter().enumerate() {
        let object_name = get_object_name(object, i);
        let file_name = create_filename(&gear_type, &object_name);
        let file_path = out_dir.join(file_name);

        let json_str = serde_json::to_string_pretty(object)?;

        fs::write(file_path, json_str)?;
    }

    Ok(())
}

fn create_filename(gear_type: &GearType, object_name: &str) -> String {
    format!("{gear_type}_{object_name}.json")
        .to_lowercase()
        .replace(' ', "_")
        .replace('-', "_")
        .replace("'s", "")
}

fn get_object_name(object: &serde_json::Value, index: usize) -> String {
    object["name"]["en"]
        .as_str()
        .map(String::from)
        .unwrap_or(format!("unkown_{}", index))
}

fn read_object_from_file<P: AsRef<Path>>(path: P) -> Result<DofusDbObject> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let object: DofusDbObject = serde_json::from_reader(reader)?;
    Ok(object)
}
