use async_graphql::{Schema, Object, InputObject, SimpleObject, Context, EmptySubscription};
use serde::{Deserialize, Serialize};

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct GraphQLProvider {
    pub name: String,
    pub enabled: bool,
    pub base_url: String,
    pub models: Vec<String>,
}

#[derive(SimpleObject, Clone, Debug, Serialize, Deserialize)]
pub struct GraphQLMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub average_latency_ms: f64,
}

#[derive(InputObject)]
pub struct ChatCompletionInput {
    pub model: String,
    pub messages: Vec<MessageInput>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
    pub stream: Option<bool>,
}

#[derive(InputObject, Clone)]
pub struct MessageInput {
    pub role: String,
    pub content: String,
}

#[derive(SimpleObject)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub model: String,
    pub content: String,
    pub usage: UsageInfo,
}

#[derive(SimpleObject)]
pub struct UsageInfo {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn providers(&self) -> Vec<GraphQLProvider> {
        vec![
            GraphQLProvider {
                name: "openai".to_string(),
                enabled: true,
                base_url: "https://api.openai.com/v1".to_string(),
                models: vec![
                    "gpt-4".to_string(),
                    "gpt-3.5-turbo".to_string(),
                ],
            },
            GraphQLProvider {
                name: "anthropic".to_string(),
                enabled: true,
                base_url: "https://api.anthropic.com/v1".to_string(),
                models: vec![
                    "claude-3-opus".to_string(),
                    "claude-3-sonnet".to_string(),
                ],
            },
        ]
    }

    async fn metrics(&self) -> GraphQLMetrics {
        GraphQLMetrics {
            total_requests: 1000,
            successful_requests: 980,
            failed_requests: 20,
            average_latency_ms: 150.5,
        }
    }

    async fn health_check(&self) -> bool {
        true
    }
}

pub struct MutationRoot;

#[MutationRoot]
impl MutationRoot {
    async fn chat_completion(&self, input: ChatCompletionInput) -> ChatCompletionResponse {
        ChatCompletionResponse {
            id: format!("chatcmpl-{}", uuid::Uuid::new_v4()),
            model: input.model,
            content: "This is a simulated response from GraphQL API".to_string(),
            usage: UsageInfo {
                prompt_tokens: 10,
                completion_tokens: 20,
                total_tokens: 30,
            },
        }
    }
}

pub type ModelLinkSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub fn create_schema() -> ModelLinkSchema {
    Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .finish()
}

pub async fn create_graphql_router() -> axum::Router {
    use axum::{routing::get, extract::Extension, response::Html};
    use async_graphql_axum::GraphQL;

    let schema = create_schema();
    
    async fn graphql_handler(
        Extension(schema): Extension<ModelLinkSchema>,
    ) -> impl axum::response::IntoResponse {
        GraphQL::new(schema)
    }

    async fn graphql_playground() -> Html<String> {
        Html(async_graphql::http::playground_source(
            async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
        ))
    }

    axum::Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .layer(Extension(schema))
}

use axum::{
    body::Body,
    response::IntoResponse,
    routing::get,
    Router,
};
