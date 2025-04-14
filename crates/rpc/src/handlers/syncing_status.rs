use serde::{Deserialize, Serialize};
use warp::{
    http::status::StatusCode,
    reject::Rejection,
    reply::{Reply, with_status},
};

use super::Data;

#[derive(Serialize, Deserialize, Default)]
pub struct SyncingStatus {}

impl SyncingStatus {
    pub fn new() -> Self {
        Self {}
    }
}

/// Called by `/eth/v1/node/syncing` and requests the beacon node to describe if it's currently
/// syncing or not, and if it is, what block it is up to.
pub async fn get_syncing_status() -> Result<impl Reply, Rejection> {
    Ok(with_status(
        Data::json(SyncingStatus::new()),
        StatusCode::OK,
    ))
}
