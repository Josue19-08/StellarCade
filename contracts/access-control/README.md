# Stellarcade Access Control

A reusable Role-Based Access Control (RBAC) contract and shared library for the Stellarcade platform.

## 📂 Features

- **Standard Roles**: `ADMIN`, `OPERATOR`, `PAUSER`, `GAME`.
- **Initialization**: Single super-admin setup.
- **Role Management**: Grant and revoke roles (restricted to ADMIN).
- **Guard Helpers**: Easily integrate role checks into other contracts.
- **Events**: Emits `role_gr` (grant) and `role_rv` (revoke) events.

## 🏗 Usage

### For Contract Deployment

1. **Build**: `cargo build --target wasm32-unknown-unknown --release`
2. **Deploy**: Use Soroban CLI to deploy the `.wasm` file.
3. **Initialize**: Call `init(super_admin_address)`.

### For Dependent Contracts (Library Approach)

Other contracts in the Stellarcade ecosystem can use this crate to manage their own roles or interact with a central Access Control contract.

#### 1. Add dependency

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
stellarcade-access-control = { path = "../access-control" }
```

#### 2. Use guard helpers in your contract

```rust
use stellarcade_access_control::{require_role, GAME};

#[contractimpl]
impl MyGameContract {
    pub fn start_game(env: Env, player: Address) {
        // Only accounts with the GAME role can trigger this
        require_role(&env, GAME, player);
        
        // ... game logic
    }
}
```

## 🔒 Security

- **Privilege Escalation**: Only the `ADMIN` can grant or revoke roles.
- **Validation**: Authorization is strictly checked using `account.require_auth()`.
- **Deterministic**: Duplicate grants or revokes are handled gracefully without redundant events.

## 🧪 Testing

Run tests locally:

```bash
cargo test
```
