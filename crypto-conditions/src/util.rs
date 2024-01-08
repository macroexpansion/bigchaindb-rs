use base64_url;

pub struct Base64Url;

impl Base64Url {
    pub fn decode(url: &str) -> Vec<u8> {
        let buffer = base64_url::decode(url).unwrap();

        buffer
    }

    pub fn encode(buffer: &[u8]) -> String {
        let url = base64_url::encode(buffer);
        let escaped_url = base64_url::escape(&url);

        escaped_url.to_string()
    }
}
