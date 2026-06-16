// destination-path: crates/sovereign-guards-core/src/lifeforce_ota_guard.rs

#![cfg_attr(not(test), deny(warnings))]
#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

/// Scalar summaries of host biophysical state used for OTA eligibility.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HostBioState {
    pub lifeforce: f64,        // 0.0..1.0 normalized lifeforce reserve
    pub blood_tokens: f64,     // 0.0..1.0 normalized BLOOD corridor
    pub protein_reserve: f64,  // 0.0..1.0 normalized PROTEIN corridor
    pub roh: f64,              // Risk-of-Harm scalar V(x)
    pub core_temp_c: f64,      // Core body temperature in Celsius
    pub tissue_delta_c: f64,   // Local tissue delta-T in Celsius
}

/// Scalar summaries of BCI and nanoswarm duty relevant to OTA.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HostDutyState {
    pub bci_duty_fraction: f64,   // 0.0..1.0 fraction of allowed BCI duty
    pub daily_ota_steps: u32,     // number of OTA windows already used today
    pub ota_energy_joules: f64,   // total Joules planned for OTA window
    pub ml_pass_fraction: f64,    // 0.0..1.0 ML duty fraction for OTA
}

/// Thresholds parsed from the ALN shard aln-cybernano-lifeforce-ota.v1.aln.
/// These are host-local and bound to mk-bluebird/Augmented-Citizen.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LifeforceOtaThresholds {
    pub lifeforce_floor: f64,
    pub blood_floor: f64,
    pub protein_floor: f64,
    pub roh_max: f64,
    pub core_temp_max_c: f64,
    pub tissue_delta_max_c: f64,
    pub bci_duty_max: f64,
    pub daily_ota_steps_max: u32,
    pub ota_energy_budget_max_j: f64,
    pub ml_pass_fraction_max: f64,
}

/// Verdict returned by the guard. This is host-local and monotone:
/// it can only veto or allow; it never weakens thresholds.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OtaEligibilityVerdict {
    Eligible,
    RejectedLifeforceFloor,
    RejectedBloodFloor,
    RejectedProteinFloor,
    RejectedRiskOfHarm,
    RejectedThermalCore,
    RejectedTissueDelta,
    RejectedBciDuty,
    RejectedDailyOtaSteps,
    RejectedOtaEnergyBudget,
    RejectedMlPassFraction,
}

