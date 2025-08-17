use crate::{
    handlers::transactions::{
        create_transaction, get_detail_transaction, get_transactions, update_transaction,
    },
    state::AppState,
};
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_transaction).get(get_transactions))
        .route(
            "/{id}",
            get(get_detail_transaction).patch(update_transaction),
        )
}
