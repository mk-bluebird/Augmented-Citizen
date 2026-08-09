export function lintResearchContract(contract, nowEpochSeconds) {
  const errors = [];
  const requiredBindings = [
    "presentation_request_id",
    "verifier_reference",
    "purpose",
    "consent_scope_id",
    "not_before",
    "expires_at",
    "policy_authority",
    "policy_version",
    "freshness"
  ];

  const requiredBoundaryOutputs = [
    "EligibilityDecision",
    "CredentialStatus",
    "HolderAuthorization",
    "PolicyLineage",
    "DisclosureReceipt",
    "AuditEvent"
  ];

  const prohibitedCoreFields = [
    "credential",
    "claim_values",
    "holder_did",
    "holder_public_key",
    "cryptographic_proof",
    "challenge",
    "transport_route",
    "raw_neural_data",
    "raw_physiological_data"
  ];

  if (!contract || typeof contract !== "object") {
    return { valid: false, errors: ["contract must be an object"] };
  }

  if (contract.f6?.profile_identifier !== "OPEN" &&
      contract.f6?.profile_identifier !== "SELECTED" &&
      contract.f6?.profile_identifier !== "REJECTED") {
    errors.push("f6.profile_identifier must be OPEN, SELECTED, or REJECTED");
  }

  if (contract.f6?.profile_identifier === "SELECTED") {
    for (const field of ["profile_version", "content_type", "digest_suite_id"]) {
      if (!contract.f6[field]) {
        errors.push(`selected F6 profile requires f6.${field}`);
      }
    }
  }

  const bindings = contract.h5?.mandatory_bindings;
  if (!Array.isArray(bindings)) {
    errors.push("h5.mandatory_bindings must be an array");
  } else {
    for (const binding of requiredBindings) {
      if (!bindings.includes(binding)) {
        errors.push(`missing H5 mandatory binding: ${binding}`);
      }
    }
  }

  const outputs = contract.core_boundary?.permitted_outputs;
  if (!Array.isArray(outputs)) {
    errors.push("core_boundary.permitted_outputs must be an array");
  } else {
    for (const output of requiredBoundaryOutputs) {
      if (!outputs.includes(output)) {
        errors.push(`missing core boundary output: ${output}`);
      }
    }
  }

  const coreFields = contract.core_boundary?.core_fields;
  if (!Array.isArray(coreFields)) {
    errors.push("core_boundary.core_fields must be an array");
  } else {
    for (const field of coreFields) {
      if (prohibitedCoreFields.includes(field)) {
        errors.push(`prohibited core field: ${field}`);
      }
    }
  }

  const freshness = contract.s6?.max_status_age_seconds;
  if (!Number.isInteger(freshness) || freshness <= 0) {
    errors.push("s6.max_status_age_seconds must be a positive integer");
  }

  const expiresAt = contract.h5?.expires_at;
  const notBefore = contract.h5?.not_before;
  if (Number.isFinite(notBefore) && Number.isFinite(expiresAt)) {
    if (notBefore >= expiresAt) {
      errors.push("h5.not_before must be earlier than h5.expires_at");
    }
    if (expiresAt < nowEpochSeconds) {
      errors.push("holder authorization is expired");
    }
  }

  const lineage = contract.policy_lineage;
  for (const field of ["authority", "version", "rule_reference", "effective_from"]) {
    if (!lineage?.[field]) {
      errors.push(`policy_lineage.${field} is required`);
    }
  }

  return {
    valid: errors.length === 0,
    errors,
    checked_at: nowEpochSeconds
  };
}
