use crate::templates::schedule::Schedule;
use anyhow::{anyhow, bail, Result};
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;

/// A client-side Supabase client.
// TODO(post-mvp) Better error handling (this panics if the user is offline)
pub static SUPABASE: Lazy<Supabase> =
    Lazy::new(|| Supabase::new().expect("couldn't connect to database"));

/// A representation of a Supabase client that does what we want. This maintains no internal state, and rather interoperates directly with JS using direct code
/// evaluation. This expects `window.supabase` to be the Supabase API.
pub struct Supabase {}
impl Supabase {
    /// Creates a new Supabase client. This will use the API information defined in `app/.supabase_config`, which should contain the anonymous key on the first
    /// line, and the API URL on the second line.
    pub fn new() -> Result<Self> {
        // We're using RLS to allow `SELECT` access to the schedule only to authenticated users
        let raw_cfg = include_str!("../.supabase_config");
        let mut cfg_lines = raw_cfg.lines();
        let anon_key = cfg_lines
            .next()
            .expect("supabase anon key should be present")
            .to_string();
        let url = cfg_lines
            .next()
            .expect("supabase url should be present")
            .to_string();

        create_client(&url, &anon_key)
            .map_err(|_| anyhow!("failed to instantiate supabase client"))?;

        Ok(Self {})
    }
    /// Signs a user in using a magic link sent to their email.
    pub async fn sign_in(&self, email: &str) -> Result<()> {
        sign_in(email)
            .await
            .map_err(|_| anyhow!("failed to log user in"))?;

        Ok(())
    }
    /// Determines whether or not the user is currently logged in.
    pub async fn is_logged_in(&self) -> bool {
        get_session().await.is_ok()
    }
    /// Gets the private schedule data from Supabase. This is only possible if the user is logged in. Unauthenticated users
    /// will be provided a response of zero rows, which will be registered as an error by this function.
    pub async fn fetch_schedule(&self) -> Result<Schedule> {
        let schedule_data = fetch_schedule()
            .await
            .map_err(|_| anyhow!("failed to fetch private schedule data"))?;

        let schedule = serde_wasm_bindgen::from_value(schedule_data)
            .map_err(|_| anyhow!("failed to parse private schedule data (are you logged in?)"))?;

        Ok(schedule)
    }
}

#[wasm_bindgen(module = "/src/supabase.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    fn create_client(url: &str, anon_key: &str) -> Result<(), JsValue>;
    #[wasm_bindgen(catch)]
    async fn sign_in(email: &str) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    async fn get_session() -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    async fn fetch_schedule() -> Result<JsValue, JsValue>;
}
