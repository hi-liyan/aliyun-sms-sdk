# aliyun-sms-sdk

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

阿里云短信服务 sdk for rust。

## 实现

- [X] 短信发送 SendSms
- [ ] ...

## 用法

### 添加依赖

```toml
[dependencies]
aliyun-sms-sdk = { git = "https://github.com/hi-liyan/aliyun-sms-sdk.git" }
```

### 构建客户端

```rust
fn main() {
    let client = SmsClient::builder()
        // 阿里云 ACCESS_KEY_ID 必填
        .access_key_id("access_key_id".to_string())
        // 阿里云 ACCESS_KEY_SECRET 必填
        .access_key_secret("access_key_secret".to_string())
        // 请求超时时间 从请求发起开始直到响应结束 默认3000
        .timeout(3000)
        // 是否开启 HTTPS 默认开启
        .https(true)
        // reqwest 参数 是否忽略证书
        .danger_accept_invalid_certs(false)
        .build();
}
```

### 发送短信 SendSms

```rust
async fn main() {
    let response = client.send_sms(
        &"18588888888".to_string(),
        &"模板名称".to_string(),
        &"SMS_123456789".to_string(),
        &"{\"code\": \"3306\"}".to_string()
    ).await.unwrap();
    
    println!("response: {:?}", response);
}
```
