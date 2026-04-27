//! Knowledge base detail page

use leptos::prelude::*;
use leptos_router::*;
use crate::state::{AuthState, KbState, KbMember};
use crate::components::{Nav, AddMemberModal};
use crate::api;

#[component]
pub fn KbDetailPage() -> impl IntoView {
    let auth_state = use_context::<AuthState>().unwrap();
    let kb_state = use_context::<KbState>().unwrap();
    let navigate = leptos_router::hooks::use_navigate();

    let params = leptos_router::hooks::use_params_map();
    let kb_id = move || params.get().get("id").and_then(|s: &String| s.parse::<i64>().ok()).unwrap_or(0);

    let show_add_modal = RwSignal::new(false);
    let loading = RwSignal::new(true);
    let error_msg = RwSignal::new(None::<String>);

    Effect::new(move |_| {
        if !auth_state.is_authenticated() {
            navigate("/", NavigateOptions::default());
            return;
        }

        let id = kb_id();
        let token = auth_state.token.read();
        if let Some(t) = token.as_ref() {
            leptos::task::spawn_local(async move {
                #[cfg(feature = "ssr")]
                {
                    match api::get_kb(id, t).await {
                        Ok(kb) => kb_state.current_kb.set(Some(kb)),
                        Err(e) => error_msg.set(Some(e.to_string())),
                    }
                    match api::list_members(id, t).await {
                        Ok(members) => kb_state.members.set(members),
                        Err(e) => error_msg.set(Some(e.to_string())),
                    }
                }
                loading.set(false);
            });
        }
    });

    let remove_member = move |user_id: i64| {
        let id = kb_id();
        let token = auth_state.token.read();
        if let Some(t) = token.as_ref() {
            leptos::task::spawn_local(async move {
                #[cfg(feature = "ssr")]
                {
                    if api::remove_member(id, user_id, t).await.is_ok() {
                        if let Some(t) = auth_state.token.read().as_ref() {
                            match api::list_members(id, t).await {
                                Ok(members) => kb_state.members.set(members),
                                _ => {}
                            }
                        }
                    }
                }
            });
        }
    };

    view! {
        <Nav />
        <AddMemberModal show=show_add_modal kb_id=kb_id />

        <main style="max-width:1200px;margin:24px auto;padding:0 24px">
            <Show when=move || loading.get()>
                <div style="text-align:center;padding:40px;color:#888">"加载中..."</div>
            </Show>

            <Show when=move || error_msg.get().is_some()>
                <div style="text-align:center;padding:24px;color:#e53935">
                    {move || error_msg.get().unwrap_or_default()}
                </div>
            </Show>

            <Show when=move || !loading.get() && kb_state.current_kb.get().is_some()>
                {move || {
                    let kb = kb_state.current_kb.get().unwrap();
                    view! {
                        <div style="margin-bottom:24px">
                            <h1 style="font-size:24px;font-weight:600">{kb.name.clone()}</h1>
                            <p style="font-size:14px;color:#666">{kb.description.clone().unwrap_or_default()}</p>
                        </div>

                        <div style="background:#fff;border:1px solid #e5e7eb;border-radius:8px;padding:24px">
                            <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:16px">
                                <div style="font-size:16px;font-weight:600">"成员管理"</div>
                                <button on:click=move |_| show_add_modal.set(true)
                                    style="padding:10px 20px;background:#111;color:#fff;border:none;border-radius:8px">
                                    "添加成员"
                                </button>
                            </div>

                            <Show when=move || kb_state.members.read().is_empty()>
                                <div style="text-align:center;padding:24px;color:#888">"暂无成员"</div>
                            </Show>

                            <Show when=move || !kb_state.members.read().is_empty()>
                                <table style="width:100%;border-collapse:collapse">
                                    <thead>
                                        <tr>
                                            <th style="padding:12px;text-align:left;font-size:13px;color:#888;border-bottom:1px solid #e5e7eb">"用户名"</th>
                                            <th style="padding:12px;text-align:left;font-size:13px;color:#888;border-bottom:1px solid #e5e7eb">"邮箱"</th>
                                            <th style="padding:12px;text-align:left;font-size:13px;color:#888;border-bottom:1px solid #e5e7eb">"角色"</th>
                                            <th style="padding:12px;text-align:left;font-size:13px;color:#888;border-bottom:1px solid #e5e7eb">"操作"</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {move || kb_state.members.get().into_iter().map(|m| {
                                            view! {
                                                <tr>
                                                    <td style="padding:12px;font-size:14px;border-bottom:1px solid #e5e7eb">{m.username.clone()}</td>
                                                    <td style="padding:12px;font-size:14px;border-bottom:1px solid #e5e7eb">{m.email.clone()}</td>
                                                    <td style="padding:12px;font-size:14px;border-bottom:1px solid #e5e7eb">{m.role_name.clone()}</td>
                                                    <td style="padding:12px;border-bottom:1px solid #e5e7eb">
                                                        <button on:click=move |_| {
                                                                #[cfg(feature = "ssr")]
                                                                {
                                                                    if web_sys::window().unwrap().confirm_with_message("确定移除？").unwrap() {
                                                                        remove_member(m.user_id);
                                                                    }
                                                                }
                                                            }
                                                            style="padding:6px 12px;background:#fee2e2;color:#dc2626;border:none;border-radius:6px">
                                                            "移除"
                                                        </button>
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tbody>
                                </table>
                            </Show>
                        </div>
                    }
                }}
            </Show>

            <Show when=move || !loading.get() && kb_state.current_kb.get().is_none()>
                <div style="text-align:center;padding:24px;color:#888">"知识库不存在"</div>
            </Show>
        </main>
    }
}