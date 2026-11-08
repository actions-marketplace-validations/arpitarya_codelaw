//! Fixes: the edit primitives, the apply loop and the verifier.

mod apply;
mod external;
mod primitives;
mod structured;
mod transaction;

pub use apply::{Candidate, Failure, LoopEnd, LoopOutcome, LoopTrace, MAX_PASSES, TraceEvent, apply_edits, fix_loop, fix_loop_traced, fix_loop_with, overlaps};
pub use external::{ExternalCall, request_json, run_external};
pub use structured::{DATA_PRIMITIVES, DataFixContext, DataStep, data_edits, parse_data_fix};
pub use transaction::{FileWrite, WriteAllError, write_all_or_nothing, write_with};
pub use primitives::{ChoiceSpec, FixContext, FixSpec, MoveTo, Place, Primitive, Target, choice_edits, edits_for, group_edits, parse_fix};

#[cfg(test)]
mod tests;
