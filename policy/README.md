# Machine-Readable Policies for Augmented Citizen Rights

## Overview

This directory contains machine-readable policies that encode neuro-rights, consent frameworks, and accountability mechanisms for integration into smart contracts, verification pipelines, and runtime enforcement systems.

## Policy Categories

### 1. Consent Policies

Define the structure and validation rules for consent operations.

#### Policy: CONSENT.GRANT.v1

```json
{
  "policy_id": "CONSENT.GRANT.v1",
  "type": "consent_grant",
  "version": "1.0",
  "schema": {
    "required_fields": [
      "host_did",
      "operation_type",
      "scope",
      "duration",
      "purpose",
      "timestamp",
      "signature"
    ],
    "validation_rules": {
      "host_did": "MUST be valid W3C DID format",
      "operation_type": "MUST be from approved operation catalog",
      "scope": "MUST specify exact data/operation boundaries",
      "duration": "MUST have explicit expiration or 'revocable'",
      "purpose": "MUST be human-readable and specific",
      "signature": "MUST be cryptographically verifiable"
    }
  },
  "enforcement": {
    "pre_operation_check": true,
    "audit_logging": true,
    "revocation_support": true
  }
}
```

#### Policy: CONSENT.REVOKE.v1

```json
{
  "policy_id": "CONSENT.REVOKE.v1",
  "type": "consent_revocation",
  "version": "1.0",
  "schema": {
    "required_fields": [
      "host_did",
      "original_consent_ref",
      "revocation_reason",
      "timestamp",
      "signature"
    ],
    "validation_rules": {
      "host_did": "MUST match original consent grantor",
      "original_consent_ref": "MUST reference valid consent record",
      "revocation_reason": "OPTIONAL but recommended",
      "signature": "MUST be cryptographically verifiable"
    }
  },
  "enforcement": {
    "immediate_effect": true,
    "notification_required": true,
    "audit_logging": true
  }
}
```

### 2. Neuro-Rights Policies

Encode fundamental neuro-rights as machine-verifiable constraints.

#### Policy: NEURO.RIGHTS.COGNITIVE_SOVEREIGNTY.v1

```json
{
  "policy_id": "NEURO.RIGHTS.COGNITIVE_SOVEREIGNTY.v1",
  "type": "rights_constraint",
  "version": "1.0",
  "invariants": [
    {
      "name": "exclusive_cognitive_authority",
      "description": "Host has exclusive authority over their cognitive processes",
      "constraint": "NO_EXTERNAL_OVERRIDE",
      "enforcement_level": "CRITICAL"
    },
    {
      "name": "ai_mediated_protection",
      "description": "AI-augmented cognition is protected as host's own",
      "constraint": "EQUAL_PROTECTION_AS_BIOLOGICAL",
      "enforcement_level": "CRITICAL"
    },
    {
      "name": "freedom_from_coercion",
      "description": "No coercive influence on cognitive processes",
      "constraint": "NO_SUBLIMINAL_NO_FORCED_MODIFICATION",
      "enforcement_level": "CRITICAL"
    }
  ],
  "violations": {
    "reporting": "IMMEDIATE",
    "logging": "IMMUTABLE",
    "remediation": "AUTOMATIC_ISOLATION"
  }
}
```

#### Policy: NEURO.RIGHTS.MENTAL_PRIVACY.v1

```json
{
  "policy_id": "NEURO.RIGHTS.MENTAL_PRIVACY.v1",
  "type": "rights_constraint",
  "version": "1.0",
  "invariants": [
    {
      "name": "neural_data_encryption",
      "description": "All neural data encrypted at rest and in transit",
      "constraint": "ENCRYPTION_REQUIRED_AES256_MINIMUM",
      "enforcement_level": "CRITICAL"
    },
    {
      "name": "consent_required_access",
      "description": "Access requires explicit verifiable consent",
      "constraint": "NO_ACCESS_WITHOUT_CONSENT",
      "enforcement_level": "CRITICAL"
    },
    {
      "name": "audit_right",
      "description": "Host can audit all accesses to neural data",
      "constraint": "FULL_AUDIT_TRAIL_MANDATORY",
      "enforcement_level": "HIGH"
    }
  ]
}
```

### 3. ALN Compliance Policies

Define ALN (Augmented Ledger Network) compliance requirements.

#### Policy: ALN.COMPLIANCE.CYBERCORE_AUTHORITY.v1

```json
{
  "policy_id": "ALN.COMPLIANCE.CYBERCORE_AUTHORITY.v1",
  "type": "aln_compliance",
  "version": "1.0",
  "requirements": [
    {
      "clause": "CYBERCORE_AUTHORITY_BINDING",
      "description": "All evolution must reference mk-bluebird/Cybercore",
      "validation": "AUTHORITY_FIELD_EQUALS_CYBERCORE",
      "mandatory": true
    },
    {
      "clause": "NO_REVERSAL",
      "description": "No downgrades or capability reductions",
      "validation": "CAPABILITY_DELTA_NON_NEGATIVE",
      "mandatory": true
    },
    {
      "clause": "IDENTITY_BINDING",
      "description": "Must bind host_did and bostrom_address",
      "validation": "BOTH_IDENTITIES_PRESENT_AND_VALID",
      "mandatory": true
    },
    {
      "clause": "AUDIT_TRAIL",
      "description": "All operations must be auditable",
      "validation": "IMMUTABLE_LOG_ENTRY_CREATED",
      "mandatory": true
    }
  ],
  "penalties": {
    "non_compliance": "REJECT_OPERATION",
    "violation_attempt": "LOG_AND_ALERT",
    "repeated_violations": "SYSTEM_ISOLATION"
  }
}
```