/// Core guard function: checks OTA eligibility against lifeforce/blood/thermal
/// corridors and BCI duty ceilings.
///
/// Mathematically, OTA is allowed only if all inequalities hold:
/// - lifeforce >= lifeforce_floor
/// - blood_tokens >= blood_floor
/// - protein_reserve >= protein_floor
/// - roh <= roh_max
/// - core_temp_c <= core_temp_max_c
/// - tissue_delta_c <= tissue_delta_max_c
/// - bci_duty_fraction <= bci_duty_max
/// - daily_ota_steps <= daily_ota_steps_max
/// - ota_energy_joules <= ota_energy_budget_max_j
/// - ml_pass_fraction <= ml_pass_fraction_max
pub fn evaluate_ota_eligibility(
    bio: HostBioState,
    duty: HostDutyState,
    t: LifeforceOtaThresholds,
) -> OtaEligibilityVerdict {
    if bio.lifeforce < t.lifeforce_floor {
        return OtaEligibilityVerdict::RejectedLifeforceFloor;
    }

    if bio.blood_tokens < t.blood_floor {
        return OtaEligibilityVerdict::RejectedBloodFloor;
    }

    if bio.protein_reserve < t.protein_floor {
        return OtaEligibilityVerdict::RejectedProteinFloor;
    }

    if bio.roh > t.roh_max {
        return OtaEligibilityVerdict::RejectedRiskOfHarm;
    }

    if bio.core_temp_c > t.core_temp_max_c {
        return OtaEligibilityVerdict::RejectedThermalCore;
    }

    if bio.tissue_delta_c > t.tissue_delta_max_c {
        return OtaEligibilityVerdict::RejectedTissueDelta;
    }

    if duty.bci_duty_fraction > t.bci_duty_max {
        return OtaEligibilityVerdict::RejectedBciDuty;
    }

    if duty.daily_ota_steps > t.daily_ota_steps_max {
        return OtaEligibilityVerdict::RejectedDailyOtaSteps;
    }

    if duty.ota_energy_joules > t.ota_energy_budget_max_j {
        return OtaEligibilityVerdict::RejectedOtaEnergyBudget;
    }

    if duty.ml_pass_fraction > t.ml_pass_fraction_max {
        return OtaEligibilityVerdict::RejectedMlPassFraction;
    }

    OtaEligibilityVerdict::Eligible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ota_allowed_when_all_inequalities_hold() {
        let bio = HostBioState {
            lifeforce: 0.80,
            blood_tokens: 0.60,
            protein_reserve: 0.50,
            roh: 0.10,
            core_temp_c: 37.2,
            tissue_delta_c: 0.5,
        };

        let duty = HostDutyState {
            bci_duty_fraction: 0.30,
            daily_ota_steps: 0,
            ota_energy_joules: 1200.0,
            ml_pass_fraction: 0.10,
        };

        let thresholds = LifeforceOtaThresholds {
            lifeforce_floor: 0.35,
            blood_floor: 0.25,
            protein_floor: 0.20,
            roh_max: 0.28,
            core_temp_max_c: 38.2,
            tissue_delta_max_c: 1.2,
            bci_duty_max: 0.55,
            daily_ota_steps_max: 1,
            ota_energy_budget_max_j: 2200.0,
            ml_pass_fraction_max: 0.20,
        };

        let verdict = evaluate_ota_eligibility(bio, duty, thresholds);
        assert_eq!(verdict, OtaEligibilityVerdict::Eligible);
    }

    #[test]
    fn ota_rejected_when_lifeforce_below_floor() {
        let bio = HostBioState {
            lifeforce: 0.20,
            blood_tokens: 0.60,
            protein_reserve: 0.50,
            roh: 0.10,
            core_temp_c: 37.1,
            tissue_delta_c: 0.4,
        };

        let duty = HostDutyState {
            bci_duty_fraction: 0.20,
            daily_ota_steps: 0,
            ota_energy_joules: 800.0,
            ml_pass_fraction: 0.05,
        };

        let thresholds = LifeforceOtaThresholds {
            lifeforce_floor: 0.35,
            blood_floor: 0.25,
            protein_floor: 0.20,
            roh_max: 0.28,
            core_temp_max_c: 38.2,
            tissue_delta_max_c: 1.2,
            bci_duty_max: 0.55,
            daily_ota_steps_max: 1,
            ota_energy_budget_max_j: 2200.0,
            ml_pass_fraction_max: 0.20,
        };

        let verdict = evaluate_ota_eligibility(bio, duty, thresholds);
        assert_eq!(verdict, OtaEligibilityVerdict::RejectedLifeforceFloor);
    }

    #[test]
    fn ota_rejected_when_roh_exceeds_ceiling() {
        let bio = HostBioState {
            lifeforce: 0.80,
            blood_tokens: 0.60,
            protein_reserve: 0.50,
            roh: 0.29,
            core_temp_c: 37.5,
            tissue_delta_c: 0.6,
        };

        let duty = HostDutyState {
            bci_duty_fraction: 0.30,
            daily_ota_steps: 0,
            ota_energy_joules: 1200.0,
            ml_pass_fraction: 0.10,
        };

        let thresholds = LifeforceOtaThresholds {
            lifeforce_floor: 0.35,
            blood_floor: 0.25,
            protein_floor: 0.20,
            roh_max: 0.28,
            core_temp_max_c: 38.2,
            tissue_delta_max_c: 1.2,
            bci_duty_max: 0.55,
            daily_ota_steps_max: 1,
            ota_energy_budget_max_j: 2200.0,
            ml_pass_fraction_max: 0.20,
        };

        let verdict = evaluate_ota_eligibility(bio, duty, thresholds);
        assert_eq!(verdict, OtaEligibilityVerdict::RejectedRiskOfHarm);
    }
}
