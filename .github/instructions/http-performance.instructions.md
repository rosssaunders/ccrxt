---
applyTo: "venues/src/**/rest/**"
---

# High-Performance HTTP Request Instructions

## Critical Performance Requirements

**MANDATORY**: All HTTP wrappers around REST API endpoints MUST follow high-performance patterns to avoid branch prediction penalties that can degrade performance in high-frequency trading scenarios.

## Core Rule: No HTTP Verbs as Parameters

**NEVER pass HTTP verbs as function parameters.** The caller already knows which HTTP verb will be used, so passing it as a parameter creates unnecessary runtime branching that hurts CPU performance through branch misprediction.

### ❌ Incorrect Pattern (Causes Performance Issues)

```rust
// BAD: Generic function that takes HTTP method as parameter
pub async fn send_request<T, P>(
    &self,
    endpoint: &str,
    method: reqwest::Method,  // ❌ This parameter causes branch misprediction
    params: Option<&P>,
    endpoint_type: EndpointType,
) -> RestResult<T>
where
    T: DeserializeOwned,
    P: Serialize,
{
    // Runtime branching on method type - PERFORMANCE PENALTY
    let (request_path, body) = if method == reqwest::Method::GET {
        // GET logic
    } else {
        // POST/PUT/DELETE logic  
    };
    // ... rest of implementation
}

// BAD: Endpoint calling generic function with method parameter
pub async fn create_order(&self, request: CreateOrderRequest) -> RestResult<CreateOrderResponse> {
    self.send_request(
        "orders",
        reqwest::Method::POST,  // ❌ Performance penalty
        Some(&request),
        EndpointType::Private,
    ).await
}
```

### ✅ Correct Pattern (High Performance)

```rust
// GOOD: Separate functions per HTTP verb - no runtime branching
pub async fn send_get_request<T, P>(
    &self,
    endpoint: &str,
    params: Option<&P>,
    endpoint_type: EndpointType,
) -> RestResult<T>
where
    T: DeserializeOwned,
    P: Serialize,
{
    // Specialized for GET requests only - no branching
    let query_string = if let Some(params) = params {
        serde_urlencoded::to_string(params)?
    } else {
        String::new()
    };
    // ... GET-specific implementation
}

pub async fn send_post_request<T, P>(
    &self,
    endpoint: &str,
    params: Option<&P>,
    endpoint_type: EndpointType,
) -> RestResult<T>
where
    T: DeserializeOwned,
    P: Serialize,
{
    // Specialized for POST requests only - no branching
    let body = if let Some(params) = params {
        serde_json::to_string(params)?
    } else {
        String::new()
    };
    // ... POST-specific implementation
}

// GOOD: Endpoint calling verb-specific function
pub async fn create_order(&self, request: CreateOrderRequest) -> RestResult<CreateOrderResponse> {
    self.send_post_request("orders", Some(&request), EndpointType::Private).await
}
```

## Required Implementation Pattern

### Mandatory HTTP Verb-Specific Functions

Every REST client MUST implement these verb-specific functions:

```rust
impl RestClient {
    /// Send GET request - optimized for query parameters
    pub async fn send_get_request<T, P>(&self, endpoint: &str, params: Option<&P>, endpoint_type: EndpointType) -> RestResult<T>;
    
    /// Send POST request - optimized for JSON body
    pub async fn send_post_request<T, P>(&self, endpoint: &str, params: Option<&P>, endpoint_type: EndpointType) -> RestResult<T>;
    
    /// Send PUT request - optimized for JSON body
    pub async fn send_put_request<T, P>(&self, endpoint: &str, params: Option<&P>, endpoint_type: EndpointType) -> RestResult<T>;
    
    /// Send DELETE request - optimized for query parameters or empty body
    pub async fn send_delete_request<T, P>(&self, endpoint: &str, params: Option<&P>, endpoint_type: EndpointType) -> RestResult<T>;
    
    /// Send PATCH request - optimized for JSON body (if needed by venue)
    pub async fn send_patch_request<T, P>(&self, endpoint: &str, params: Option<&P>, endpoint_type: EndpointType) -> RestResult<T>;
}
```

### Performance Optimizations Per Verb

Each verb-specific function MUST be optimized for its intended use:

- **GET requests**: Optimized for query parameter serialization
- **POST/PUT/PATCH requests**: Optimized for JSON body serialization
- **DELETE requests**: Optimized for either query parameters or empty body

### Endpoint Implementation Requirements

All endpoint wrapper functions MUST:

1. **Call verb-specific functions directly**
2. **NEVER pass HTTP method as parameter**
3. **Use the appropriate verb-specific function for the API endpoint**

Example endpoint implementations:

```rust
// GET endpoint
pub async fn get_account_info(&self, request: AccountInfoRequest) -> RestResult<AccountInfo> {
    self.send_get_request(ACCOUNT_INFO_ENDPOINT, Some(&request), EndpointType::Private).await
}

// POST endpoint  
pub async fn create_order(&self, request: CreateOrderRequest) -> RestResult<CreateOrderResponse> {
    self.send_post_request(ORDERS_ENDPOINT, Some(&request), EndpointType::Private).await
}

// DELETE endpoint
pub async fn cancel_order(&self, request: CancelOrderRequest) -> RestResult<CancelOrderResponse> {
    self.send_delete_request(ORDERS_ENDPOINT, Some(&request), EndpointType::Private).await
}
```

## Performance Reasoning

### Why This Matters for High-Frequency Trading

1. **Branch Prediction**: Modern CPUs use branch prediction to speculatively execute code. When a function branches on a runtime parameter (like HTTP method), the CPU cannot predict which branch will be taken, leading to pipeline stalls.

2. **Code Specialization**: Verb-specific functions allow for better compiler optimizations since each function is specialized for exactly one HTTP verb pattern.

3. **Cache Efficiency**: Separate functions reduce instruction cache pressure by avoiding unused code paths.

4. **Elimination of Runtime Checks**: No need to check `if method == GET` vs `if method == POST` at runtime.

### Benchmark Impact

In high-frequency trading scenarios where thousands of API calls per second are made, this optimization can provide:
- Reduced latency per request (microseconds matter)
- Better CPU utilization
- More predictable performance characteristics
- Reduced instruction cache misses

## Migration Guide

For existing implementations using generic `send_request()` functions:

1. **Create verb-specific functions** following the required pattern above
2. **Update all endpoint wrappers** to call appropriate verb-specific functions  
3. **Remove or deprecate** generic `send_request()` functions that take method parameters
4. **Test performance improvements** in high-load scenarios

## Enforcement

- All new REST client implementations MUST follow this pattern
- Existing implementations SHOULD be migrated to this pattern
- Code reviews MUST reject any HTTP method parameters in function signatures
- Linting rules MAY be added to automatically detect violations

## Exceptions

There are **NO exceptions** to this rule. All HTTP request functions must be verb-specific for optimal performance.