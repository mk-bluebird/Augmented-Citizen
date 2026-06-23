#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BindingId(pub i64);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CityPassDomain {
    OfflineTransit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityPassCapability {
    pub cap_id: String,
    pub domain: CityPassDomain,
    pub owner_did: String,
    pub owner_bostrom: String,
    pub eco_contract_id: String,
    pub neurorights_contract_id: String,
    pub city_id: String,
    pub route_corridor_id: String,
    pub validity_start_utc: String,
    pub validity_end_utc: String,
    pub max_taps: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityPassBinding {
    pub host_binding: BindingId,
    pub capability_id: String,
    pub issued_at_utc: String,
    pub remaining_taps: u32,
    pub sealed: bool,
}
