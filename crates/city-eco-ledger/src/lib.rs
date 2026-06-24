// crates/city-eco-ledger/src/lib.rs
#![forbid(unsafe_code)]

use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct TapEnergySample {
    pub kiosk_id: String,
    pub tap_id: String,
    pub ts_utc: SystemTime,
    pub e_online_wh: f64,
    pub e_offline_wh: f64,
}

#[derive(Debug, Clone)]
pub struct CityEcoAccumulator {
    pub city_id: String,
    pub date_ymd: String,
    pub e_city_wh: f64,
}

impl CityEcoAccumulator {
    pub fn new(city_id: String, date_ymd: String) -> Self {
        Self {
            city_id,
            date_ymd,
            e_city_wh: 0.0,
        }
    }

    pub fn apply_tap(&mut self, sample: &TapEnergySample) {
        let delta = sample.e_online_wh - sample.e_offline_wh;
        if delta > 0.0 {
            self.e_city_wh += delta;
        }
    }

    pub fn e_city_kwh(&self) -> f64 {
        self.e_city_wh / 1000.0
    }
}
