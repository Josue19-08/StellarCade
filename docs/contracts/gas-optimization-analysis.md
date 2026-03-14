# gas-optimization-analysis

## Public Methods

### `init`
```rust
pub fn init(env: Env, admin: Address) -> Result<(), Error>
```

### `record_sample`
```rust
pub fn record_sample( env: Env, admin: Address, method: Symbol, cpu: u64, read_bytes: u64, write_bytes: u64, ) -> Result<MethodProfile, Error>
```

### `get_method_profile`
```rust
pub fn get_method_profile(env: Env, method: Symbol) -> MethodProfile
```

### `get_hotspots`
```rust
pub fn get_hotspots(env: Env, limit: u32) -> Vec<MethodHotspot>
```

### `get_recommendations`
```rust
pub fn get_recommendations(env: Env, limit: u32) -> Vec<OptimizationRecommendation>
```

