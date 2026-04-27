//! Main application component with routing

use leptos::prelude::*;
use leptos_router::components::*;
use crate::components::LoginPage;
use crate::state::*;

#[component]
pub fn App() -> impl IntoView {
    // Provide global state
    provide_context(AuthState::default());
    provide_context(KbState::default());

    view! {
        <BrowserRouter>
            <main>
                <Routes>
                    <Route path="/" view=LoginPage />
                    <Route path="/dashboard" view=crate::components::DashboardPage />
                    <Route path="/kb/:id" view=crate::components::KbDetailPage />
                </Routes>
            </main>
        </BrowserRouter>
    }
}