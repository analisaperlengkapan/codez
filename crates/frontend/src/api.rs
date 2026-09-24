pub const BASE_URL: &str = "/api/v1";

pub fn api_url(path: &str) -> String {
    format!("{}{}", BASE_URL, path)
}
