# ModelLink API Documentation

## Overview

ModelLink is a local proxy that allows AI coding tools to transparently use any third-party model. This document describes the REST API endpoints provided by ModelLink.

## Base URL

All endpoints are relative to the base URL configured in the server settings. By default:
```
http://localhost:9191
```

## Endpoints

### 1. Chat Completions (OpenAI Compatible)

**POST** `/v1/chat/completions`

Create a chat completion request.

#### Request Body

```json
{
  "model": "gpt-4",
  "messages": [
    {
      "role": "system",
      "content": "You are a helpful assistant."
    },
    {
      "role": "user",
      "content": "Hello, how are you?"
    }
  ],
  "temperature": 0.7,
  "max_tokens": 1000,
  "stream": false,
  "top_p": 1.0,
  "frequency_penalty": 0.0,
  "presence_penalty": 0.0
}
```

#### Parameters

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| model | string | Yes | The model name to use |
| messages | array | Yes | Array of message objects |
| temperature | number | No | Controls randomness (0-2) |
| max_tokens | integer | No | Maximum tokens to generate |
| stream | boolean | No | Enable streaming response |
| top_p | number | No | Nucleus sampling (0-1) |
| frequency_penalty | number | No | Reduces repetition (-2 to 2) |
| presence_penalty | number | No | Encourages new topics (-2 to 2) |

#### Message Object

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| role | string | Yes | Message role: `user`, `assistant`, `system`, or `tool` |
| content | string | Yes* | Message content (required unless function_call/tool_calls present) |
| function_call | object | No | Function call specification |
| tool_calls | array | No | Array of tool call objects |

#### Response (Non-streaming)

```json
{
  "id": "chatcmpl-abc123",
  "object": "chat.completion",
  "created": 1677652288,
  "model": "gpt-4",
  "choices": [
    {
      "index": 0,
      "message": {
        "role": "assistant",
        "content": "I'm doing well, thank you! How can I assist you today?"
      },
      "finish_reason": "stop"
    }
  ],
  "usage": {
    "prompt_tokens": 20,
    "completion_tokens": 15,
    "total_tokens": 35
  }
}
```

#### Response (Streaming)

```json
{
  "id": "chatcmpl-abc123",
  "object": "chat.completion.chunk",
  "created": 1677652288,
  "model": "gpt-4",
  "choices": [
    {
      "index": 0,
      "delta": {
        "content": "Hello"
      },
      "finish_reason": null
    }
  ]
}
```

### 2. Anthropic Messages

**POST** `/v1/messages`

Create a message request in Anthropic format.

#### Request Body

```json
{
  "model": "claude-3-opus",
  "messages": [
    {
      "role": "user",
      "content": "Hello!"
    }
  ],
  "max_tokens": 1000,
  "temperature": 0.7
}
```

#### Response

```json
{
  "id": "msg_abc123",
  "type": "message",
  "role": "assistant",
  "content": [
    {
      "type": "text",
      "text": "Hello! How can I assist you today?"
    }
  ],
  "model": "claude-3-opus",
  "usage": {
    "input_tokens": 10,
    "output_tokens": 15
  }
}
```

### 3. Health Check

**GET** `/health`

Check if the service is healthy.

#### Response

```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T12:00:00Z",
  "version": "0.1.0"
}
```

### 4. Readiness Check

**GET** `/ready`

Check if the service is ready to accept requests.

#### Response

```json
{
  "status": "ready",
  "providers": 3,
  "mappings": 10
}
```

### 5. Metrics

**GET** `/metrics`

Get Prometheus metrics.

#### Response

```
# HELP model_link_requests_total Total number of requests
# TYPE model_link_requests_total counter
model_link_requests_total 100

# HELP model_link_errors_total Total number of errors
# TYPE model_link_errors_total counter
model_link_errors_total 5

# HELP model_link_tokens_total Total number of tokens processed
# TYPE model_link_tokens_total counter
model_link_tokens_total 5000

# HELP model_link_active_streams Number of active streaming connections
# TYPE model_link_active_streams gauge
model_link_active_streams 2

# HELP model_link_request_duration_seconds_avg Average request latency in seconds
# TYPE model_link_request_duration_seconds_avg gauge
model_link_request_duration_seconds_avg 0.5

# HELP model_link_up Up whether model-link is running
# TYPE model_link_up gauge
model_link_up 1
```

## Error Responses

All endpoints return standardized error responses:

```json
{
  "error": {
    "message": "Error description",
    "type": "error_type"
  }
}
```

### Error Types

| Error Type | HTTP Status | Description |
|------------|-------------|-------------|
| validation_error | 400 | Invalid request parameters |
| authentication_error | 401 | Invalid or missing API key |
| rate_limit_error | 429 | Rate limit exceeded |
| not_found | 404 | Resource not found |
| network_error | 502 | Upstream service unavailable |
| configuration_error | 500 | Server configuration error |
| internal_error | 500 | Unexpected server error |

## Headers

### Request Headers

| Header | Description |
|--------|-------------|
| Authorization | Bearer token for authentication |
| Content-Type | Must be `application/json` |
| X-Client-Id | Optional client identifier for rate limiting |

### Response Headers

| Header | Description |
|--------|-------------|
| X-Request-Id | Unique request identifier |
| X-RateLimit-Remaining | Remaining requests in current window |
| X-RateLimit-Reset | Seconds until rate limit resets |

## Rate Limiting

ModelLink implements rate limiting to prevent abuse:

- Default: 10 requests per second per client
- Burst limit: 50 requests
- Configurable via `rate_limit` section in config.yaml

## Example Usage

### cURL

```bash
curl http://localhost:9191/v1/chat/completions \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer your-api-key" \
  -d '{
    "model": "gpt-4",
    "messages": [{"role": "user", "content": "Hello"}]
  }'
```

### Python

```python
import requests

response = requests.post(
    "http://localhost:9191/v1/chat/completions",
    headers={"Authorization": "Bearer your-api-key"},
    json={
        "model": "gpt-4",
        "messages": [{"role": "user", "content": "Hello"}]
    }
)
print(response.json())
```

### JavaScript

```javascript
const response = await fetch("http://localhost:9191/v1/chat/completions", {
  method: "POST",
  headers: {
    "Content-Type": "application/json",
    "Authorization": "Bearer your-api-key"
  },
  body: JSON.stringify({
    model: "gpt-4",
    messages: [{ role: "user", content: "Hello" }]
  })
});
const data = await response.json();
console.log(data);
```

## Status Codes

| Status Code | Description |
|-------------|-------------|
| 200 OK | Success |
| 400 Bad Request | Invalid input |
| 401 Unauthorized | Missing/invalid authentication |
| 404 Not Found | Resource not found |
| 429 Too Many Requests | Rate limit exceeded |
| 500 Internal Server Error | Server error |
| 502 Bad Gateway | Upstream service error |