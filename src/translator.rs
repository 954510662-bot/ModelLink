use serde_json::{json, Value};
use std::collections::HashMap;
use tracing;

use crate::config::ModelCapabilities;

pub struct ParameterTranslator {
    #[allow(dead_code)]
    source_caps: Option<ModelCapabilities>,
    target_caps: Option<ModelCapabilities>,
}

impl ParameterTranslator {
    pub fn new() -> Self {
        Self {
            source_caps: None,
            target_caps: None,
        }
    }
    
    pub fn with_target_capabilities(capabilities: ModelCapabilities) -> Self {
        Self {
            source_caps: None,
            target_caps: Some(capabilities),
        }
    }
    
    pub fn translate_request(&self, mut request: Value) -> TranslateResult<Value> {
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        
        if let Some(caps) = &self.target_caps {
            request = self.translate_temperature(request, caps, &mut warnings);
            request = self.translate_top_p(request, caps, &mut warnings);
            request = self.translate_top_k(request, caps, &mut warnings);
            request = self.translate_max_tokens(request, caps, &mut warnings);
            request = self.translate_functions(request, caps, &mut errors, &mut warnings);
            request = self.translate_parameter_aliases(request, caps);
        }
        
        TranslateResult {
            request,
            warnings,
            errors,
        }
    }
    
    fn translate_temperature(&self, mut request: Value, caps: &ModelCapabilities, warnings: &mut Vec<String>) -> Value {
        if !caps.supports_temperature {
            if request.get("temperature").is_some() {
                warnings.push("temperature not supported by target model, removing".to_string());
                if let Some(obj) = request.as_object_mut() {
                    obj.remove("temperature");
                }
            }
            return request;
        }
        
        if let Some(temp) = request.get("temperature").and_then(|v| v.as_f64()) {
            if let Some(max_temp) = caps.max_temperature {
                if temp > max_temp as f64 {
                    warnings.push(format!(
                        "temperature {} exceeds max {}, clamping to {}",
                        temp, max_temp, max_temp
                    ));
                    if let Some(obj) = request.as_object_mut() {
                        if let Some(t) = obj.get_mut("temperature") {
                            *t = json!(max_temp);
                        }
                    }
                }
            }
        }
        
        request
    }
    
    fn translate_top_p(&self, mut request: Value, caps: &ModelCapabilities, warnings: &mut Vec<String>) -> Value {
        if !caps.supports_top_p {
            if request.get("top_p").is_some() {
                warnings.push("top_p not supported by target model, removing".to_string());
                if let Some(obj) = request.as_object_mut() {
                    obj.remove("top_p");
                }
            }
            return request;
        }
        
        if let Some(top_p) = request.get("top_p").and_then(|v| v.as_f64()) {
            if let Some(max_top_p) = caps.max_top_p {
                if top_p > max_top_p as f64 {
                    warnings.push(format!(
                        "top_p {} exceeds max {}, clamping to {}",
                        top_p, max_top_p, max_top_p
                    ));
                    if let Some(obj) = request.as_object_mut() {
                        if let Some(t) = obj.get_mut("top_p") {
                            *t = json!(max_top_p);
                        }
                    }
                }
            }
        }
        
        request
    }
    
    fn translate_top_k(&self, mut request: Value, caps: &ModelCapabilities, warnings: &mut Vec<String>) -> Value {
        if !caps.supports_top_k {
            if request.get("top_k").is_some() {
                warnings.push("top_k not supported by target model, removing".to_string());
                if let Some(obj) = request.as_object_mut() {
                    obj.remove("top_k");
                }
            }
            return request;
        }
        
        if let Some(top_k) = request.get("top_k").and_then(|v| v.as_u64()) {
            if let Some(max_top_k) = caps.max_top_k {
                if top_k > max_top_k as u64 {
                    warnings.push(format!(
                        "top_k {} exceeds max {}, clamping to {}",
                        top_k, max_top_k, max_top_k
                    ));
                    if let Some(obj) = request.as_object_mut() {
                        if let Some(t) = obj.get_mut("top_k") {
                            *t = json!(max_top_k);
                        }
                    }
                }
            }
        }
        
        request
    }
    
    fn translate_max_tokens(&self, mut request: Value, caps: &ModelCapabilities, warnings: &mut Vec<String>) -> Value {
        if let Some(limit) = caps.max_tokens_limit {
            if let Some(max_tokens) = request.get("max_tokens").and_then(|v| v.as_u64()) {
                if max_tokens > limit as u64 {
                    warnings.push(format!(
                        "max_tokens {} exceeds limit {}, clamping to {}",
                        max_tokens, limit, limit
                    ));
                    if let Some(obj) = request.as_object_mut() {
                        if let Some(t) = obj.get_mut("max_tokens") {
                            *t = json!(limit);
                        }
                    }
                }
            }
        }
        
        request
    }
    
    fn translate_functions(
        &self,
        mut request: Value,
        caps: &ModelCapabilities,
        errors: &mut Vec<String>,
        warnings: &mut Vec<String>,
    ) -> Value {
        let has_functions = request.get("functions").is_some();
        let has_tools = request.get("tools").is_some();
        
        if !caps.supports_function_calling && (has_functions || has_tools) {
            errors.push(format!(
                "Function calling not supported by target model. \
                Consider using a model with function calling support, \
                or remove function calls from your request."
            ));
            
            warnings.push("Removing unsupported function/tool calls".to_string());
            if let Some(obj) = request.as_object_mut() {
                obj.remove("functions");
                obj.remove("function_call");
                obj.remove("tools");
                obj.remove("tool_choice");
            }
        }
        
        request
    }
    
