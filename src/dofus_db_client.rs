use crate::dofus_db_models::GetObjectsResponse;

pub async fn fetch_amulets(skip: u32) -> reqwest::Result<GetObjectsResponse> {
    let url = format!("https://api.dofusdb.fr/items?typeId[$in][]=1&$sort=-id&$skip={}", skip);

    let resp = reqwest::get(url).await?;
    let data: GetObjectsResponse = resp.json().await?;
    Ok(data)
}