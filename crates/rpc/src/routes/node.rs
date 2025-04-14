use warp::{Filter, Rejection, filters::path::end, get, log, path, reply::Reply};

use crate::handlers::version::{get_syncing_status, get_version};

/// Creates and returns all `/node` routes.
pub fn get_node_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    path("node")
        .and(path("version"))
        .and(end())
        .and(get())
        .and_then(get_version)
        .and_then(get_syncing_status)
        .with(log("version"))
}
