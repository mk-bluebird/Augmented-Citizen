use crate::domain::{BindingId, CityPassBinding, CityPassCapability};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CityPassError {
    AlreadySealed,
    Expired,
    NoRemainingTaps,
    HostMismatch,
    InvalidTimestamps,
    ZeroDuration,
}

fn parse_time(ts: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(ts).ok().map(|dt| dt.with_timezone(&Utc))
}

pub fn is_expired(cap: &CityPassCapability, now_utc: &str) -> bool {
    match (parse_time(&cap.validity_start_utc),
           parse_time(&cap.validity_end_utc),
           parse_time(now_utc)) {
        (Some(start), Some(end), Some(now)) => now < start || now > end,
        _ => true,
    }
}

pub fn issue_offline_pass(
    binding_id: BindingId,
    cap: &CityPassCapability,
    now_utc: &str,
    existing: Option<CityPassBinding>,
) -> Result<CityPassBinding, CityPassError> {
    if let Some(b) = existing {
        if b.sealed {
            return Err(CityPassError::AlreadySealed);
        }
    }
    let start = parse_time(&cap.validity_start_utc).ok_or(CityPassError::InvalidTimestamps)?;
    let end = parse_time(&cap.validity_end_utc).ok_or(CityPassError::InvalidTimestamps)?;
    let now = parse_time(now_utc).ok_or(CityPassError::InvalidTimestamps)?;
    if end <= start {
        return Err(CityPassError::ZeroDuration);
    }
    if now < start || now > end {
        return Err(CityPassError::Expired);
    }
    if cap.max_taps == 0 {
        return Err(CityPassError::NoRemainingTaps);
    }

    Ok(CityPassBinding {
        host_binding: binding_id,
        capability_id: cap.cap_id.clone(),
        issued_at_utc: now_utc.to_owned(),
        remaining_taps: cap.max_taps,
        sealed: true,
    })
}

pub fn verify_tap(
    host_binding: BindingId,
    cap: &CityPassCapability,
    binding: &mut CityPassBinding,
    now_utc: &str,
) -> Result<(), CityPassError> {
    if binding.host_binding != host_binding {
        return Err(CityPassError::HostMismatch);
    }
    if is_expired(cap, now_utc) {
        return Err(CityPassError::Expired);
    }
    if binding.remaining_taps == 0 {
        return Err(CityPassError::NoRemainingTaps);
    }
    binding.remaining_taps -= 1;
    Ok(())
}
