//! kb-web - Leptos frontend for InteleBase

pub mod app;
pub mod components;
pub mod api;
pub mod state;

pub use app::App;

// Re-export leptos prelude (includes spawn_local, RwSignal, Effect, etc.)
pub use leptos::prelude::*;

// leptos_router exports
pub use leptos_router::{BrowserRouter, NavigateOptions, ParamsMap};

#[cfg(feature = "ssr")]
pub use leptos_axum::{LeptosRoutes, generate_route_list};