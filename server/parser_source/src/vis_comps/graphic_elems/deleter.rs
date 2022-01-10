use serde::{Serialize};
use super::super::unique_id_generator::UID;

#[derive(Debug, Clone, Serialize)]
pub struct ElemDeleter {
    #[serde(flatten)]
    pub (in super) unique_id: UID
}
