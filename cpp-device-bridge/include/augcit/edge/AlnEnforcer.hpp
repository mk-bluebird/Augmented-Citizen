// filename: AlnEnforcer.hpp
// destination: cpp-device-bridge/include/augcit/edge/AlnEnforcer.hpp
// SPDX-License-Identifier: MIT OR Apache-2.0

#pragma once

#include <string_view>
#include <cstdint>
#include <vector>
#include <expected> // C++23, or use custom Result if strictly C++20. Using custom for max compatibility.

namespace augcit::edge {

enum class ChannelCategory : uint8_t {
    PERCEPTUAL, AFFECTIVE, MOTOR_INTENTION, MEMORY_ADJACENT, META_COGNITIVE
};

enum class OperationType : uint8_t {
    READ_SUMMARY, READ_DIAGNOSTIC, MODULATE_THERAPEUTIC, MODULATE_SUPPORTIVE
};

struct EnforcementResult {
    bool allowed;
    std::string_view denial_reason;
    uint64_t audit_event_id;
};

// Compiled policy cache loaded from ALN at boot. 
// Edge devices do not parse raw ALN text; they load a binary-compiled policy struct.
struct CompiledPolicyCache {
    bool is_loaded = false;
    // Internal bitmasks and lookup tables for O(1) red-line and consent checks
};

/**
 * Minimal, strict enforcer for edge devices.
 * Treats ALN inputs as absolute policies. Fails closed on any error or missing context.
 */
class AlnEnforcer {
public:
    explicit AlnEnforcer(const CompiledPolicyCache& cache);

    EnforcementResult isChannelAllowedForRead(
        std::string_view host_did, 
        ChannelCategory channel, 
        std::string_view purpose
    ) const;

    EnforcementResult isModulationAllowed(
        std::string_view host_did, 
        ChannelCategory channel, 
        OperationType operation
    ) const;

private:
    const CompiledPolicyCache& policy_cache_;
    
    // Internal helpers
    bool hasValidConsent(std::string_view host_did, ChannelCategory channel, std::string_view purpose) const;
    bool isRedLineTriggered(ChannelCategory channel, OperationType operation, std::string_view purpose) const;
    uint64_t generateAuditId() const;
    
    EnforcementResult failClosed(std::string_view reason) const;
};

} // namespace augcit::edge
