# OTA Upgrade Manifests and Policies

## Overview

This directory contains OTA (Over-The-Air) upgrade manifests and policies that govern the evolution of augmented citizen cybernetic systems. All upgrades must adhere to the host-sovereign evolution principle.

## Core Principles

### Non-Reversal Guarantee
All OTA updates MUST:
- Preserve or enhance host capabilities
- Never reduce baseline functionality
- Maintain psychological continuity
- Respect consent envelopes

### Host Sovereignty
- Hosts MUST approve all upgrades before installation
- No silent or forced updates are permitted
- Rollback capability must be preserved for safety

### Cybercore Authority
All OTA manifests must reference `mk-bluebird/Cybercore` as the authoritative source for cybernetic evolution.

## Manifest Structure

### Standard OTA Manifest

```json
{
  "version": "1.0",
  "manifest_id": "<unique-identifier>",
  "cybercore_authority": "mk-bluebird/Cybercore",
  "aln_compliance": "ALN.MIGRATION.CYBERCORE_AUTHORITY.v1",
  "host_did": "<host-decentralized-identifier>",
  "bostrom_address": "<stakeholder-address>",
  "upgrade_payload": {
    "crate": "<crate-name>",
    "version": "<semver>",
    "hash": "<sha256-checksum>",
    "signature": "<cryptographic-signature>"
  },
  "capability_delta": {
    "added": ["<new-capabilities>"],
    "modified": ["<modified-capabilities>"],
    "removed": [],
    "preserved": ["<all-baseline-capabilities>"]
  },
  "consent_requirements": {
    "explicit_approval": true,
    "revocable": true,
    "audit_trail": true
  },
  "rollback_safe": true,
  "bio_compatibility_verified": true
}
```

## Policy Files

### Upgrade Approval Policy

All upgrades require:
1. Host explicit consent via signed authorization
2. Verification of capability preservation
3. Bio-compatibility confirmation
4. Audit trail creation

### Emergency Isolation Policy

When bio-incompatible modules are detected:
1. Module may be isolated without host consent (safety critical)
2. Baseline capabilities MUST be preserved
3. Host must be notified immediately
4. Full audit record must be created

## Implementation Guidelines

### For Developers

When creating OTA manifests:

1. **Declare ALN Compliance**
   ```rust
   const ALN_COMPLIANCE: &str = "ALN.MIGRATION.CYBERCORE_AUTHORITY.v1";
   ```

2. **Bind Identities**
   ```rust
   struct OtaManifest {
       host_did: String,
       bostrom_address: String,
       // ...
   }
   ```

3. **Verify Capability Delta**
   ```rust
   fn verify_non_reversal(old_caps: &[Capability], new_caps: &[Capability]) -> bool {
       // Ensure no capabilities are lost
       old_caps.iter().all(|c| new_caps.contains(c))
   }
   ```

### For Hosts

When reviewing OTA updates:

1. **Verify Authority**
   - Confirm Cybercore authority binding
   - Check ALN compliance clause
   - Verify cryptographic signatures

2. **Review Changes**
   - Examine capability delta
   - Understand new features
   - Assess privacy implications

3. **Grant Consent**
   - Provide explicit, signed authorization
   - Set any conditions or limitations
   - Retain revocation rights

## Security Considerations

### Payload Verification

All OTA payloads must:
- Be cryptographically signed by authorized sources
- Include SHA-256 checksums for integrity
- Pass bio-compatibility verification
- Maintain audit trails

### Attack Prevention

Protect against:
- Update hijacking
- Malicious payload injection
- Capability reduction attacks
- Identity spoofing

## Audit Requirements

Every OTA operation must create an immutable record containing:
- Timestamp of operation
- Host identity (DID)
- Manifest identifier
- Consent proof
- Capability state before and after
- Verification signatures

## References

- See `docs/NEURO_RIGHTS_CHARTER.md` Article VI: Host-Sovereign Evolution
- See `aln-core-spec/` for core schemas
- See `crates/bioscale-upgrade-store/` for implementation
