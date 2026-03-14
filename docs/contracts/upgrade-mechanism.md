# upgrade-mechanism

## Public Methods

### `init`
```rust
pub fn init( env: Env, admin: Address, initial_wasm_hash: BytesN<32>, initial_schema_version: u32, ) -> Result<(), Error>
```

### `pause`
```rust
pub fn pause(env: Env, admin: Address) -> Result<(), Error>
```

### `unpause`
```rust
pub fn unpause(env: Env, admin: Address) -> Result<(), Error>
```

### `trigger_kill_switch`
```rust
pub fn trigger_kill_switch( env: Env, admin: Address, reason_hash: BytesN<32>, ) -> Result<(), Error>
```

### `configure_test_gate`
```rust
pub fn configure_test_gate( env: Env, admin: Address, gate_contract: Address, suite: Symbol, enabled: bool, ) -> Result<(), Error>
```

### `stage_upgrade`
```rust
pub fn stage_upgrade( env: Env, admin: Address, version: u32, wasm_hash: BytesN<32>, target_schema_version: u32, migration_hash: BytesN<32>, changelog_hash: BytesN<32>, ) -> Result<(), Error>
```

### `execute_upgrade`
```rust
pub fn execute_upgrade(env: Env, admin: Address) -> Result<(), Error>
```

### `rollback`
```rust
pub fn rollback(env: Env, admin: Address, reason_hash: BytesN<32>) -> Result<(), Error>
```

### `state`
```rust
pub fn state(env: Env) -> Result<UpgradeState, Error>
```

### `get_release`
```rust
pub fn get_release(env: Env, version: u32) -> Result<Option<ReleaseRecord>, Error>
```

### `set_ready`
```rust
pub fn set_ready(env: Env, ready: bool)
```

### `is_release_ready`
```rust
pub fn is_release_ready(env: Env, _suite: Symbol) -> bool
```

