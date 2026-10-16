use crate::domain::question::ContestId;
use crate::Question;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Submission {
    pub id: SubmissionId,
    pub contestId: ContestId,
    pub creationTimeSeconds : u64,
    #[serde(rename = "problem")]
    pub question: Question,
    pub verdict: Veredict,
}

#[derive(Deserialize, Debug)]
pub struct SubmissionId(u32);

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Veredict {
    Failed,
    Ok,
    Partial,
    CompilationError,
    RuntimeError,
    WrongAnswer,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    IdlenessLimitExceeded,
    SecurityViolated,
    Crashed,
    InputPreparationCrashed,
    Challenged,
    Skipped,
    Testing,
    Rejected,
    Submitted,
}
