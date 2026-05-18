# ModelLink - GitHub Issues TODO List

---

## 📋 Project Board Structure

| Column | Description |
|--------|-------------|
| **Backlog** | To be planned |
| **To Do** | Ready to start |
| **In Progress** | Currently working on |
| **Review** | Under review |
| **Done** | Completed |

---

## 🏷️ Label Definitions

| Label | Color | Description |
|-------|-------|-------------|
| `priority/P0` | 🔴 #ff0000 | Immediate action |
| `priority/P1` | 🟠 #ff9900 | High priority |
| `priority/P2` | 🟡 #ffff00 | Medium priority |
| `priority/P3` | 🟢 #00ff00 | Low priority |
| `type/feature` | 🟣 #9900ff | New feature |
| `type/infrastructure` | 🔵 #0099ff | Infrastructure |
| `type/docs` | 📄 #00ffff | Documentation |
| `type/security` | 🛡️ #ff00ff | Security |
| `stage/design` | 🎨 #ffcc00 | Design stage |
| `stage/development` | 👷 #33cc00 | Development stage |
| `stage/testing` | ✅ #00cc99 | Testing stage |
| `stage/review` | 🔍 #0066cc | Review stage |

---

## ✅ P0 - Immediate Action (Weeks 1-2)

### Issue #1: Complete streaming conversion PoC verification
**Labels**: `priority/P0`, `type/feature`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-05-30
**Status**: ✅ Done

**Description**:
- Implement minimal working proof-of-concept for OpenAI ↔ Anthropic streaming protocol conversion
- Cover boundary test cases:
  - Normal stream test
  - Error stream test
  - Connection interruption recovery test
  - Concurrent stream test
  - Special character test
  - Extra long response test

**Acceptance Criteria**:
- All 6 boundary test cases pass
- Generate working PoC code and test cases

---

### Issue #2: Define config file schema
**Labels**: `priority/P0`, `type/infrastructure`, `stage/design`
**Assignee**: Architect
**Due Date**: 2026-05-30
**Status**: ✅ Done

**Description**:
- Use JSON Schema to define config file structure
- Provide `model-link config init` command to generate annotated config template

**Acceptance Criteria**:
- Config file passes JSON Schema validation
- Support environment variable reading (e.g., `${DEEPSEEK_KEY}`)

---

### Issue #3: Design error code system
**Labels**: `priority/P0`, `type/infrastructure`, `stage/design`
**Assignee**: Tech Lead
**Due Date**: 2026-05-30
**Status**: ✅ Done

**Description**:
- Establish unified error code classification and message specification
- Support tiered error messaging (CLI concise + log detailed)

**Acceptance Criteria**:
- All errors have unique error codes
- Error messages are clear and developer-friendly

---

### Issue #4: Create project repository, initialize Rust project structure
**Labels**: `priority/P0`, `type/infrastructure`, `stage/development`
**Assignee**: Tech Lead
**Due Date**: 2026-05-26
**Status**: ✅ Done

**Description**:
- Create GitHub repository
- Initialize Rust project structure
- Configure CI/CD pipeline

**Acceptance Criteria**:
- Clear project structure
- CI runs normally

---

## 🟠 P1 - High Priority (Weeks 3-6)

### Issue #5: Implement config file hot reloading
**Labels**: `priority/P1`, `type/feature`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-06-13
**Status**: ✅ Done

**Description**:
- Use `notify` crate for file change monitoring
- Config changes apply within 2 seconds without interrupting existing connections

**Acceptance Criteria**:
- Config changes take effect within 2 seconds
- Existing connections not interrupted
- Log output "Config updated"

---

### Issue #6: Implement HTTP request forwarding engine core
**Labels**: `priority/P1`, `type/feature`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-06-13
**Status**: ✅ Done

**Description**:
- Implement HTTP request forwarding logic
- Support OpenAI `/v1/chat/completions` endpoint

**Acceptance Criteria**:
- Support non-streaming request forwarding
- Transparent forwarding without modifying request/response content

---

### Issue #7: Implement SSE streaming response handling
**Labels**: `priority/P1`, `type/feature`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-06-13
**Status**: ✅ Done

**Description**:
- Implement SSE streaming response forwarding
- Support OpenAI streaming format

**Acceptance Criteria**:
- P99 latency < 50ms
- Streams not interleaved or lost

---

### Issue #8: Establish model capability database
**Labels**: `priority/P1`, `type/infrastructure`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-06-20
**Status**: ✅ Done

**Description**:
- Define model capability data structure
- Pre-populate capability declarations for major models (OpenAI, Anthropic, DeepSeek, Qwen)

**Acceptance Criteria**:
- Cover 5+ major models
- Support parameter support detection

---

### Issue #9: Implement parameter translation engine
**Labels**: `priority/P1`, `type/feature`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-06-20
**Status**: ✅ Done

**Description**:
- Implement automatic generic parameter mapping (temperature, max_tokens, etc.)
- Automatically strip unsupported parameters with warnings

**Acceptance Criteria**:
- Support temperature parameter range adjustment
- Unsupported parameters automatically stripped and logged

---

### Issue #10: Implement health check endpoint `/health`
**Labels**: `priority/P1`, `type/infrastructure`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-06-27
**Status**: ✅ Done

**Description**:
- Implement `/health` health check endpoint
- Return service status, version, config loaded status

**Acceptance Criteria**:
- Support Kubernetes/Docker health checks
- Return JSON format status information

---

### Issue #11: Implement audit logging system
**Labels**: `priority/P1`, `type/security`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-06-27
**Status**: ✅ Done

**Description**:
- Implement structured audit logging
- Support content masking (e.g., API keys)

