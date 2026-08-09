# Skynet Gate-Closure Corrections

## Normative Authority

Skynet governance is authoritative for:

- Internal core-boundary rules.
- Profile selection.
- P3 policy semantics.
- ALN policy-lineage integration.
- Release and code-generation gates.

External standards are normative for protocol requirements explicitly adopted by
a selected Skynet profile. They remain interoperability and security evidence;
they are not substitutes for Skynet policy governance.

## Gate Sequence

```text
Selection sequence:
F6 -> S6 -> H5 -> P3 -> combined-stack threat model

Parallel research permitted:
- H5 semantic binding research may proceed during F6/S6 research.
- Final H5 serialization remains blocked until F6 selection.
- P3 implementation remains blocked until F6, S6, H5, and P3 adoption.
```

## F6 Layers

```text
F6-A = CWT/COSE container rules
F6-B = Skynet credential application profile
F6-C = selective disclosure and holder binding
F6-D = OpenID4VP transport interoperability
```

## S6 Mapping

```text
fresh verified usable evidence     -> Active
expired credential validity        -> Expired
fresh verified reversible disable  -> Suspended
stale/missing/conflicting evidence -> Unavailable
unsupported mechanism/profile      -> Unrecognized
```

```text
Unavailable != Expired
Unrecognized != Suspended
Only Active may satisfy an active-required policy rule.
```

## Prohibited Assumptions

```text
- Do not expand ALN without verified grammar and validator evidence.
- Do not require blockchain anchoring for credential status.
- Do not invent a P3 policy language or reason taxonomy.
- Do not put CWT, COSE, OpenID4VP, wallet, or transport payloads in core types.
- Do not generate gate-dependent implementation until its upstream gate closes.
```

## Corrected Completion Condition

```text
FOUNDATION_CHECKS = PASSING
SERIALIZATION_BOUNDARY_AUDIT = COMPLETE
F6 = SELECTED
S6 = SELECTED
H5 = SELECTED
P3 = ADOPTED
THREAT_MODEL = UPDATED
POLICY_IR = REVIEWED
PIPELINE_STATE_MACHINE = REVIEWED
ALN_GRAMMAR = VERIFIED
FIXTURE_SCHEMAS = FROZEN
FORMAL_PROOFS = PASSING
```
