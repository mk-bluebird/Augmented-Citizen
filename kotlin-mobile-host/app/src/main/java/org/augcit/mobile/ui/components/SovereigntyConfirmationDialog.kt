// filename: SovereigntyConfirmationDialog.kt
// destination: kotlin-mobile-host/app/src/main/java/org/augcit/mobile/ui/components/SovereigntyConfirmationDialog.kt
// SPDX-License-Identifier: MIT OR Apache-2.0

package org.augcit.mobile.ui.components

import androidx.compose.foundation.layout.*
import androidx.compose.material3.*
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import org.augcit.policy.model.Decision

/**
 * High-salience confirmation dialog shown when the backend returns a RequireConfirmation decision.
 * Ensures the host explicitly acknowledges the engaged neurorights before proceeding.
 */
@Composable
fun SovereigntyConfirmationDialog(
    decision: Decision.RequireConfirmation,
    onConfirm: () -> Unit,
    onDecline: () -> Unit
) {
    AlertDialog(
        onDismissRequest = onDecline,
        title = { Text("Neurorights Confirmation Required") },
        text = {
            Column {
                Text("The requested operation engages sensitive cognitive states.")
                Spacer(modifier = Modifier.height(8.dp))
                Text("Autonomy Score: ${"%.2f".format(decision.autonomyScore)}")
                Spacer(modifier = Modifier.height(8.dp))
                Text("Engaged Rights:")
                decision.engagedNeurorights.forEach { right ->
                    Text("• $right", style = MaterialTheme.typography.bodyMedium)
                }
                Spacer(modifier = Modifier.height(16.dp))
                Text(
                    "Only proceed if you explicitly understand and consent to this modulation.",
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.error
                )
            }
        },
        confirmButton = {
            Button(onClick = onConfirm) {
                Text("I Consent")
            }
        },
        dismissButton = {
            OutlinedButton(onClick = onDecline) {
                Text("Deny & Halt")
            }
        }
    )
}
