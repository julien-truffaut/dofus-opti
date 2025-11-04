use anyhow::Result;

use std::path::Path;

use dofus_opti_core::model::GearType;
use dofus_opti_core::file::{write_generic_gears, read_generic_gears};

pub fn write_dofus_db_jsons<P: AsRef<Path>>(
    base_path: P,
    gear_type: &GearType,
    gears: &Vec<serde_json::Value>,
) -> Result<()> {
    write_generic_gears(base_path, gear_type, gears, |g, i| get_file_name(g, i))
}

fn get_file_name(object: &serde_json::Value, index: usize) -> String {
    let object_name = object["name"]["en"]
        .as_str()
        .map(String::from)
        .unwrap_or(format!("unkown_{}", index));
    format!("{object_name}.json")
        .to_lowercase()
        .trim()
        .replace(' ', "_")
        .replace('-', "_")
        .replace("'s", "")
}

pub fn read_dofus_db_jsons<P: AsRef<Path>>(base_path: P, gear_type: &GearType) -> Result<Vec<serde_json::Value>> {
    read_generic_gears(base_path, gear_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_file_name_with_english_name() -> anyhow::Result<()>  {
      let data = r#"
        {
            "name" : {
                "en": "Great Amulet",
                "fr": "Grande Amulette"
            }
        }"#;
      let json: serde_json::Value = serde_json::from_str(data)?;
      assert_eq!(get_file_name(&json, 5), String::from("great_amulet.json"));
      Ok(())
    }

    #[test]
    fn get_file_name_without_english_name() -> anyhow::Result<()>  {
      let data = r#"
        {
            "name" : {
                "fr": "Grande Amulette"
            }
        }"#;
      let json: serde_json::Value = serde_json::from_str(data)?;
      assert_eq!(get_file_name(&json, 5), String::from("unkown_5.json"));
      Ok(())
    }

}