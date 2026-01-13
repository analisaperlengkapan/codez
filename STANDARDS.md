# Codeza Development Standards & Guidelines

## Code Style Guide

### Rust Formatting
```bash
# Format all code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check
```

### Naming Conventions
```rust
// Types: PascalCase
pub struct Repository { }
pub enum IssueState { }

// Functions/Variables: snake_case
pub async fn create_repository() { }
let repo_name = "my-repo";

// Constants: SCREAMING_SNAKE_CASE
const MAX_REPO_NAME_LENGTH: usize = 100;

// Lifetimes: single lowercase letters
fn process<'a>(s: &'a str) { }

// Type parameters: PascalCase
struct Container<T> { }
```

### File Organization
```
src/
├── main.rs          # Entry point, module declarations
├── router.rs        # Route definitions and AppState
├── error.rs         # Error types and handling
├── validation.rs    # Input validators
├── handlers/
│   ├── mod.rs       # Handler module exports
│   ├── repo.rs      # Repository handlers
│   ├── user.rs      # User/auth handlers
│   ├── admin.rs     # Admin handlers
│   ├── project.rs   # Project handlers
│   ├── package.rs   # Package handlers
│   └── action.rs    # CI/Action handlers
└── tests.rs         # Integration tests
```

## API Design Standards

### Endpoint Naming
```
GET    /api/v1/repos                    # List all
GET    /api/v1/repos/:id                # Get single
POST   /api/v1/repos                    # Create
PATCH  /api/v1/repos/:id                # Update
DELETE /api/v1/repos/:id                # Delete

GET    /api/v1/repos/:id/issues         # List related
POST   /api/v1/repos/:id/issues         # Create related
```

### Request/Response Format
```json
{
  "id": 1,
  "name": "example",
  "created_at": "2024-01-13T10:00:00Z",
  "description": "Optional field"
}
```

### Error Response Format
```json
{
  "code": "NOT_FOUND",
  "message": "Repository 'example' not found"
}
```

### Status Codes
| Code | Meaning | Usage |
|------|---------|-------|
| 200 | OK | GET, safe POST |
| 201 | Created | POST creating new resources |
| 204 | No Content | PATCH, PUT, DELETE |
| 400 | Bad Request | Invalid input |
| 401 | Unauthorized | Missing/invalid auth |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource doesn't exist |
| 409 | Conflict | Duplicate/state conflict |
| 500 | Server Error | Internal error |

## Handler Implementation Pattern

### Standard Handler Structure
```rust
pub async fn create_repo(
    State(state): State<AppState>,
    Json(payload): Json<CreateRepoOption>,
) -> Result<(StatusCode, Json<Repository>), AppError> {
    // 1. Validate input
    validate_repo_name(&payload.name)?;
    
    // 2. Check preconditions
    let repos = state.repos.read()
        .map_err(|_| AppError::InternalError("lock poisoned".into()))?;
    
    if repos.iter().any(|r| r.name == payload.name) {
        return Err(AppError::Conflict(format!(
            "Repository '{}' already exists",
            payload.name
        )));
    }
    
    // 3. Perform operation
    let mut repos = state.repos.write()
        .map_err(|_| AppError::InternalError("lock poisoned".into()))?;
    
    let repo = Repository { /* ... */ };
    repos.push(repo.clone());
    
    // 4. Record activity/audit
    state.activities.write()
        .map_err(|_| AppError::InternalError("lock poisoned".into()))?
        .push(Activity { /* ... */ });
    
    // 5. Return result
    Ok((StatusCode::CREATED, Json(repo)))
}
```

## Type Definitions Standards

### Struct Design
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Repository {
    // Required fields first
    pub id: u64,
    pub name: String,
    pub owner: String,
    
    // Optional fields with descriptive names
    pub description: Option<String>,
    
    // Boolean flags grouped
    pub private: bool,
    pub is_mirror: bool,
    pub is_archived: bool,
    
    // Counts at end
    pub stars_count: u64,
    pub forks_count: u64,
}
```

### Create/Update Options
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRepoOption {
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub private: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRepoOption {
    pub description: Option<String>,
    pub private: Option<bool>,
}
```

## Testing Standards

### Test Organization
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    // Unit tests for validators
    #[test]
    fn test_valid_username() {
        assert!(Validator::validate_username("john_doe").is_ok());
    }
    
    // Integration tests with state
    #[tokio::test]
    async fn test_create_repo_flow() {
        let state = setup_test_state();
        // Test here
    }
}
```

### Test Naming
```rust
// test_<function>_<scenario>_<expected_result>
#[test]
fn test_validate_username_invalid_format_returns_error() { }

