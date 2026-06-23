// filename: crates/neurorights-evidence/src/lib.rs
// License: MIT OR Apache-2.0
#![forbid(unsafe_code)]

#[derive(Clone, Debug)]
pub enum NeurorightsScope {
    Constitutional,
    Treaty,
    Guideline,
    Commentary,
}

#[derive(Clone, Debug)]
pub struct NeurorightsSource {
    pub evidid: &'static str,
    pub name: &'static str,
    pub url: &'static str,
    pub scope: NeurorightsScope,
    pub jurisdiction: &'static str,
    pub summary: &'static str,
}

pub static NEURORIGHTS_SOURCES: &[NeurorightsSource] = &[
    NeurorightsSource {
        evidid: "neurorights.foundation.5rights.v1",
        name: "NeuroRights Foundation – Five Core Neurorights",
        url: "https://neurorightsfoundation.org",
        scope: NeurorightsScope::Guideline,
        jurisdiction: "Global",
        summary: "Defines mental privacy, personal identity, free will, fair access, and protection from bias as neurorights baselines.",
    },
    NeurorightsSource {
        evidid: "columbia.nri.v1",
        name: "Columbia NeuroRights Initiative",
        url: "https://nri.columbia.edu",
        scope: NeurorightsScope::Guideline,
        jurisdiction: "Global",
        summary: "Academic program elaborating neurorights theory and technical translation for neurotechnology.",
    },
    NeurorightsSource {
        evidid: "chile.ley21_383.v1",
        name: "Chile Neuroprotection Law 21.383",
        url: "https://www.bcn.cl/leychile/navegar?idNorma=1166989",
        scope: NeurorightsScope::Constitutional,
        jurisdiction: "Chile",
        summary: "First constitutional-level neurorights law protecting brain data and mental integrity.",
    },
    NeurorightsSource {
        evidid: "unesco.neuroethics.v1",
        name: "UNESCO Neuroethics Framework",
        url: "https://en.unesco.org/themes/ethics-science-and-technology/neuroethics",
        scope: NeurorightsScope::Treaty,
        jurisdiction: "UNESCO",
        summary: "Global ethical recommendations for neuroscience and neurotechnology governance.",
    },
    NeurorightsSource {
        evidid: "ieee.neuroethics.v1",
        name: "IEEE Neuroethics Framework",
        url: "https://standards.ieee.org/initiatives/neuroethics/index.html",
        scope: NeurorightsScope::Guideline,
        jurisdiction: "Global",
        summary: "Engineering-focused guidance on ethical design and deployment of neurotechnologies.",
    },
];

#[derive(Clone, Debug)]
pub struct NeurorightsEnvelope {
    pub noscorefrominnerstate: bool,
    pub noexclusionbasicservices: bool,
    pub noneurocoercion: bool,
    pub revocableatwill: bool,
    pub augmentationcontinuity: bool,
    pub projectcontinuity: bool,
    pub legal_bases: &'static [&'static str],
}

pub const NEURORIGHTS_ENVELOPE_CITIZEN_V1_ID: &str = "neurorights.envelope.citizen.v1";

pub static NEURORIGHTS_ENVELOPE_CITIZEN_V1: NeurorightsEnvelope = NeurorightsEnvelope {
    noscorefrominnerstate: true,
    noexclusionbasicservices: true,
    noneurocoercion: true,
    revocableatwill: true,
    augmentationcontinuity: true,
    projectcontinuity: true,
    legal_bases: &[
        "neurorights.foundation.5rights.v1",
        "columbia.nri.v1",
        "chile.ley21_383.v1",
        "unesco.neuroethics.v1",
        "ieee.neuroethics.v1",
    ],
};

pub fn find_source(evidid: &str) -> Option<&'static NeurorightsSource> {
    NEURORIGHTS_SOURCES.iter().find(|s| s.evidid == evidid)
}

pub fn validate_envelope(envelope: &NeurorightsEnvelope) -> bool {
    if !envelope.noscorefrominnerstate {
        return false;
    }
    if !envelope.noexclusionbasicservices {
        return false;
    }
    if !envelope.noneurocoercion {
        return false;
    }
    if !envelope.revocableatwill {
        return false;
    }
    if envelope.legal_bases.is_empty() {
        return false;
    }
    envelope
        .legal_bases
        .iter()
        .all(|id| find_source(id).is_some())
}
