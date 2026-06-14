// filename: browser/background.js

// This file is a minimal skeleton for wiring the browser side of Augmented-ID.
// It detects age-gated content and forwards metadata to a host-local Augmented-ID agent.
// The host agent is responsible for calling the Rust guard crate and returning a signed proof.

const AUGID_HOST_ENDPOINT = "http://127.0.0.1:8743/augmented-id/evaluate";

function isAgeGatedRequest(details) {
  const url = details.url.toLowerCase();
  if (url.includes("agecheck") || url.includes("age-verification")) {
    return true;
  }
  return false;
}

async function evaluateAugmentedId(requestDetails) {
  const requestedAgeBand = "over_18";

  const payload = {
    requested_age_band: requestedAgeBand,
    url: requestDetails.url,
    method: requestDetails.method
  };

  const response = await fetch(AUGID_HOST_ENDPOINT, {
    method: "POST",
    body: JSON.stringify(payload),
    headers: {
      "Content-Type": "application/json"
    }
  });

  if (!response.ok) {
    return null;
  }

  return await response.json();
}

chrome.webRequest.onBeforeRequest.addListener(
  async function (details) {
    if (!isAgeGatedRequest(details)) {
      return {};
    }

    try {
      const verdict = await evaluateAugmentedId(details);

      if (!verdict) {
        return { cancel: true };
      }

      if (verdict.verdict === "AutoAllowed") {
        return {};
      }

      return { cancel: true };
    } catch (_err) {
      return { cancel: true };
    }
  },
  { urls: ["<all_urls>"] },
  ["blocking"]
);
