use dofus_opti_core::model::GearType;

use crate::model::{DofusDbTypeId, GetObjectsResponse};

pub async fn fetch_all_gears(gear_type: &GearType) -> reqwest::Result<Vec<serde_json::Value>> {
    let mut gears: Vec<serde_json::Value> = vec!();

    loop {
        let mut response = fetch_gear(gear_type, gears.len()).await?;
        // println!("Received {} {gear_type}", response.data.len());
        if response.data.is_empty() {
            break;
        } else {
            gears.append(&mut response.data);
        }
    }

    Ok(gears)
}

pub async fn fetch_gear(gear_type: &GearType, skip: usize) -> reqwest::Result<GetObjectsResponse> {
    let type_id = DofusDbTypeId::from(gear_type);
    let url = format!("https://api.dofusdb.fr/items?typeId[$in][]={}&$sort=-id&$skip={}", type_id.0, skip);

    let resp = reqwest::get(url).await?;
    let data: GetObjectsResponse = resp.json().await?;
    Ok(data)
}