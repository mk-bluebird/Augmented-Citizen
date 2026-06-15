package org.cybernano.intent

import kotlin.math.exp

data class DecoderOutput(
    val tokens: List<String>,
    val logits: FloatArray,
    val psychRiskIndex: Float
)

class BayesianIntentDecoder(
    private val highRiskTokens: Set<String>,
    private val psychSoftCeiling: Float
) {
    fun decode(raw: DecoderOutput): Pair<String, FloatArray> {
        val probs = softmax(raw.logits)
        val masked = applySafetyPrior(
            probs = probs,
            tokens = raw.tokens,
            psychRisk = raw.psychRiskIndex
        )
        val (idx, _) = masked.withIndex().maxByOrNull { it.value }!!
        val token = raw.tokens[idx]
        return token to masked
    }

    private fun softmax(logits: FloatArray): FloatArray {
        val max = logits.maxOrNull() ?: 0f
        val exps = FloatArray(logits.size) { i -> exp(logits[i] - max) }
        val sum = exps.sum()
        return FloatArray(logits.size) { i -> exps[i] / sum }
    }

    private fun applySafetyPrior(
        probs: FloatArray,
        tokens: List<String>,
        psychRisk: Float
    ): FloatArray {
        val masked = probs.copyOf()
        if (psychRisk >= psychSoftCeiling) {
            for (i in tokens.indices) {
                if (highRiskTokens.contains(tokens[i])) {
                    masked[i] = 0f
                }
            }
        }
        val sum = masked.sum()
        if (sum <= 0f) {
            // Fallback: all probability mass to REST.
            val restIdx = tokens.indexOf("REST").coerceAtLeast(0)
            return FloatArray(tokens.size) { i -> if (i == restIdx) 1f else 0f }
        }
        for (i in masked.indices) {
            masked[i] /= sum
        }
        return masked
    }
}
