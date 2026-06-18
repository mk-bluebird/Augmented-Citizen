// anti-coercion-enclave/src/state_machine.rs (extend)

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ConsentVerdict {
    Valid,
    Invalid,
    CoercionSuspect,
    UnderDuress,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccessLevel {
    Allow,
    Restricted,
    Deny,
}

impl ConsentVerdict {
    pub fn access_level(self) -> AccessLevel {
        match self {
            ConsentVerdict::Valid => AccessLevel::Allow,
            ConsentVerdict::UnderDuress => AccessLevel::Restricted,
            ConsentVerdict::CoercionSuspect => AccessLevel::Restricted,
            ConsentVerdict::Invalid => AccessLevel::Deny,
        }
    }
}
