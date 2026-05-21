use axum::http::HeaderMap;
use reqwest::header::HeaderMap as ReqwestHeaderMap;

pub fn convert_headers(headers: &HeaderMap) -> ReqwestHeaderMap {
    let mut result = ReqwestHeaderMap::new();
    for (key, value) in headers.iter() {
        let key_lower = key.as_str().to_lowercase();
        if key_lower != "host" && key_lower != "content-length" {
            if let Ok(name) = reqwest::header::HeaderName::try_from(key.as_str()) {
                if let Ok(val) = value.to_str() {
                    if let Ok(parsed) = val.parse() {
                        result.insert(name, parsed);
                    }
                }
            }
        }
    }
    result
}

pub fn sanitize_log_input(input: &str) -> String {
    let sanitized: String = input
        .chars()
        .map(|c| {
            if c.is_ascii_graphic() || c.is_whitespace() {
                c
            } else {
                '?'
            }
        })
        .collect();
    sanitized.chars().take(1000).collect()
}

pub fn generate_request_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    #[test]
    fn test_convert_headers_basic() {
        let mut headers = HeaderMap::new();
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        headers.insert("authorization", HeaderValue::from_static("Bearer token"));
        
        let result = convert_headers(&headers);
        
        assert!(result.contains_key("content-type"));
        assert!(result.contains_key("authorization"));
    }

    #[test]
    fn test_convert_headers_excludes_host() {
        let mut headers = HeaderMap::new();
        headers.insert("host", HeaderValue::from_static("localhost"));
        headers.insert("content-type", HeaderValue::from_static("application/json"));
        
        let result = convert_headers(&headers);
        
        assert!(!result.contains_key("host"));
        assert!(result.contains_key("content-type"));
    }

    #[test]
    fn test_sanitize_log_input() {
        let input = "Hello\nWorld\t<script>alert('xss')</script>";
        let sanitized = sanitize_log_input(input);
        
        assert!(!sanitized.contains("<script>"));
        assert!(sanitized.contains("Hello"));
    }

    #[test]
    fn test_generate_request_id() {
        let id1 = generate_request_id();
        let id2 = generate_request_id();
        
        assert_ne!(id1, id2);
        assert!(id1.len() == 36);
    }
}
