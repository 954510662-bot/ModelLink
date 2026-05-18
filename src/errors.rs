use std::fmt;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub enum ModelLinkError {
    ConfigurationError(String),
    ValidationError(String),
    NetworkError(String),
    ProtocolError(String),
    TransformError(String),
    NotFoundError(String),
    RateLimitError(String),
    AuthenticationError(String),
    InternalError(String),
}

impl std::error::Error for ModelLinkError {}

impl fmt::Display for ModelLinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModelLinkError::ConfigurationError(msg) => write!(f, "配置错误: {}", msg),
            ModelLinkError::ValidationError(msg) => write!(f, "验证错误: {}", msg),
            ModelLinkError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            ModelLinkError::ProtocolError(msg) => write!(f, "协议错误: {}", msg),
            ModelLinkError::TransformError(msg) => write!(f, "转换错误: {}", msg),
            ModelLinkError::NotFoundError(msg) => write!(f, "未找到: {}", msg),
            ModelLinkError::RateLimitError(msg) => write!(f, "限流错误: {}", msg),
            ModelLinkError::AuthenticationError(msg) => write!(f, "认证错误: {}", msg),
            ModelLinkError::InternalError(msg) => write!(f, "内部错误: {}", msg),
        }
    }
}

impl IntoResponse for ModelLinkError {
    fn into_response(self) -> Response {
        let (status, error_type) = match &self {
            ModelLinkError::ConfigurationError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "configuration_error"),
            ModelLinkError::ValidationError(_) => (StatusCode::BAD_REQUEST, "validation_error"),
            ModelLinkError::NetworkError(_) => (StatusCode::BAD_GATEWAY, "network_error"),
            ModelLinkError::ProtocolError(_) => (StatusCode::BAD_GATEWAY, "protocol_error"),
            ModelLinkError::TransformError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "transform_error"),
            ModelLinkError::NotFoundError(_) => (StatusCode::NOT_FOUND, "not_found"),
            ModelLinkError::RateLimitError(_) => (StatusCode::TOO_MANY_REQUESTS, "rate_limit_error"),
            ModelLinkError::AuthenticationError(_) => (StatusCode::UNAUTHORIZED, "authentication_error"),
            ModelLinkError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error"),
        };
        
        let body = json!({
            "error": {
                "message": self.to_string(),
                "type": error_type,
            }
        });
        
        (status, Json(body)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, ModelLinkError>;

impl From<std::io::Error> for ModelLinkError {
    fn from(e: std::io::Error) -> Self {
        ModelLinkError::NetworkError(e.to_string())
    }
}

impl From<serde_json::Error> for ModelLinkError {
    fn from(e: serde_json::Error) -> Self {
        ModelLinkError::ValidationError(e.to_string())
    }
}

impl From<serde_yaml::Error> for ModelLinkError {
    fn from(e: serde_yaml::Error) -> Self {
        ModelLinkError::ConfigurationError(e.to_string())
    }
}

impl From<url::ParseError> for ModelLinkError {
    fn from(e: url::ParseError) -> Self {
        ModelLinkError::ValidationError(e.to_string())
    }
}
