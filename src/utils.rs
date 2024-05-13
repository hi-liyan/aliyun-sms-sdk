use chrono::Utc;
use hmac::{Hmac, Mac};
use percent_encoding::{percent_encode, AsciiSet, CONTROLS};
use sha1::Sha1;
use sha2::Sha256;
use sha256::digest;

/// 获取 UTC 时间
pub fn get_utc() -> String {
    let now = Utc::now();
    let utc = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    utc
}

/// HMAC SHA1 加密
#[allow(unused)]
pub fn hmac_sha1(key: &[u8], message: &[u8]) -> Vec<u8> {
    let mut hmac: Hmac<Sha1> =
        Hmac::<Sha1>::new_from_slice(key).expect("HMAC can take key of any size");
    hmac.update(message);
    hmac.finalize().into_bytes().to_vec()
}

/// SHA256 摘要
pub fn sha256(message: &[u8]) -> String {
    digest(message)
}

/// HMAC SHA256 加密
pub fn hmac_sha256(key: &[u8], message: &[u8]) -> Vec<u8> {
    let mut hamc = Hmac::<Sha256>::new_from_slice(key).expect("HMAC can take key of any size");
    hamc.update(message);
    hamc.finalize().into_bytes().to_vec()
}

/// HMAC SHA256 加密，返回 hex 字符串
pub fn hmac_sha256_hex(key: &[u8], message: &[u8]) -> String {
    hex::encode(hmac_sha256(key, message))
}

pub const DEFAULT_ENCODING_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'!')
    .add(b'"')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b',')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'<')
    .add(b'=')
    .add(b'>')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}');

/// URL 编码
fn encode_param(param: &str) -> String {
    percent_encode(param.as_bytes(), DEFAULT_ENCODING_SET).collect::<String>()
}

/// 生成 Canonical Query String
pub fn generate_canonical_query_string(params: &[(&str, &str)]) -> String {
    let mut encoded_params: Vec<String> = params
        .iter()
        .map(|(key, value)| format!("{}={}", encode_param(key), encode_param(value)))
        .collect();
    encoded_params.sort();
    encoded_params.join("&")
}
