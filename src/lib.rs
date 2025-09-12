mod domain;
mod application;
mod infra;

pub use application::game_service::GameService;
pub use infra::wasm_bindings::*;
