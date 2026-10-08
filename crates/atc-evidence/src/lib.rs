#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceState {
    Implemented,
    Executed,
    Tested,
    EvidenceCollected,
    ExactShaVerified,
    Verified,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRecord {
    pub state: EvidenceState,
    pub commit_sha: String,
    pub run_id: Option<String>,
    pub exit_code: Option<i32>,
}
impl EvidenceRecord {
    pub fn exact_sha_verified(commit_sha: impl Into<String>) -> Self {
        Self {
            state: EvidenceState::ExactShaVerified,
            commit_sha: commit_sha.into(),
            run_id: None,
            exit_code: None,
        }
    }
}
