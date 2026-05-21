use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use model_link::*;
use serde_json::json;
use std::time::Duration;

fn bench_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_validation");
    
    let schema = get_config_schema();
    let validator = ConfigValidator::new(schema).expect("Failed to create validator");
    
    let valid_config = json!({
        "server": {
            "host": "127.0.0.1",
            "port": 8080,
            "tls": {
                "enabled": false
            }
        },
        "providers": {
            "openai": {
                "base_url": "https://api.openai.com/v1",
                "api_key": "sk-test-key",
                "enabled": true,
                "timeout": 30,
                "max_retries": 3
            },
            "anthropic": {
                "base_url": "https://api.anthropic.com/v1",
                "api_key": "sk-ant-test-key",
                "enabled": true
            }
        },
        "default_provider": "openai",
        "rate_limit": {
            "enabled": true,
            "requests_per_minute": 60,
            "requests_per_hour": 1000
        },
        "logging": {
            "level": "info",
            "format": "json"
        },
        "health_check": {
            "enabled": true,
            "interval_seconds": 30
        }
    });

    group.bench_function("simple_config", |b| {
        let simple_config = json!({
            "server": {
                "host": "127.0.0.1",
                "port": 8080
            },
            "providers": {
                "openai": {
                    "api_key": "test-key"
                }
            }
        });
        
        b.iter(|| {
            validator.validate(black_box(&simple_config))
        });
    });

    group.bench_function("complex_config", |b| {
        b.iter(|| {
            validator.validate(black_box(&valid_config))
        });
    });

    group.bench_function("validate_config_function", |b| {
        let simple_config = json!({
            "server": {
                "host": "127.0.0.1",
                "port": 8080
            },
            "providers": {
                "openai": {
                    "api_key": "test-key"
                }
            }
        });
        
        b.iter(|| {
            validate_config(black_box(&simple_config))
        });
    });

    group.finish();
}

fn bench_http_client(c: &mut Criterion) {
    let mut group = c.benchmark_group("http_operations");
    
    group.bench_function("create_provider", |b| {
        b.iter(|| {
            let _provider = create_provider(
                black_box("openai"),
                black_box("test-api-key".to_string())
            );
        });
    });

    group.bench_function("provider_query", |b| {
        let provider = create_provider("openai", "test-api-key".to_string());
        
        b.iter(|| {
            black_box(provider.name());
            black_box(provider.base_url());
            black_box(provider.supports_streaming());
            black_box(provider.supports_functions());
        });
    });

    group.bench_function("schema_generation", |b| {
        b.iter(|| {
            let _schema = get_config_schema();
        });
    });

    group.finish();
}

fn bench_json_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("json_operations");
    
    let request_json = r#"{
        "model": "gpt-4",
        "messages": [
            {"role": "user", "content": "Hello, how are you?"}
        ],
        "max_tokens": 100,
        "temperature": 0.7
    }"#;

    group.bench_function("parse_json", |b| {
        b.iter(|| {
            let _: serde_json::Value = serde_json::from_str(black_box(request_json)).unwrap();
        });
    });

    group.bench_function("serialize_json", |b| {
        let value = json!({
            "model": "gpt-4",
            "messages": [
                {"role": "user", "content": "Hello"}
            ],
            "max_tokens": 100
        });
        
        b.iter(|| {
            let _ = serde_json::to_string(black_box(&value));
        });
    });

    group.bench_function("json_pointer_access", |b| {
        let value = json!({
            "server": {
                "host": "127.0.0.1",
                "port": 8080
            },
            "providers": {
                "openai": {
                    "api_key": "test-key"
                }
            }
        });
        
        b.iter(|| {
            let _ = value.get("server");
            let _ = value.pointer("/server/host");
            let _ = value.pointer("/providers/openai/api_key");
        });
    });

    group.finish();
}

fn bench_provider_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("provider_creation");
    
    let provider_types = ["openai", "anthropic", "gemini", "deepseek", "cohere"];
    
    group.bench_function("create_single_provider", |b| {
        b.iter(|| {
            let _provider = create_provider(
                black_box("openai"),
                black_box("test-key".to_string())
            );
        });
    });

    group.bench_function("create_all_provider_types", |b| {
        b.iter(|| {
            for provider_type in provider_types {
                let _ = create_provider(
                    black_box(provider_type),
                    black_box("test-key".to_string())
                );
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_validation,
    bench_http_client,
    bench_json_operations,
    bench_provider_creation
);
criterion_main!(benches);
