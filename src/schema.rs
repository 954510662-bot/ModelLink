use jsonschema::JSONSchema;
use serde_json::Value;

pub struct ConfigValidator {
    schema: JSONSchema,
}

impl ConfigValidator {
    pub fn new(schema: Value) -> anyhow::Result<Self> {
        let schema = JSONSchema::compile(&schema)?;
        Ok(Self { schema })
    }
    
    pub fn validate(&self, config: &Value) -> Result<(), Vec<String>> {
        let result = self.schema.validate(config);
        match result {
            Ok(_) => Ok(()),
            Err(errors) => {
                let msgs: Vec<String> = errors.map(|e| e.to_string()).collect();
                Err(msgs)
            }
        }
    }
}

pub fn get_config_schema() -> Value {
    json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "type": "object",
        "properties": {
            "server": {
                "type": "object",
                "properties": {
                    "host": {
                        "type": "string",
                        "description": "Server bind host address",
                        "default": "127.0.0.1"
                    },
                    "port": {
                        "type": "integer",
                        "description": "Server port number",
                        "minimum": 1,
                        "maximum": 65535,
                        "default": 8080
                    },
                    "tls": {
                        "type": "object",
                        "properties": {
                            "enabled": {
                                "type": "boolean",
                                "default": false
                            },
                            "cert_path": {
                                "type": "string"
                            },
                            "key_path": {
                                "type": "string"
                            }
                        }
                    }
                },
                "required": ["host", "port"]
            },
            "providers": {
                "type": "object",
                "description": "AI provider configurations",
                "additionalProperties": {
                    "type": "object",
                    "properties": {
                        "base_url": {
                            "type": "string",
                            "description": "Provider API base URL"
                        },
                        "api_key": {
                            "type": "string",
                            "description": "Provider API key"
                        },
                        "enabled": {
                            "type": "boolean",
                            "default": true
                        },
                        "timeout": {
                            "type": "integer",
                            "description": "Request timeout in seconds",
                            "minimum": 1,
                            "maximum": 300
                        },
                        "max_retries": {
                            "type": "integer",
                            "description": "Maximum number of retries",
                            "minimum": 0,
                            "maximum": 10,
                            "default": 3
                        }
                    },
                    "required": ["api_key"]
                }
            },
            "default_provider": {
                "type": "string",
                "description": "Default AI provider name"
            },
            "rate_limit": {
                "type": "object",
                "properties": {
                    "enabled": {
                        "type": "boolean",
                        "default": true
                    },
                    "requests_per_minute": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 10000,
                        "default": 60
                    },
                    "requests_per_hour": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 100000,
                        "default": 1000
                    }
                }
            },
            "logging": {
                "type": "object",
                "properties": {
                    "level": {
                        "type": "string",
                        "enum": ["trace", "debug", "info", "warn", "error"],
                        "default": "info"
                    },
                    "format": {
                        "type": "string",
                        "enum": ["json", "pretty"],
                        "default": "json"
                    }
                }
            },
            "health_check": {
                "type": "object",
                "properties": {
                    "enabled": {
                        "type": "boolean",
                        "default": true
                    },
                    "interval_seconds": {
                        "type": "integer",
                        "minimum": 1,
                        "maximum": 3600,
                        "default": 30
                    }
                }
            }
        },
        "required": ["server", "providers"]
    })
}

pub fn validate_config(config: &Value) -> anyhow::Result<()> {
    let schema = get_config_schema();
    let validator = ConfigValidator::new(schema)?;
    validator.validate(config).map_err(|errors| {
        anyhow::anyhow!("Configuration validation failed: {}", errors.join("; "))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_validation_valid() {
        let config = json!({
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

        let result = validate_config(&config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_config_validation_invalid_port() {
        let config = json!({
            "server": {
                "host": "127.0.0.1",
                "port": 70000
            },
            "providers": {
                "openai": {
                    "api_key": "test-key"
                }
            }
        });

        let result = validate_config(&config);
        assert!(result.is_err());
    }

    #[test]
    fn test_config_validation_missing_required() {
        let config = json!({
            "server": {
                "host": "127.0.0.1"
            }
        });

        let result = validate_config(&config);
        assert!(result.is_err());
    }
}
