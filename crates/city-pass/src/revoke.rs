use crate::domain::CityPassBinding;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RevocationRecord {
    pub capability_id: String,
    pub revoked_at_utc: String,
    pub reason: String,
}

pub trait CityPassRevocationStore {
    fn is_revoked(&self, capability_id: &str) -> bool;
}

pub fn is_binding_revoked<S: CityPassRevocationStore>(
    store: &S,
    binding: &CityPassBinding,
) -> bool {
    store.is_revoked(&binding.capability_id)
}
