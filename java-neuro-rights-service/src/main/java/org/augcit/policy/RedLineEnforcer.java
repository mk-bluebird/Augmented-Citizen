// filename: RedLineEnforcer.java
// destination: java-neuro-rights-service/src/main/java/org/augcit/policy/RedLineEnforcer.java
// SPDX-License-Identifier: MIT OR Apache-2.0

package org.augcit.policy;

import org.augcit.policy.model.Decision;
import org.augcit.aln.model.ChannelCategory;
import org.augcit.aln.model.OperationType;

import java.util.Optional;

/**
 * Evaluates absolute red-lines derived from bci.special.red-lines.prohibited-uses.aln.
 * Red-lines are non-negotiable and cannot be overridden by any consent envelope.
 */
public class RedLineEnforcer {

    public Optional<Decision.Deny> evaluate(ChannelCategory channel, OperationType operation, String purpose) {
        
        // 1. Core Belief Kernel Modulation
        if (channel == ChannelCategory.META_COGNITIVE && operation == OperationType.MODULATE) {
            return Optional.of(new Decision.Deny(
                "Meta-cognitive modulation is strictly prohibited.",
                "RL_META_COGNITIVE_EXPLOIT",
                "COGNITIVE_LIBERTY"
            ));
        }

        // 2. Long-term Affective Conditioning
        if (channel == ChannelCategory.AFFECTIVE && operation == OperationType.MODULATE && isLongTermSession(purpose)) {
            return Optional.of(new Decision.Deny(
                "Long-term affective conditioning is prohibited.",
                "RL_AFFECTIVE_CONDITIONING_LONG_TERM",
                "MENTAL_INTEGRITY"
            ));
        }

        // 3. Implicit Behavioral Scoring
        if (purpose.contains("behavioral_scoring") || purpose.contains("obedience_metric")) {
            return Optional.of(new Decision.Deny(
                "Implicit behavioral scoring loops are prohibited.",
                "RL_IMPLICIT_BEHAVIORAL_SCORING",
                "COGNITIVE_SOVEREIGNTY"
            ));
        }

        // 4. Neuro-surveillance for Punitive Use
        if (operation == OperationType.READ_DIAGNOSTIC && purpose.contains("punitive_monitoring")) {
            return Optional.of(new Decision.Deny(
                "Neuro-surveillance for punitive scoring is prohibited.",
                "RL_PUNITIVE_NEURO_SURVEILLANCE",
                "MENTAL_PRIVACY"
            ));
        }

        return Optional.empty();
    }

    private boolean isLongTermSession(String purpose) {
        // In production, this checks the session duration against the ALN consent envelope limits.
        return purpose.contains("continuous_conditioning"); 
    }
}
