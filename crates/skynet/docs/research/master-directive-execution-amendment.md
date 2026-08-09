# Skynet Master Directive Execution Amendment

## Status Discipline

Research records may propose candidates but cannot self-select governance-gated
architecture.

```text
F6_PROFILE_IDENTIFIER = CANDIDATE | SELECTED | REJECTED
S6_MECHANISM = CANDIDATE | SELECTED | REJECTED
H5_SERIALIZATION = DRAFT | SELECTED | REJECTED
P3_STATUS = OPEN | PROPOSED | ADOPTED | REJECTED
```

`SELECTED` and `ADOPTED` require an authoritative Skynet governance record.

## F6 Required Rejection Matrix

```text
Malformed CBOR                         -> adapter reject
Invalid COSE_Sign1 structure           -> adapter reject
Duplicate COSE header label            -> adapter reject
Protected/unprotected header collision -> adapter reject
Unsupported critical header            -> adapter reject
Unsupported algorithm                  -> adapter reject
Unknown profile identifier             -> Unrecognized
Unsupported profile version            -> Unrecognized
Invalid signature or issuer trust      -> adapter reject
Expired or not-yet-valid credential    -> normalized lifecycle result
Audience mismatch                      -> adapter reject
Invalid holder binding                 -> adapter reject
Disclosure beyond approved scope       -> disclosure nonconformance
```

## S6 Candidate Neutrality

Use these mechanism-neutral terms until S6 is selected:

```text
status evidence
status authority
publication unit
freshness bound
offline evidence
authority conflict
authority recovery
```

Use `witness` only in a selected or explicitly evaluated accumulator candidate.

## H5 Ownership Rules

```text
Adapter owns:
- nonce/challenge validation
- proof verification
- consumed-request/replay state
- wallet interaction
- raw authorization serialization

Core owns:
- typed context comparison
- temporal validity comparison
- policy-lineage comparison
- semantic RequestBinding handling
- closed decision generation after P3 adoption
```

## No-Code Rule

The following remain blocked pending their upstream gates:

```text
src/policy.rs
src/pipeline.rs
src/network.rs
aln/skynet-civic-identity.v1.aln
fixtures/credential/
fixtures/presentation/
fixtures/policy/
fixtures/audit/
```

Research documents, matrices, schemas, source inventories, and test plans are
permitted. New gate-dependent Rust implementation is not.
