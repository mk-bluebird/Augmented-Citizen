// filename: PolicyEvaluationEngine.java
// destination: java-neuro-rights-service/src/main/java/org/augcit/policy/PolicyEvaluationEngine.java
// SPDX-License-Identifier: MIT OR Apache-2.0

package org.augcit.policy;

import org.augcit.aln.model.*;
import org.augcit.host.HostProfile;
import org.augcit.policy.model.Decision;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

/**
 * Core policy evaluation engine. Gates all neurodata reads and modulations.
 * Computes a policy-only autonomy score (no real-time physiological feedback).
 */
@ApplicationScoped
public class PolicyEvaluationEngine {

    @Inject RedLineEnforcer redLineEnforcer;
    @Inject ConsentResolver consentResolver;
    @Inject AutonomyScorer autonomyScorer;

    public Decision checkNeurodataRead(HostProfile host, ChannelCategory channel, String purpose) {
        // 1. Red-line check (Hard Block)
        var redLineBlock = redLineEnforcer.evaluate(channel, OperationType.READ, purpose);
        if (redLineBlock.isPresent()) return redLineBlock.get();

        // 2. Consent check
        var consent = consentResolver.resolveReadConsent(host, channel, purpose);
        if (consent.isEmpty()) {
            return new Decision.Deny("No valid consent envelope found for read operation.", null, "MENTAL_PRIVACY");
        }

        // 3. Autonomy Score (Policy-only)
        double score = autonomyScorer.computeReadScore(host, channel, purpose, consent.get());
        
        if (score < 0.60) {
            return new Decision.Deny("Autonomy score below hard-block threshold.", null, "COGNITIVE_LIBERTY");
        } else if (score < 0.80) {
            return new Decision.RequireConfirmation(
                "Operation engages sensitive neurorights. Host confirmation required.",
                score,
                new String[]{"MENTAL_PRIVACY", "COGNITIVE_LIBERTY"}
            );
        }

        return new Decision.Allow("Read permitted under consent envelope.", consent.get().envelopeId());
    }

    public Decision checkTherapeuticModulation(HostProfile host, ChannelCategory channel, ModulationParameters params) {
        // 1. Red-line check (Hard Block)
        var redLineBlock = redLineEnforcer.evaluate(channel, OperationType.MODULATE, params.purpose());
        if (redLineBlock.isPresent()) return redLineBlock.get();

        // 2. Consent check (Must be explicit therapeutic)
        var consent = consentResolver.resolveModulationConsent(host, channel, params);
        if (consent.isEmpty() || !consent.get().isTherapeuticControlled()) {
            return new Decision.Deny("Therapeutic modulation requires explicit, controlled consent.", null, "MENTAL_INTEGRITY");
        }

        // 3. Autonomy Score
        double score = autonomyScorer.computeModulationScore(host, channel, params, consent.get());
        
        if (score < 0.70) {
            return new Decision.Deny("Modulation autonomy score below threshold.", null, "MENTAL_INTEGRITY");
        } else if (score < 0.85) {
            return new Decision.RequireConfirmation(
                "Therapeutic modulation requires high-salience host confirmation.",
                score,
                new String[]{"MENTAL_INTEGRITY", "PSYCHOLOGICAL_CONTINUITY"}
            );
        }

        return new Decision.Allow("Therapeutic modulation permitted.", consent.get().envelopeId());
    }
}
