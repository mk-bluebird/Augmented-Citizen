pub struct SafetyEnvelope {
    pub roh: f32,
    pub anticoercion: f32,
    pub psych_debt: f32,
    pub neurorights_ok: bool,
}

pub fn verify_safety(s: &SafetyEnvelope) -> bool {
    s.roh <= 0.30 && s.anticoercion >= 0.8 && s.psych_debt < 0.7 && s.neurorights_ok
}

pub fn verify_shadowed(first_ts: u64, second_ts: u64) -> bool {
    let dt = second_ts.saturating_sub(first_ts);
    (300..=700).contains(&dt) // 400ms window
}
