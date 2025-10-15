use crate::{dofus_db_models::GetObjectsResponse, models::GearType};
use crate::{dofus_db_parser::gear_type_to_code};

pub async fn fetch_all_gears(gear_type: &GearType) -> reqwest::Result<Vec<serde_json::Value>> {
    let mut gears: Vec<serde_json::Value> = vec!();

    loop {
        let mut response = fetch_gear(gear_type, gears.len()).await?;
        println!("Received {} {gear_type}", response.data.len());
        if response.data.is_empty() {
            break;
        } else {
            gears.append(&mut response.data);
        }
    }

    Ok(gears)
}

pub async fn fetch_gear(gear_type: &GearType, skip: usize) -> reqwest::Result<GetObjectsResponse> {
    let gear_code = gear_type_to_code(gear_type);
    let url = format!("https://api.dofusdb.fr/items?typeId[$in][]={}&$sort=-id&$skip={}", gear_code, skip);

    let resp = reqwest::get(url).await?;
    let data: GetObjectsResponse = resp.json().await?;
    Ok(data)
}