use actix_web::{HttpResponse, Responder, get, web::Data};
use ream_beacon_api_types::{
    error::ApiError,
    responses::{DataResponse, EXECUTION_OPTIMISTIC},
    sync::SyncStatus,
};
use ream_execution_engine::ExecutionEngine;
use ream_fork_choice::store::Store;
use ream_storage::{db::ReamDB, tables::Table};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Serialize, Deserialize, Default)]
pub struct Syncing {
    sync_status: SyncStatus,
}

impl Syncing {
    pub fn new(head_slot: u64, sync_distance: u64, el_offline: bool, is_syncing: bool) -> Self {
        Self {
            sync_status: SyncStatus {
                head_slot,
                sync_distance,
                is_syncing,
                el_offline,
                is_optimistic: EXECUTION_OPTIMISTIC,
            },
        }
    }
}

/// Called by `eth/v1/node/syncing` to get the Node Version.
#[get("/node/syncing")]
pub async fn get_syncing_status(
    db: Data<ReamDB>,
    execution_engine: Data<ExecutionEngine>,
) -> Result<impl Responder, ApiError> {
    let store = Store {
        db: db.get_ref().clone(),
    };

    // get head_slot
    let head = store.get_head().map_err(|err| {
        error!("Failed to get current slot, error: {err:?}");
        ApiError::InternalError
    })?;

    let head_slot = match db.beacon_block_provider().get(head) {
        Ok(Some(block)) => block.message.slot,
        err => {
            error!("Failed to get head slot, error: {err:?}");
            return Err(ApiError::InternalError);
        }
    };

    // calculate sync_distance
    let current_slot = store.get_current_slot().map_err(|err| {
        error!("Failed to get current slot, error: {err:?}");
        ApiError::InternalError
    })?;

    let sync_distance = current_slot.saturating_sub(head_slot);

    // get el_offline
    let el_offline = match execution_engine.eth_chain_id().await {
        Ok(_) => false,
        Err(err) => {
            error!("Execution engine is offline or erroring, error: {err:?}");
            true
        }
    };

    Ok(HttpResponse::Ok().json(DataResponse::new(Syncing::new(
        head_slot,
        sync_distance,
        el_offline,
        // get is_syncing
        sync_distance > 1,
    ))))
}
