// filename: AlnEnforcer.cpp
// destination: cpp-device-bridge/src/AlnEnforcer.cpp
// SPDX-License-Identifier: MIT OR Apache-2.0

#include "augcit/edge/AlnEnforcer.hpp"
#include "augcit/edge/AuditHook.hpp"
#include <atomic>

namespace augcit::edge {

static std::atomic<uint64_t> audit_counter{0};

AlnEnforcer::AlnEnforcer(const CompiledPolicyCache& cache) 
    : policy_cache_(cache) {}

EnforcementResult AlnEnforcer::isChannelAllowedForRead(
    std::string_view host_did, 
    ChannelCategory channel, 
    std::string_view purpose) const 
{
    // 1. Fail closed if policy cache is not loaded
    if (!policy_cache_.is_loaded) {
        return failClosed("Policy cache not initialized");
    }

    // 2. Red-line check (Hard Block)
    if (isRedLineTriggered(channel, OperationType::READ_SUMMARY, purpose)) {
        return failClosed("Red-line violation: Prohibited read purpose");
    }

    // 3. Consent check (Never infer consent locally)
    if (!hasValidConsent(host_did, channel, purpose)) {
        return failClosed("No valid consent envelope for read");
    }

    // 4. Permit and emit audit hook
    uint64_t audit_id = generateAuditId();
    AuditHook::emitReadEvent(host_did, channel, purpose, audit_id);

    return EnforcementResult{true, "", audit_id};
}

EnforcementResult AlnEnforcer::isModulationAllowed(
    std::string_view host_did, 
    ChannelCategory channel, 
    OperationType operation) const 
{
    if (!policy_cache_.is_loaded) {
        return failClosed("Policy cache not initialized");
    }

    // Meta-cognitive modulation is absolutely forbidden at the edge
    if (channel == ChannelCategory::META_COGNITIVE && 
        (operation == OperationType::MODULATE_THERAPEUTIC || operation == OperationType::MODULATE_SUPPORTIVE)) {
        return failClosed("Red-line violation: Meta-cognitive modulation blocked");
    }

    // Affective long-term conditioning blocked
    if (channel == ChannelCategory::AFFECTIVE && operation == OperationType::MODULATE_THERAPEUTIC) {
        // In production, checks session duration limits from compiled policy
        return failClosed("Red-line violation: Affective conditioning limits exceeded");
    }

    if (!hasValidConsent(host_did, channel, "")) {
        return failClosed("No valid consent envelope for modulation");
    }

    uint64_t audit_id = generateAuditId();
    AuditHook::emitModulationEvent(host_did, channel, operation, audit_id);

    return EnforcementResult{true, "", audit_id};
}

bool AlnEnforcer::hasValidConsent(std::string_view host_did, ChannelCategory channel, std::string_view purpose) const {
    // Queries the compiled policy cache for a valid, non-expired consent signature 
    // matching the host_did, channel, and purpose.
    return false; // Stub: Fails closed by default if not implemented
}

bool AlnEnforcer::isRedLineTriggered(ChannelCategory channel, OperationType operation, std::string_view purpose) const {
    // Checks compiled bitmask of prohibited patterns
    return false; // Stub
}

uint64_t AlnEnforcer::generateAuditId() const {
    return audit_counter.fetch_add(1, std::memory_order_relaxed);
}

EnforcementResult AlnEnforcer::failClosed(std::string_view reason) const {
    return EnforcementResult{false, reason, 0};
}

} // namespace augcit::edge
