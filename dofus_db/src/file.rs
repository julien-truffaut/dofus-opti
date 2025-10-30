use anyhow::{Context, Result};

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use dofus_opti_core::model::GearType;

pub fn save_dofus_db_jsons<P: AsRef<Path>>(
    base_path: P,
    gear_type: &GearType,
    gears: &Vec<serde_json::Value>,
) -> Result<()> {
    let out_dir = base_path.as_ref().join(gear_type.to_string());
    fs::create_dir_all(&out_dir).context("Failed to create output dir")?;

    for (i, object) in gears.iter().enumerate() {
        let object_name = get_object_name(object, i);
        let file_name = create_filename(&object_name);
        let file_path = out_dir.join(file_name);

        let json_str = serde_json::to_string_pretty(object)?;

        fs::write(file_path, json_str).context("Failed to write json file")?;
    }

    Ok(())
}

fn get_object_name(object: &serde_json::Value, index: usize) -> String {
    object["name"]["en"]
        .as_str()
        .map(String::from)
        .unwrap_or(format!("unkown_{}", index))
}

fn create_filename(object_name: &str) -> String {
    format!("{object_name}.json")
        .to_lowercase()
        .replace(' ', "_")
        .replace('-', "_")
        .replace("'s", "")
}

pub fn read_dofus_db_jsons<P: AsRef<Path>>(base_path: P, gear_type: &GearType) -> Result<Vec<serde_json::Value>> {
    let mut results = vec!();
    let dir = base_path.as_ref().join(gear_type.to_string());

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let json = read_json(path)?;
            results.push(json);
        }
    }
    Ok(results)
}

pub fn read_json<P: AsRef<Path>>(path: P) -> Result<serde_json::Value> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let json: serde_json::Value = serde_json::from_reader(reader)?;
    Ok(json)
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::collections::HashSet;

    #[test]
    fn get_object_name_with_english_name() -> anyhow::Result<()>  {
      let data = r#"
        {
            "name" : {
                "en": "Great Amulet",
                "fr": "Grande Amulette"
            }
        }"#;
      let json: serde_json::Value = serde_json::from_str(data)?;
      assert_eq!(get_object_name(&json, 5), String::from("Great Amulet"));
      Ok(())
    }

    #[test]
    fn get_object_name_without_english_name() -> anyhow::Result<()>  {
      let data = r#"
        {
            "name" : {
                "fr": "Grande Amulette"
            }
        }"#;
      let json: serde_json::Value = serde_json::from_str(data)?;
      assert_eq!(get_object_name(&json, 5), String::from("unkown_5"));
      Ok(())
    }

    #[test]
    fn write_read_gears() -> anyhow::Result<()> {
      let json_1 = r#"
      {
          "name" : {
                "en": "Great Amulet",
                "fr": "Grande Amulette"
            }
      }"#;

      let json_2 = r#"
      {
          "foo" : "bar"
      }"#;
   
      let json_strs = vec!(json_1, json_2);
      let json_values = json_strs.into_iter()
        .map(|str| serde_json::from_str(str))
        .collect::<Result<Vec<_>, _>>()?;

      let base_dir = TempDir::new()?;
      let gear_type = GearType::Amulet;
      save_dofus_db_jsons(&base_dir, &gear_type, &json_values)?;
      let read_json_values = read_dofus_db_jsons(&base_dir, &gear_type)?;

      let input_set: HashSet<_> = json_values.iter().collect();
      let output_set: HashSet<_> = read_json_values.iter().collect();

      assert_eq!(input_set, output_set);

      Ok(())
    }

}