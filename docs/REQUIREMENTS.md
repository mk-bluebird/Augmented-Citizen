# Augmented Citizen Sovereignty: Requirements & Threat Model

## Overview

This document establishes the requirements and threat model for defending neuro-rights and cognitive sovereignty of augmented citizens. It serves as the foundational reference for all technical implementations in this repository.

## Core Requirements

### R1: Cognitive Sovereignty
- The augmented host MUST have exclusive authority over their thoughts, memories, and decision-making processes
- AI-augmented reasoning MUST be subject to the same sovereignty guarantees as biological cognition
- No external entity MAY override or bypass host consent for cognitive operations

### R2: Mental Privacy
- All neural data (BCI/MCI/EEG) MUST be encrypted at rest and in transit
- Access to mental state data REQUIRES explicit, verifiable consent
- Hosts MUST be able to audit all accesses to their neural data

### R3: Memory Integrity
- Memory systems MUST maintain cryptographic integrity proofs
- Unauthorized modifications MUST be detectable and reversible
- Psychological continuity MUST be preserved across all system states

### R4: Cognitive Liberty
- Hosts MUST be free from non-consensual behavioral manipulation
- No system MAY implement covert influence operations
- All neuromodulation MUST have explicit consent envelopes

### R5: Explicit, Revocable Consent
- Consent MUST be machine-verifiable and cryptographically bound
- Revocation MUST take effect immediately upon declaration
- Consent scopes MUST be granular and purpose-limited

### R6: Host-Sovereign Evolution
- All upgrades MUST increase or preserve host capabilities
- Rollbacks and downgrades are FORBIDDEN except for isolating bio-incompatible modules
- No micro-reversals of capability are permitted

### R7: On-Chain Accountability
- All rights assertions MUST be anchored to verifiable records
- Bostrom addresses and DIDs MUST be used for identity binding
- Audit trails MUST be tamper-evident and durable

## Threat Model

### Adversaries

#### A1: State Actors
- Unlawful surveillance programs
- Coercive behavioral control initiatives
- Non-consensual experimentation frameworks

#### A2: Corporate Entities
- Data exploitation without consent
- Manipulative AI systems
- Covert telemetry collection

#### A3: Malicious Third Parties
- Neural data theft
- Identity spoofing attacks
- Unauthorized neuromodulation

### Attack Vectors

#### V1: Non-Consensual Data Access
- Intercepting neural telemetry
- Compromising storage systems
- Social engineering for consent bypass

#### V2: Covert Manipulation
- Subliminal AI influence
- Behavioral nudging without disclosure
- Exploitation of cognitive vulnerabilities

#### V3: System Compromise
- Firmware backdoors
- Supply chain attacks on neuro-devices
- OTA update hijacking

#### V4: Identity Attacks
- DID spoofing
- Bostrom address forgery
- Consent record tampering

### Historical Context

This project explicitly rejects and counters historical mind-control programs including:
- MKUltra and derivatives
- Drug-based behavior modification experiments
- Covert psychological operations on unwitting subjects

These historical abuses serve as negative examples—design patterns to avoid at all costs.

## Compliance Mappings

### International Human Rights
- Universal Declaration of Human Rights, Article 12 (Privacy)
- International Covenant on Civil and Political Rights, Article 17
- Emerging Neuro-Rights Frameworks (Chile, Spain, etc.)

### Technical Standards
- W3C Decentralized Identifiers (DID)
- Blockchain-based accountability systems
- Zero-knowledge proof systems for consent verification

## Implementation Guidance

All crates in this repository MUST:
1. Declare ALN compliance clauses
2. Bind to `host_did` and `bostrom_address`
3. Implement consent gating before any neural operation
4. Provide audit trails for all sensitive operations
5. Enforce non-reversal invariants

## References

- See `aln-core-spec/` for core schemas
- See `aln-bci-special/` for BCI-specific protections
- See `aln-consent-envelopes/` for consent templates
- See `crates/sovereign-guards-core/` for implementation
