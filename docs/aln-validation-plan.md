# ALN Validation Plan

## Decision

Do not adopt the submitted script as the repository validator.

It is incomplete for the observed ALN syntaxes, does not establish semantic
type compatibility, and is outside the approved implementation language for
this repository.

## Approved Direction

Create a Rust-only tool after Skynet core contracts are approved:

```text
tools/aln-contract-lint/
```

The tool must not parse or retain raw neural, credential-claim, device,
location, or host-operational audit data. It validates ALN syntax and policy
structure only.

## Initial Validation Scope

1. Detect the declared ALN dialect.
2. Validate balanced structural delimiters.
3. Resolve only repository-local imports.
4. Reject unresolved imports.
5. Validate required metadata fields.
6. Validate canonical policy-authority binding.
7. Validate that schema instances provide required fields.
8. Reject incompatible field declarations.
9. Reject prohibited Skynet data classes in Skynet ALN artifacts.
10. Emit machine-readable diagnostics containing file path, line, code, and
    severity without copying sensitive source values.

## Deferred Scope

The first validator does not evaluate device commands, clinical policies,
neural data, credential claims, remote endpoints, ledger contents, or
biophysical measurements.