    fn translate_parameter_aliases(&self, mut request: Value, caps: &ModelCapabilities) -> Value {
        if caps.parameter_aliases.is_empty() {
            return request;
        }
        
        let mut reverse_aliases = HashMap::new();
        for (from, to) in &caps.parameter_aliases {
            reverse_aliases.insert(to.clone(), from.clone());
        }
        
        if let Some(obj) = request.as_object_mut() {
            for (alias, original) in &reverse_aliases {
                if let Some(value) = obj.remove(alias) {
                    obj.insert(original.clone(), value);
                }
            }
        }
        
        request
    }
}

impl Default for ParameterTranslator {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct TranslateResult<T> {
    pub request: T,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

impl TranslateResult<Value> {
    pub fn log_warnings(&self) {
        for warning in &self.warnings {
            tracing::warn!("Parameter translation: {}", warning);
        }
    }
    
    pub fn log_errors(&self) {
        for error in &self.errors {
            tracing::error!("Parameter translation: {}", error);
        }
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

pub fn translate_request_for_model(
    request: Value,
    target_caps: &ModelCapabilities,
) -> TranslateResult<Value> {
    let translator = ParameterTranslator::with_target_capabilities(target_caps.clone());
    let result = translator.translate_request(request);
    
    if !result.warnings.is_empty() {
        result.log_warnings();
    }
    if !result.errors.is_empty() {
        result.log_errors();
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_translate_temperature_clamp() {
        let caps = ModelCapabilities {
            max_temperature: Some(1.0),
            ..Default::default()
        };
        
        let request = json!({
            "model": "test",
            "temperature": 1.5,
            "messages": []
        });
        
        let translator = ParameterTranslator::with_target_capabilities(caps);
        let result = translator.translate_request(request);
        
        assert!(!result.errors.is_empty() || result.warnings.iter().any(|w| w.contains("exceeds max")));
        assert!(result.request["temperature"].as_f64().unwrap() <= 1.0);
    }
    
    #[test]
    fn test_remove_unsupported_top_k() {
        let caps = ModelCapabilities {
            supports_top_k: false,
            ..Default::default()
        };
        
        let request = json!({
            "model": "test",
            "top_k": 50,
            "messages": []
        });
        
        let translator = ParameterTranslator::with_target_capabilities(caps);
        let result = translator.translate_request(request);
        
        assert!(result.request.get("top_k").is_none());
        assert!(result.warnings.iter().any(|w| w.contains("top_k")));
    }
    
    #[test]
    fn test_remove_unsupported_top_p() {
        let caps = ModelCapabilities {
            supports_top_p: false,
            ..Default::default()
        };
        
        let request = json!({
            "model": "test",
            "top_p": 0.9,
            "messages": []
        });
        
        let translator = ParameterTranslator::with_target_capabilities(caps);
        let result = translator.translate_request(request);
        
        assert!(result.request.get("top_p").is_none());
    }
    
    #[test]
    fn test_function_calling_not_supported() {
        let caps = ModelCapabilities {
            supports_function_calling: false,
            ..Default::default()
        };
        
        let request = json!({
            "model": "test",
            "messages": [],
            "functions": [{
                "name": "get_weather",
                "parameters": {}
            }]
        });
        
        let translator = ParameterTranslator::with_target_capabilities(caps);
        let result = translator.translate_request(request);
        
        assert!(result.request.get("functions").is_none());
        assert!(result.errors.iter().any(|e| e.contains("Function calling not supported")));
    }
    
    #[test]
    fn test_parameter_alias_translation() {
        let mut aliases = HashMap::new();
        aliases.insert("max_tokens".to_string(), "maxTokens".to_string());
        
        let caps = ModelCapabilities {
            parameter_aliases: aliases,
            ..Default::default()
        };
        
        let request = json!({
            "model": "test",
            "messages": [],
            "maxTokens": 1000
        });
        
        let translator = ParameterTranslator::with_target_capabilities(caps);
        let result = translator.translate_request(request);
        
        assert!(result.request.get("maxTokens").is_none());
        assert!(result.request.get("max_tokens").is_some());
    }
    
    #[test]
    fn test_max_tokens_limit() {
        let caps = ModelCapabilities {
            max_tokens_limit: Some(1000),
            ..Default::default()
        };
        
        let request = json!({
            "model": "test",
            "messages": [],
            "max_tokens": 2000
        });
        
        let translator = ParameterTranslator::with_target_capabilities(caps);
        let result = translator.translate_request(request);
        
        assert!(result.request["max_tokens"].as_u64().unwrap() <= 1000);
    }
    
    #[test]
    fn test_no_changes_needed() {
        let caps = ModelCapabilities::default();
        
        let request = json!({
            "model": "test",
            "messages": [],
            "temperature": 0.7,
            "max_tokens": 1000
        });
        
        let translator = ParameterTranslator::with_target_capabilities(caps);
        let result = translator.translate_request(request);
        
        assert!(result.warnings.is_empty());
        assert!(result.errors.is_empty());
        assert_eq!(result.request["temperature"], 0.7);
    }
}
