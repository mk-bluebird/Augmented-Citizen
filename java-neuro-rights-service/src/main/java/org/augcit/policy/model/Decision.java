// filename: Decision.java
// destination: java-neuro-rights-service/src/main/java/org/augcit/policy/model/Decision.java
// SPDX-License-Identifier: MIT OR Apache-2.0

package org.augcit.policy.model;

/**
 * Sealed hierarchy representing the outcome of a neurorights policy evaluation.
 * All decisions are final and auditable.
 */
public sealed interface Decision permits Decision.Allow, Decision.Deny, Decision.RequireConfirmation {

    record Allow(String reason, String envelopeId) implements Decision {}

    record Deny(String reason, String redLineId, String invariantViolated) implements Decision {}

    record RequireConfirmation(
            String reason, 
            double autonomyScore, 
            String[] engagedNeurorights
    ) implements Decision {}
}
