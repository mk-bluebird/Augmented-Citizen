// crates/state-only-channel/src/lib.rs

#![forbid(unsafe_code)]

mod state_only_vec;
mod pain_index;

pub use state_only_vec::StateOnlyVec16;
pub use pain_index::PainIndexState;
