# Skynet Policy Reconciliation and Build Blockers v1

## Status

```text
source_generation = blocked
aln_generation = blocked
workspace_membership = blocked
dependency_selection = resolved
privacy_boundary = resolved
```

## Build Blockers

### B1: Missing CityPass manifest

The root workspace lists:

```text
crates/city-pass
```

but `crates/city-pass/Cargo.toml` is absent.

Resolution requires exactly one action:

```text
create crates/city-pass/Cargo.toml
remove crates/city-pass from workspace members
exclude crates/city-pass until implementation is complete
```

### B2: Missing sovereign-guards-core library root

`crates/sovereign-guards-core/Cargo.toml` declares a normal library package but
`crates/sovereign-guards-core/src/lib.rs` is absent.

Resolution requires exactly one action:

```text
create src/lib.rs with explicit module declarations
declare an alternate library path in Cargo.toml
exclude the crate from workspace members until it has a library target
```

### B3: Root workspace normalization

Before adding `crates/skynet`, the root workspace manifest requires:

```text
remove duplicate member paths
remove or relocate root package feature declarations
remove or relocate root target-specific dependency declarations
reconcile declared workspace license with repository license files
reconcile workspace Kani toolchain with Skynet verification requirement
```

## ALN Blocker

No canonical ALN grammar is proven.

The repository currently presents several declaration forms with incompatible
field layouts. No source has established:

```text
canonical parser crate
validator command
grammar version
schema import resolution
field-requiredness rules
cross-schema type checking
fixture-validation command
```

Therefore `crates/skynet/aln/` remains documentation-only until the ALN
toolchain is verified.

## Skynet Policy Projection

Skynet adopts only these policy semantics:

```text
deny by default
explicit purpose declaration
time-bounded authorization
holder-controlled revocation
no automatic renewal
no silent reduction of rights protection
no credential transport after policy failure
content-minimized audit records
```

Skynet excludes these policy inputs:

```text
neural data
neural summaries
brain-state commits
RoH values or bands
psychological state values
clinical context
device-control state
host direct identifier
Bostrom address
location data
network data
free-text reason
free-text notes
raw credential
credential claims
credential proof material
```

## Non-Reversal Projection

For Skynet, non-reversal means:

```text
a successor policy version cannot weaken required consent,
disclosure minimization, verifier authorization, status evaluation,
deployment authorization, audit minimization, or prohibited-data rules.
```

It does not mean that Skynet manages device capabilities, biological state,
clinical intervention, upgrade mechanisms, or emergency device control.

## Dependency Decision

```text
core dependencies permitted:
  serde
  serde_json
  thiserror

core dependencies prohibited:
  ac-aln-core
  augmented-id-guard
  city-pass
  sovereign-guards-core
  brain-identity-core
  device, network, wallet, ledger, transport, database, browser,
  credential-format, BCI, telemetry, and clinical crates
```

`serde_json` is fixture-only. No Skynet public contract may contain an
unbounded JSON value.
