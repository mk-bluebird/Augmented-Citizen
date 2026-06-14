// filename: ConsentDashboardViewModel.kt
// destination: kotlin-mobile-host/app/src/main/java/org/augcit/mobile/ui/ConsentDashboardViewModel.kt
// SPDX-License-Identifier: MIT OR Apache-2.0

package org.augcit.mobile.ui

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import org.augcit.mobile.host.ConsentSyncManager
import org.augcit.mobile.model.HostNeurorightsState
import javax.inject.Inject

/**
 * ViewModel for the host's consent and neurorights dashboard.
 * Surfaces the current ALN envelope state and allows host-initiated revocations.
 */
@HiltViewModel
class ConsentDashboardViewModel @Inject constructor(
    private val consentSyncManager: ConsentSyncManager
) : ViewModel() {

    private val _uiState = MutableStateFlow<HostNeurorightsState>(HostNeurorightsState.Loading)
    val uiState: StateFlow<HostNeurorightsState> = _uiState.asStateFlow()

    init {
        loadCurrentState()
    }

    private fun loadCurrentState() {
        viewModelScope.launch {
            try {
                val state = consentSyncManager.fetchHostNeurorightsState()
                _uiState.value = state
            } catch (e: Exception) {
                _uiState.value = HostNeurorightsState.Error("Failed to load sovereignty state.")
            }
        }
    }

    /**
     * Host-initiated action: Revoke a specific consent envelope.
     * This pushes a revocation event to the ALN audit ledger via the backend.
     */
    fun revokeConsent(envelopeId: String) {
        viewModelScope.launch {
            consentSyncManager.revokeEnvelope(envelopeId)
            loadCurrentState() // Refresh UI
        }
    }

    /**
     * Host-initiated action: Opt-in to a new research context.
     * Requires explicit cryptographic signing by the host device.
     */
    fun optInToResearch(researchProtocolId: String) {
        viewModelScope.launch {
            consentSyncManager.signAndSubmitConsent(researchProtocolId)
            loadCurrentState()
        }
    }
}
