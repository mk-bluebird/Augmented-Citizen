// crates/neuro-content-safety/src/lib.rs
pub fn trust_score(transparency: f64, surveillance: f64) -> f64 {
    let c = transparency.clamp(0.0, 1.0);
    let s = surveillance.clamp(0.0, 1.0);
    c * (1.0 - s)
}
