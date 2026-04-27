//! Login and registration page

use leptos::prelude::*;
use leptos_router::*;
use crate::state::{AuthState, LoginResponse};
use crate::api;

#[component]
pub fn LoginPage() -> impl IntoView {
    let is_login_mode = RwSignal::new(true);
    let error_msg = RwSignal::new(None::<String>);
    let loading = RwSignal::new(false);

    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let username = RwSignal::new(String::new());
    let full_name = RwSignal::new(String::new());

    let auth_state = use_context::<AuthState>().unwrap();

    let toggle_mode = move |_| {
        is_login_mode.update(|m| *m = !*m);
        error_msg.set(None);
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        error_msg.set(None);
        loading.set(true);

        let mode = is_login_mode.get();

        if mode {
            let email_val = email.get();
            let password_val = password.get();

            leptos::task::spawn_local(async move {
                #[cfg(feature = "ssr")]
                {
                    match api::login(email_val, password_val).await {
                        Ok(resp) => {
                            auth_state.token.set(Some(resp.access_token));
                            auth_state.user.set(Some(resp.user));
                            let navigate = leptos_router::hooks::use_navigate();
                            navigate("/dashboard", NavigateOptions::default());
                        }
                        Err(e) => {
                            error_msg.set(Some(e.to_string()));
                        }
                    }
                }
                loading.set(false);
            });
        } else {
            let username_val = username.get();
            let email_val = email.get();
            let password_val = password.get();
            let full_name_val = if full_name.get().is_empty() { None } else { Some(full_name.get()) };

            leptos::task::spawn_local(async move {
                #[cfg(feature = "ssr")]
                {
                    match api::register(username_val, email_val, password_val, full_name_val).await {
                        Ok(_) => {
                            error_msg.set(Some("注册成功，请登录".to_string()));
                            is_login_mode.set(true);
                        }
                        Err(e) => {
                            error_msg.set(Some(e.to_string()));
                        }
                    }
                }
                loading.set(false);
            });
        }
    };

    view! {
        <div style="height:100vh;display:flex;align-items:center;justify-content:center;background:#f5f6f8">
            <div style="width:360px;padding:32px;background:#fff;border:1px solid #e5e7eb;border-radius:8px">
                <h1 style="font-size:22px;font-weight:600;margin-bottom:24px;text-align:center">
                    {move || if is_login_mode.get() { "登录" } else { "注册" }}
                </h1>

                <Show when=move || error_msg.get().is_some()>
                    <div style="color:#e53935;font-size:13px;margin-bottom:12px;text-align:center">
                        {move || error_msg.get().unwrap_or_default()}
                    </div>
                </Show>

                <form on:submit=on_submit>
                    <Show when=move || !is_login_mode.get()>
                        <div style="margin-bottom:16px">
                            <input type="text" placeholder="用户名" bind:value=username
                                style="width:100%;padding:12px;border:1px solid #e5e7eb;border-radius:8px" />
                        </div>
                        <div style="margin-bottom:16px">
                            <input type="text" placeholder="姓名（可选）" bind:value=full_name
                                style="width:100%;padding:12px;border:1px solid #e5e7eb;border-radius:8px" />
                        </div>
                    </Show>

                    <div style="margin-bottom:16px">
                        <input type="email" placeholder="邮箱" required=true bind:value=email
                            style="width:100%;padding:12px;border:1px solid #e5e7eb;border-radius:8px" />
                    </div>

                    <div style="margin-bottom:16px">
                        <input type="password" placeholder="密码（至少8位）" required=true bind:value=password
                            style="width:100%;padding:12px;border:1px solid #e5e7eb;border-radius:8px" />
                    </div>

                    <button type="submit" disabled=move || loading.get()
                        style="width:100%;padding:12px;background:#111;color:#fff;border:none;border-radius:8px">
                        {move || if loading.get() { "处理中..." } else if is_login_mode.get() { "登录" } else { "注册" }}
                    </button>
                </form>

                <div style="margin-top:16px;text-align:center;font-size:12px;color:#888">
                    <span on:click=toggle_mode style="color:#111;cursor:pointer;text-decoration:underline">
                        {move || if is_login_mode.get() { "没有账号？注册" } else { "已有账号？登录" }}
                    </span>
                    <br/><br/>
                    "© 2026 InteleBase"
                </div>
            </div>
        </div>
    }
}