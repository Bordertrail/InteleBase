//! Navigation bar component

use leptos::prelude::*;
use leptos_router::*;
use crate::state::AuthState;

#[component]
pub fn Nav() -> impl IntoView {
    let auth_state = use_context::<AuthState>().unwrap();

    let logout = move |_| {
        auth_state.clear();
        let navigate = leptos_router::hooks::use_navigate();
        navigate("/", NavigateOptions::default());
    };

    view! {
        <nav style="background:#fff;padding:16px 24px;border-bottom:1px solid #e5e7eb;display:flex;justify-content:space-between;align-items:center">
            <div style="font-size:18px;font-weight:600">"InteleBase"</div>
            <div style="display:flex;align-items:center;gap:12px;font-size:14px;color:#666">
                {move || auth_state.user.get().map(|u| view! { <span>{u.username}</span> })}
                <button on:click=logout
                    style="padding:8px 16px;background:#f5f6f8;border:none;border-radius:6px;font-size:14px;cursor:pointer">
                    "退出"
                </button>
            </div>
        </nav>
    }
}