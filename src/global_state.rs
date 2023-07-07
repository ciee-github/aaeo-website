use perseus::{engine_only_fn, state::GlobalStateCreator, ReactiveState};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ReactiveState, Clone)]
#[rx(alias = "AppStateRx")]
pub struct AppState {
    /// Whether or not the user is logged in. As the actual authentication is managed almost solely by Supabase in JS, we don't
    /// have to care about this very much. If the user fudges this, they still won't be able to retrieve any access-controlled
    /// information.
    authenticated: bool,
}

pub fn get_gsc() -> GlobalStateCreator {
    GlobalStateCreator::new().build_state_fn(get_build_state)
}

#[engine_only_fn]
async fn get_build_state() -> AppState {
    // The app is designed not to do too bad of a flash here
    AppState {
        authenticated: false,
    }
}
