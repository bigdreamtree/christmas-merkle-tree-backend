use reqwest::multipart;
use serde::{Deserialize, Serialize};
use std::env;

const PINATA_UPLOAD_URL: &str = "https://uploads.pinata.cloud";

#[derive(Serialize, Deserialize, Debug)]
pub struct PinataUploadFileData {
    pub id: String,
    pub name: String,
    pub cid: String,
    pub size: u64,
    pub number_of_files: u64,
    pub mime_type: String,
    pub user_id: String,
    pub group_id: String,
    pub is_duplicate: Option<bool>,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinataUploadFileResponse {
    pub data: PinataUploadFileData,
}

pub async fn upload_file(data: String, file_name: String) -> Result<PinataUploadFileResponse, Box<dyn std::error::Error>> {
    let jwt = env::var("PINATA_JWT").expect("Pinata JWT not set");
    let group_id = env::var("PINATA_GROUP_ID").expect("Pinata Group ID not set");
    
    let client = reqwest::Client::new();

    // create multipart form
    let part = multipart::Part::bytes(data.into_bytes()).file_name(file_name);
    let form = multipart::Form::new().part("file", part).text("group_id", group_id);

    // 요청 보내기
    let response = client
        .post(PINATA_UPLOAD_URL.to_owned() + "/v3/files")
        .bearer_auth(jwt)
        .multipart(form)
        .send()
        .await?;

    if response.status().is_success() {
        let response_json: PinataUploadFileResponse = response.json().await?;
        Ok(response_json)
    } else {
        let response_text = response.text().await?;
        Err(format!("Failed to upload file: {response_text}").into())
    }
}

pub async fn get_file(file_id: String) -> Result<PinataUploadFileResponse, Box<dyn std::error::Error>> {
    let jwt = env::var("PINATA_JWT").expect("Pinata JWT not set");

    let client = reqwest::Client::new();

    let response = client
        .get(PINATA_UPLOAD_URL.to_owned() + "/v3/files/" + &file_id)
        .bearer_auth(jwt)
        .query(&[("pinataMetadata", "true"), ("pinataContent", "true")])
        .send()
        .await?;

    if response.status().is_success() {
        let response_json: PinataUploadFileResponse = response.json().await?;
        Ok(response_json)
    } else {
        let response_text = response.text().await?;
        Err(format!("Failed to get file: {response_text}").into())
    }
}