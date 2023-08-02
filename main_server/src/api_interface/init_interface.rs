use candid::CandidType;
use serde::Deserialize;

#[derive(Debug, CandidType, Deserialize)]
pub struct InitCanisterRequest {
    pub env: String,
}