**Acceptance Criteria**:
- Log request time, client IP, target model, response status
- Sensitive data in request content automatically masked

---

### Issue #12: Design and select open source license
**Labels**: `priority/P1`, `type/docs`, `stage/design`
**Assignee**: Product Owner
**Due Date**: 2026-05-30
**Status**: ✅ Done

**Description**:
- Evaluate MIT/Apache 2.0/GPL and other licenses
- Select appropriate license and add LICENSE file

**Acceptance Criteria**:
- License type determined
- LICENSE file added to repository root

---

## 🟡 P2 - Medium Priority (Weeks 7-10)

### Issue #13: Implement interactive config wizard
**Labels**: `priority/P2`, `type/feature`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-07-11
**Status**: ✅ Done

**Description**:
- Implement `model-link config init` interactive command
- Guide user through initial configuration

**Acceptance Criteria**:
- User completes configuration within 5 minutes
- Generate annotated config file

---

### Issue #14: Implement Prometheus Metrics endpoint
**Labels**: `priority/P2`, `type/infrastructure`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-07-11
**Status**: ✅ Done

**Description**:
- Implement `/metrics` endpoint
- Expose request count, latency, active connections and other metrics

**Acceptance Criteria**:
- Support Prometheus scraping
- Include request total, latency histogram, active stream count

---

### Issue #15: Implement failover mechanism
**Labels**: `priority/P2`, `type/feature`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-07-18
**Status**: ✅ Done

**Description**:
- Implement model health checks
- Automatically switch to backup provider when upstream unavailable

**Acceptance Criteria**:
- Support multiple model instance configuration
- Automatically detect upstream health status and switch

---

### Issue #16: Implement config version migration mechanism
**Labels**: `priority/P2`, `type/infrastructure`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-07-18
**Status**: ✅ Done

**Description**:
- Implement config file version declaration
- Support automatic migration and rollback

**Acceptance Criteria**:
- Config file includes version number
- Version upgrade automatically migrates, rollback on failure

---

### Issue #17: Implement config automatic backup
**Labels**: `priority/P2`, `type/feature`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-07-25
**Status**: ✅ Done

**Description**:
- Implement config automatic backup
- Support manual backup and restore

**Acceptance Criteria**:
- Config changes trigger automatic backup
- Keep last 10 backups

---

### Issue #18: Implement Mock/offline mode
**Labels**: `priority/P2`, `type/feature`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-08-01
**Status**: ✅ Done

**Description**:
- Implement Mock response mode
- Support recording and playback of real responses

**Acceptance Criteria**:
- Support mock, record, replay three modes
- Configurable delay and response content

---

## 🟢 P3 - Low Priority (Weeks 11-14)

### Issue #19: Implement Shell completion support
**Labels**: `priority/P3`, `type/feature`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-08-15
**Status**: ✅ Done

**Description**:
- Support Bash, Zsh, Fish, PowerShell completion
- Provide installation command

**Acceptance Criteria**:
- Support 4 major shells
- Provide `model-link completion install` command

---

### Issue #20: Implement OpenTelemetry tracing
**Labels**: `priority/P3`, `type/infrastructure`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-08-22
**Status**: ⏸️ Deferred (Optional)

**Description**:
- Integrate OpenTelemetry
- Support distributed tracing

**Acceptance Criteria**:
- Generate trace spans
- Support Jaeger/Zipkin export

---

### Issue #21: Publish Rust SDK
**Labels**: `priority/P3`, `type/feature`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-08-29
**Status**: 📋 Backlog

**Description**:
- Create Rust SDK crate
- Publish to crates.io

**Acceptance Criteria**:
- SDK supports chat/chat_stream methods
- Published to crates.io

---

### Issue #22: Implement multi-instance load balancing
**Labels**: `priority/P3`, `type/feature`, `stage/development`
**Assignee**: Backend Developer
**Due Date**: 2026-09-05
**Status**: 📋 Backlog

**Description**:
- Support multiple instances of same model
- Implement round-robin/weighted load balancing

**Acceptance Criteria**:
- Support multiple upstream instance configuration
- Support weight configuration and round-robin strategy

---

## 📊 Milestone Planning

| Milestone | Target Version | Due Date | Included Issues |
|-----------|----------------|----------|-----------------|
| **MVP Foundation** | v0.1.0 | 2026-06-13 | #1, #2, #3, #4, #5, #6, #7 |
| **Anthropic Support** | v0.2.0 | 2026-07-04 | #8, #9, #10, #11 |
| **Robustness Improvements** | v0.3.0 | 2026-07-25 | #13, #14, #15, #16, #17 |
| **Desktop Tray** | v1.0.0 | 2026-09-05 | #18, #19, #20, #21, #22 |

---

## 📌 Import Notes

### Method 1: Using GitHub CLI

```bash
# Bulk create issues (requires GitHub CLI installed)
gh issue create --title "Complete streaming conversion PoC verification" \
  --body "$(cat issue-1-body.md)" \
  --label "priority/P0" \
  --label "type/feature" \
  --assignee "backend-dev" \
  --milestone "v0.1.0"
```

### Method 2: Using GitHub Importer

Save the following as CSV file for import:

```csv
Title,Labels,Assignees,Due Date,Description
Complete streaming conversion PoC verification,"priority/P0,type/feature",backend-dev,2026-05-30,"Implement OpenAI ↔ Anthropic streaming protocol conversion..."
Define config file schema,"priority/P0,type/infrastructure",architect,2026-05-30,"Use JSON Schema to define config file structure..."
```

---

**File Version**: v1.0
**Created**: 2026-05-19
**Related Document**: [Plan.md](file:///d:/WSL-Windows.Projects/ModelLink/Plan.md)
