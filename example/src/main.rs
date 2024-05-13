use std::env;

use aliyun_sms_sdk::sms::SmsClient;

#[tokio::main]
async fn main() {
    let client = SmsClient::builder()
        .access_key_id(env::var("ACCESS_KEY_ID").unwrap())
        .access_key_secret(env::var("ACCESS_KEY_SECRET").unwrap())
        .https(false)
        .build();

    let response = client.send_sms(
        &"18588888888".to_string(),
        &"模板名称".to_string(),
        &"SMS_123456789".to_string(),
        &"{\"code\": \"3306\"}".to_string()
    ).await.unwrap();

    println!("{:?}", response);
}