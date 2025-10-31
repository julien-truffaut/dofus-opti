use anyhow::{Context, Result};

use std::fs;
use std::path::Path;

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::model::{Gear, GearType};

pub fn write_gears<P: AsRef<Path>>(
    base_path: P,
    gear_type: &GearType,
    gears: &Vec<Gear>
) -> Result<()> {
    write_generic_gears(base_path, gear_type, gears, |gear, _| gear_file_name(&gear.name))
}

pub fn read_gears<P: AsRef<Path>>(base_path: P, gear_type: &GearType) -> Result<Vec<Gear>> {
    read_generic_gears(base_path, gear_type)
}

pub fn write_generic_gears<P, A, F>(
    base_path: P,
    gear_type: &GearType,
    gears: &Vec<A>,
    get_file_name: F
) -> Result<()> 
where
    P: AsRef<Path>, 
    A: Serialize,
    F: Fn(&A, usize) -> String,
{
    let out_dir = base_path.as_ref().join(gear_type.to_string());
    fs::create_dir_all(&out_dir).context("Failed to create output dir")?;

    for (i, object) in gears.iter().enumerate() {
        let file_name = get_file_name(&object, i);
        let file_path = out_dir.join(file_name);

        let json_str = serde_json::to_string_pretty(object)?;

        fs::write(file_path, json_str).context("Failed to write json file")?;
    }

    Ok(())
}

pub fn read_generic_gears<P, A>(base_path: P, gear_type: &GearType) -> Result<Vec<A>> 
where
    P: AsRef<Path>, 
    A: DeserializeOwned,
{
    let mut results = vec!();
    let dir = base_path.as_ref().join(gear_type.to_string());

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let data = std::fs::read_to_string(path)?;
            let gear: A = serde_json::from_str(&data)?;
            results.push(gear);
        }
    }
    Ok(results)
}

pub fn gear_file_name(gear_name: &String) -> String {
    format!("{gear_name}.json")
        .to_lowercase()
        .replace(' ', "_")
        .replace('-', "_")
        .replace("'s", "")
}

#[cfg(test)]
mod tests {
    use crate::CharacteristicRange;
    use crate::CharacteristicType::*;

    use super::*;
    use tempfile::TempDir;
    use std::collections::HashSet;

    #[test]
    fn write_read_gears() -> anyhow::Result<()> {
      let gear_1 = Gear {
        name: String::from("Great Amulet"),
        gear_type: GearType::Amulet,
        level: 200,
        characteristics: vec!(
            CharacteristicRange { kind: Strength, min: 30, max: 60 },
            CharacteristicRange { kind: Vitality, min: 100, max: 250 },
        )
      };

      let gear_2 = Gear {
        name: String::from("Deadly Amulet"),
        gear_type: GearType::Amulet,
        level: 149,
        characteristics: vec!(
            CharacteristicRange { kind: EarthDamage, min: 2, max: 10 },
            CharacteristicRange { kind: Power, min: 40, max: 80 },
        )
      };
      let gears = vec!(gear_1, gear_2);

      let base_dir = TempDir::new()?;
      let gear_type = GearType::Amulet;
      write_gears(&base_dir, &gear_type, &gears)?;
      let read_gears = read_gears(&base_dir, &gear_type)?;

      let input_set: HashSet<_> = gears.iter().collect();
      let output_set: HashSet<_> = read_gears.iter().collect();

      assert_eq!(input_set, output_set);

      Ok(())
    }

}