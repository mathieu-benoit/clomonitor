use super::datasource::scorecard;
use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;

/// Check identifier.
pub(crate) const ID: CheckId = "code_review";

/// Check score weight.
pub(crate) const WEIGHT: usize = 3;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Code];

/// Check main function.
#[allow(clippy::unnecessary_wraps)]
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    Ok(scorecard::get_check(&input.scorecard, ID).into())
}
