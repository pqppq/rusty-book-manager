use axum::{
    routing::{delete, get, put},
    Router,
};
use registry::AppRegistry;

use crate::handler::user::{
    change_password, change_role, delete_user, get_checkouts, get_current_user, list_users,
    register_user,
};

pub fn build_user_router() -> Router<AppRegistry> {
    let users_routers = Router::new()
        .route("/", get(list_users).post(register_user))
        .route("/:user_id", delete(delete_user))
        .route("/:user_id/role", put(change_role))
        .route("/me", get(get_current_user))
        .route("/me/checkout", get(get_checkouts))
        .route("/me/password", put(change_password));

    Router::new().nest("/users", users_routers)
}
