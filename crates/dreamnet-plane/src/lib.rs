// crates/dreamnet-plane/src/lib.rs
pub fn dream_gaming_carbon_reduction(baseline_kwh: f64, dreamnet_kwh: f64) -> f64 {
    if baseline_kwh <= 0.0 {
        return 0.0;
    }
    1.0 - (dreamnet_kwh / baseline_kwh)
}