### 4. Resource Accounting Policies

Govern resource usage for biological and cybernetic resources.

#### Policy: RESOURCE.ACCOUNTING.BIO_TOKENS.v1

```json
{
  "policy_id": "RESOURCE.ACCOUNTING.BIO_TOKENS.v1",
  "type": "resource_accounting",
  "version": "1.0",
  "token_domains": [
    "BLOOD",
    "SUGAR",
    "PROTEIN",
    "LIFEFORCE",
    "OXYGEN",
    "BRAIN",
    "WAVE",
    "DW",
    "PAIN",
    "FEAR"
  ],
  "constraints": {
    "PAIN": {
      "manipulation_allowed": false,
      "host_verified_controls_only": true,
      "audit_level": "MAXIMUM"
    },
    "FEAR": {
      "manipulation_allowed": false,
      "host_verified_controls_only": true,
      "audit_level": "MAXIMUM"
    },
    "BLOOD": {
      "overload_prevention": true,
      "long_term_degradation_check": true
    },
    "BRAIN": {
      "overload_prevention": true,
      "long_term_degradation_check": true
    }
  },
  "enforcement": {
    "quota_monitoring": true,
    "alert_thresholds": true,
    "automatic_throttling": true
  }
}
```

### 5. Audit Policies

Define audit trail requirements and verification.

#### Policy: AUDIT.TRAIL.IMMUTABLE.v1

```json
{
  "policy_id": "AUDIT.TRAIL.IMMUTABLE.v1",
  "type": "audit_policy",
  "version": "1.0",
  "requirements": {
    "entries_must_include": [
      "timestamp_utc",
      "operation_type",
      "host_did",
      "actor_identity",
      "consent_reference",
      "operation_result",
      "cryptographic_hash"
    ],
    "storage": {
      "immutability": "CRYPTOGRAPHICALLY_ENFORCED",
      "retention": "PERMANENT",
      "accessibility": "HOST_READABLE_ALWAYS"
    },
    "verification": {
      "chain_integrity": "HASH_CHAIN_VERIFIED",
      "tamper_detection": "AUTOMATIC_ALERT",
      "periodic_audit": "CONTINUOUS"
    }
  }
}
```

## Implementation Patterns

### Rust Integration

```rust
// Example policy enforcement in Rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsentPolicy {
    pub policy_id: String,
    pub policy_type: String,
    pub version: String,
    pub schema: PolicySchema,
    pub enforcement: EnforcementRules,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PolicySchema {
    pub required_fields: Vec<String>,
    pub validation_rules: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnforcementRules {
    pub pre_operation_check: bool,
    pub audit_logging: bool,
    pub revocation_support: bool,
}

impl ConsentPolicy {
    pub fn validate(&self, operation: &ConsentOperation) -> Result<(), PolicyViolation> {
        // Validate required fields
        for field in &self.schema.required_fields {
            if !operation.has_field(field) {
                return Err(PolicyViolation::MissingRequiredField(field.clone()));
            }
        }
        
        // Apply validation rules
        for (field, rule) in &self.schema.validation_rules {
            if !self.apply_rule(rule, operation.get_field(field)?) {
                return Err(PolicyViolation::RuleViolation(field.clone(), rule.clone()));
            }
        }
        
        Ok(())
    }
}
```

### Smart Contract Integration

```solidity
// Example Solidity policy enforcement
contract NeuroRightsEnforcer {
    mapping(string => bool) public consentRecords;
    mapping(string => address) public hostIdentities;
    
    event ConsentGranted(string indexed hostDID, string operationType, uint256 timestamp);
    event ConsentRevoked(string indexed hostDID, string consentRef, uint256 timestamp);
    
    function grantConsent(
        string memory hostDID,
        string memory operationType,
        string memory scope,
        uint256 duration
    ) external returns (string memory) {
        require(validateHostIdentity(hostDID, msg.sender), "Invalid host identity");
        
        string memory consentId = generateConsentId();
        consentRecords[consentId] = true;
        
        emit ConsentGranted(hostDID, operationType, block.timestamp);
        return consentId;
    }
    
    function revokeConsent(string memory consentId) external {
        require(consentRecords[consentId], "Consent not found");
        consentRecords[consentId] = false;
        
        emit ConsentRevoked(msg.sender, consentId, block.timestamp);
    }
    
    function verifyConsent(string memory consentId) external view returns (bool) {
        return consentRecords[consentId];
    }
}
```

## Verification Pipeline

### Pre-Operation Checks

Before any neural operation:
1. Verify host identity (DID)
2. Check valid consent exists
3. Confirm operation within consent scope
4. Validate ALN compliance
5. Log operation intent

### Post-Operation Auditing

After any neural operation:
1. Record operation details
2. Update audit trail
3. Verify no rights violations occurred
4. Generate cryptographic proof
5. Notify host if required

## References

- See `docs/NEURO_RIGHTS_CHARTER.md` for full rights framework
- See `docs/REQUIREMENTS.md` for implementation requirements
- See `aln-core-spec/` for core schemas
- See `crates/sovereign-guards-core/` for Rust implementations
