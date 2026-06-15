# BUILD.md - Building Augmented-Citizen

## Prerequisites

- Rust compiler version 1.85 or later with edition 2024 support
- Git CLI for repository management
- No external tool installation required beyond standard Rust toolchain

## Building the Workspace

### Full Workspace Build

```bash
cd /workspace
cargo build --workspace
```

### Build Specific Crates

```bash
# Build sovereign guards core
cargo build -p sovereign-guards-core

# Build ALN core
cargo build -p ac-aln-core

# Build neuro-content-safety
cargo build -p neuro-content-safety
```

### Release Build

```bash
cargo build --workspace --release
```

## Running Tests

### All Tests

```bash
cargo test --workspace
```

### Test with Output

```bash
cargo test --workspace -- --nocapture
```

### Specific Crate Tests

```bash
cargo test -p sovereign-guards-core
cargo test -p augmented-id-guard
cargo test -p econet-healthcare-guard
```

## Verification

### Check Without Building

```bash
cargo check --workspace
```

### Clippy Linting

```bash
cargo clippy --workspace -- -D warnings
```

### Format Check

```bash
cargo fmt --check
```

## Documentation

### Generate Documentation

```bash
cargo doc --workspace --no-deps
```

### Open Documentation

```bash
cargo doc --workspace --no-deps --open
```

## ALN Schema Validation

The ALN (Augmented Ledger Network) schemas in `aln/` directories should be validated against their respective schemas:

```bash
# Validate core spec ALN files
for file in aln-core-spec/*.aln; do
    echo "Validating $file"
    # Add validation logic as needed
done
```

## Integration Points

### Cybercore Binding

Ensure `mk-bluebird/Cybercore` is accessible if building integration components:

```bash
# Verify Cybercore binding in manifests
grep -r "CYBERCORE_AUTHORITY" crates/*/src/*.rs
```

### Identity Binding Verification

Verify that crates properly bind to host identities:

```bash
grep -r "host_did\|bostrom_address" crates/*/src/*.rs
```

## Troubleshooting

### Edition 2024 Errors

If you encounter edition-related errors, ensure your Rust version is 1.85+:

```bash
rustc --version
```

Update if necessary:

```bash
rustup update
```

### Missing Crate Dependencies

If a crate is missing from the workspace:

1. Check `Cargo.toml` workspace members
2. Verify the crate directory exists
3. Ensure the crate has a valid `Cargo.toml`

### ALN Schema Issues

If ALN files fail validation:

1. Check syntax against `aln-audit-ledger/audit.schema.events.aln`
2. Verify compliance clauses match expected format
3. Ensure all required fields are present

## Security Considerations

When building for production:

1. Always use release builds for deployment
2. Verify cryptographic signatures on OTA payloads
3. Audit dependency tree for vulnerabilities
4. Enable all security lints

## Compliance Verification

Before deployment, verify:

- [ ] All crates declare ALN compliance clauses
- [ ] Identity bindings are properly configured
- [ ] Consent gating is implemented for neural operations
- [ ] Audit trails are enabled for sensitive operations
- [ ] Non-reversal invariants are enforced

## References

- See `docs/REQUIREMENTS.md` for core requirements
- See `docs/NEURO_RIGHTS_CHARTER.md` for rights framework
- See `README.md` for overall project guidance