#[tokio::test]
async fn test_create_repo_duplicate_returns_conflict() { }
```

## Documentation Standards

### Module Documentation
```rust
//! This module handles repository operations.
//!
//! It provides handlers for:
//! - Creating repositories
//! - Listing repositories
//! - Updating repository settings
//!
//! # Examples
//!
//! ```no_run
//! let repo = create_repo(state, payload).await?;
//! ```
```

### Function Documentation
```rust
/// Creates a new repository.
///
/// # Arguments
/// * `state` - Application state containing repositories
/// * `payload` - Repository creation options
///
/// # Returns
/// * `Ok((StatusCode::CREATED, repo))` - Successfully created
/// * `Err(AppError)` - Conflict if already exists
///
/// # Errors
/// Returns `AppError::Conflict` if repository name already exists.
pub async fn create_repo(
    State(state): State<AppState>,
    Json(payload): Json<CreateRepoOption>,
) -> Result<(StatusCode, Json<Repository>), AppError> {
    // ...
}
```

## Error Handling Standards

### Use Result Types
```rust
// ❌ Avoid
let repos = state.repos.read().unwrap();

// ✅ Prefer
let repos = state.repos.read()
    .map_err(|_| AppError::InternalError("Failed to acquire lock".into()))?;
```

### Descriptive Error Messages
```rust
// ❌ Poor
Err(AppError::NotFound("not found".into()))

// ✅ Good
Err(AppError::NotFound(format!(
    "Repository '{}' not found in owner '{}'",
    repo_name, owner
)))
```

### Error Context
```rust
pub async fn get_repo(
    State(state): State<AppState>,
    Path((owner, repo_name)): Path<(String, String)>,
) -> Result<Json<Repository>, AppError> {
    let repos = state.repos.read()
        .map_err(|e| AppError::InternalError(format!(
            "Failed to acquire repos lock: {}",
            e
        )))?;
    
    repos.iter()
        .find(|r| r.owner == owner && r.name == repo_name)
        .cloned()
        .map(Json)
        .ok_or_else(|| AppError::NotFound(format!(
            "Repository '{}/{}' not found",
            owner, repo_name
        )))
}
```

## Performance Guidelines

### Memory Efficiency
```rust
// ❌ Cloning unnecessarily
let repo = state.repos.read().unwrap()
    .iter()
    .find(|r| r.id == id)
    .cloned();

// ✅ Use references when possible  
let repo = state.repos.read().unwrap()
    .iter()
    .find(|r| r.id == id)
    .map(|r| r.clone());  // Only clone when needed
```

### Async/Await
```rust
// ✅ Don't block in async
pub async fn handler() {
    // Avoid: std::thread::sleep();
    // Use: tokio::time::sleep().await
}

// ✅ Use join for parallel operations
let results = futures::future::join_all(vec![
    async_op_1(),
    async_op_2(),
]).await;
```

## Dependency Management

### Workspace Dependencies
```toml
[workspace]
members = ["crates/backend", "crates/frontend", "crates/shared"]
resolver = "2"

# Keep shared dependencies consistent across workspace
```

### Version Pinning
```toml
# Lock major versions in production
axum = "0.7"        # Not "~0.7" or "*"
tokio = "1.0"

# Allow minor/patch for bug fixes
serde = { version = "1.0", features = ["derive"] }
```

## Commit Message Standards

### Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `refactor`: Code refactoring
- `perf`: Performance improvement
- `test`: Adding tests
- `docs`: Documentation
- `chore`: Build/dependency updates
- `style`: Code style changes

### Examples
```
feat(repos): add repository search functionality

Implement full-text search for repositories by name and description.
Adds new GET /api/v1/repos/search endpoint with query parameter support.

Closes #123

fix(handlers): remove unwrap() calls in repo handler

Replace unwrap() with proper error handling using Result types.
Improves error messages for debugging.

refactor(state): use proper ID generation with uuid

Replace manual ID incrementing with uuid v4 generation.
Prevents ID collisions in distributed systems.
```

## Review Checklist

Before submitting code:
- [ ] Runs `cargo fmt`
- [ ] Runs `cargo clippy` with no warnings
- [ ] All tests pass (`cargo test`)
- [ ] API tests pass (`bash test_api.sh`)
- [ ] Code follows naming conventions
- [ ] Error handling is appropriate
- [ ] Functions are documented
- [ ] No `unwrap()` calls except in tests
- [ ] No hardcoded values (use constants)
- [ ] No secrets in code
- [ ] Performance reasonable

## Linting & Formatting

### Setup
```bash
# Install tools
rustup component add rustfmt clippy

# Format code
cargo fmt --all

# Check formatting
cargo fmt --all -- --check

# Lint code
cargo clippy --all -- -D warnings

# Run tests
cargo test --all
```

### CI/CD Integration
```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo fmt --all -- --check
      - run: cargo clippy --all -- -D warnings
      - run: cargo test --all
      - run: bash test_api.sh
```

## Conclusion

These standards ensure:
- ✅ Consistent code quality
- ✅ Better maintainability
- ✅ Fewer bugs through proper error handling
- ✅ Performance and safety
- ✅ Clear documentation
- ✅ Smooth collaboration
