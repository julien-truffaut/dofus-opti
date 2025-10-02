use crate::{dofus_db_models::GetObjectsResponse, models::GearType};
use crate::{dofus_db_parser::gear_type_to_code};

pub async fn fetch_gear(gear_type: &GearType, skip: u32) -> reqwest::Result<GetObjectsResponse> {
    let gear_code = gear_type_to_code(gear_type);
    let url = format!("https://api.dofusdb.fr/items?typeId[$in][]={}&$sort=-id&$skip={}", gear_code, skip);

    let resp = reqwest::get(url).await?;
    let data: GetObjectsResponse = resp.json().await?;
    Ok(data)
}