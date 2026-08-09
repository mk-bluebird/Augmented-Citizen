# Research-Agent Decisions for the Skynet Readiness Audit

## 1. Implementation-Readiness Priority

Validate the current five-member workspace as the primary evidence target.

The agent must not narrow validation only to Skynet-enabling work, because
Skynet cannot safely join an invalid, unlicensed, or unverified workspace.
However, remediation recommendations must be ordered by Skynet impact.

Use this remediation order:

1. Workspace manifest validity and resolvable active members.
2. License completeness and manifest-license consistency.
3. Rust formatting, check, test, and lint results.
4. Kani toolchain reachability and proof-target discovery.
5. ALN grammar and validation-toolchain determination.
6. Skynet privacy-boundary verification for candidate dependencies.
7. Skynet crate integration readiness.

Report every finding under one of these labels:

```text
BLOCKS_CURRENT_WORKSPACE
BLOCKS_SKYNET_INTEGRATION
BLOCKS_BOTH
NON_BLOCKING
```

A workspace defect that prevents Cargo metadata, package resolution, compilation,
testing, or CI execution is a `BLOCKS_BOTH` finding.

## 2. Skynet Boundary Filtering and Redaction

Apply a deny-by-default data-class filter.

In addition to the explicitly prohibited Skynet data classes, treat the
following as sensitive, linkable, or out of scope for Skynet core contracts:

```text
stable pseudonymous identifier
cross-session correlation identifier
direct account identifier
issuer-controlled identifier
verifier-controlled identifier
credential proof material
public-key material
signature material
wallet handle
transport receipt containing payload data
endpoint URL
network address
device identifier
hardware serial
route identifier
geofence
time-series identifier
precise timestamp where it enables tracking
free-text reason
free-text note
exception message containing source values
unbounded JSON value
serialized map with unrestricted keys
serialized list with unrestricted values
clinical classification
diagnostic inference
risk score
brain-state summary
affective-state summary
cognitive inference
biophysical measurement
derived physiological metric
device-control state
security token
credential status payload
audit-chain implementation field
```

When reporting a sensitive finding, do not reproduce the value.

Use this redaction form:

```text
field_name = [REDACTED: prohibited direct identifier]
field_name = [REDACTED: derived physiological metric]
field_name = [REDACTED: free-text content]
field_name = [REDACTED: credential payload material]
```

The report may include:

```text
repository path
type name
field name
data-class classification
whether the type is public
whether the type is serializable
whether the field crosses a crate boundary
recommended Skynet disposition
```

The report must not include actual secret, biometric, clinical, credential,
wallet, endpoint, host-specific audit, or telemetry values.

Use these Skynet dependency dispositions:

```text
CORE_SAFE
ADAPTER_ONLY
POLICY_REFERENCE_ONLY
PROHIBITED
```

A dependency is `CORE_SAFE` only when its public contracts can be proven not to
admit prohibited, linkable, unbounded, or unstructured data.

## 3. ALN Validator Scope

The initial Rust-only ALN validator must support the current repository dialects
that are needed to assess existing policy and schema consistency.

It must not invent support for speculative future syntax.

Initial supported scope:

```text
current repository-local ALN declaration forms
repository-local imports
metadata blocks
schema declarations
records
enumerations
constraints
invariants
bindings
catalogs
policy instances
required-field checks
cross-file import resolution
schema-instance field compatibility
Skynet prohibited-data checks
policy-authority binding checks
```

The validator must use a versioned dialect registry:

```text
DialectId
DialectVersion
ParserProfile
SemanticRuleSet
FixtureSet
MigrationRuleSet
```

Future Skynet ALN extensions require all of the following before support is
added:

1. A written grammar proposal.
2. A versioned `DialectId` or extension identifier.
3. A semantic contract with required and prohibited fields.
4. Positive and negative fixtures.
5. Import-resolution behavior.
6. Compatibility and migration rules.
7. A privacy review proving no prohibited Skynet data class can enter a
   Skynet policy artifact.
8. Validator tests covering parsing, semantic validation, and rejection paths.

The initial validator may reject unknown syntax with a structured diagnostic:

```text
code = ALN_UNKNOWN_DIALECT
severity = error
action = require_registered_dialect_or_migration_rule
```

Do not implement automatic syntax conversion, automatic schema repair, or
silent compatibility fallback.
