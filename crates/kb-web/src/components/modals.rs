//! Modal components

use leptos::prelude::*;
use leptos_router::*;
use crate::state::{AuthState, KbState, KnowledgeBase, KbMember};
use crate::api;

#[component]
pub fn CreateKbModal(show: RwSignal<bool>) -> impl IntoView {
    let auth_state = use_context::<AuthState>().unwrap();
    let kb_state = use_context::<KbState>().unwrap();

    let kb_name = RwSignal::new(String::new());
    let kb_desc = RwSignal::new(String::new());
    let loading = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    let close_modal = move |_| {
        show.set(false);
        kb_name.set(String::new());
        kb_desc.set(String::new());
        error_msg.set(None);
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        error_msg.set(None);
        loading.set(true);

        let name = kb_name.get();
        let desc = if kb_desc.get().is_empty() { None } else { Some(kb_desc.get()) };

        let token = auth_state.token.read();
        if let Some(t) = token.as_ref() {
            leptos::task::spawn_local(async move {
                #[cfg(feature = "ssr")]
                {
                    match api::create_kb(name, desc, t).await {
                        Ok(kb) => {
                            kb_state.list.update(|list| list.push(kb));
                            close_modal(None);
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
        <Show when=move || show.get()>
            <div style="position:fixed;inset:0;background:rgba(0,0,0,0.4);display:flex;align-items:center;justify-content:center">
                <div style="width:360px;padding:32px;background:#fff;border-radius:12px">
                    <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:24px">
                        <div style="font-size:18px;font-weight:600">"创建知识库"</div>
                        <button on:click=close_modal style="font-size:24px;color:#888;cursor:pointer;background:none;border:none">"×"</button>
                    </div>

                    <Show when=move || error_msg.get().is_some()>
                        <div style="color:#e53935;font-size:13px;margin-bottom:12px;text-align:center">
                            {move || error_msg.get().unwrap_or_default()}
                        </div>
                    </Show>

                    <form on:submit=on_submit>
                        <div style="margin-bottom:16px">
                            <input type="text" placeholder="知识库名称" required=true bind:value=kb_name
                                style="width:100%;padding:12px;border:1px solid #e5e7eb;border-radius:8px" />
                        </div>
                        <div style="margin-bottom:16px">
                            <input type="text" placeholder="描述（可选）" bind:value=kb_desc
                                style="width:100%;padding:12px;border:1px solid #e5e7eb;border-radius:8px" />
                        </div>
                        <button type="submit" disabled=move || loading.get()
                            style="width:100%;padding:12px;background:#111;color:#fff;border:none;border-radius:8px">
                            {move || if loading.get() { "创建中..." } else { "创建" }}
                        </button>
                    </form>
                </div>
            </div>
        </Show>
    }
}

#[component]
pub fn AddMemberModal(show: RwSignal<bool>, kb_id: impl Fn() -> i64 + 'static) -> impl IntoView {
    let auth_state = use_context::<AuthState>().unwrap();
    let kb_state = use_context::<KbState>().unwrap();

    let user_id = RwSignal::new(String::new());
    let role = RwSignal::new("viewer".to_string());
    let loading = RwSignal::new(false);
    let error_msg = RwSignal::new(None::<String>);

    let close_modal = move |_| {
        show.set(false);
        user_id.set(String::new());
        role.set("viewer".to_string());
        error_msg.set(None);
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        error_msg.set(None);
        loading.set(true);

        let uid = user_id.get().parse::<i64>().unwrap_or(0);
        let role_val = role.get();
        let id = kb_id();

        let token = auth_state.token.read();
        if let Some(t) = token.as_ref() {
            leptos::task::spawn_local(async move {
                #[cfg(feature = "ssr")]
                {
                    match api::add_member(id, uid, role_val, t).await {
                        Ok(member) => {
                            kb_state.members.update(|list| list.push(member));
                            close_modal(None);
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
        <Show when=move || show.get()>
            <div style="position:fixed;inset:0;background:rgba(0,0,0,0.4);display:flex;align-items:center;justify-content:center">
                <div style="width:360px;padding:32px;background:#fff;border-radius:12px">
                    <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:24px">
                        <div style="font-size:18px;font-weight:600">"添加成员"</div>
                        <button on:click=close_modal style="font-size:24px;color:#888;cursor:pointer;background:none;border:none">"×"</button>
                    </div>

                    <Show when=move || error_msg.get().is_some()>
                        <div style="color:#e53935;font-size:13px;margin-bottom:12px;text-align:center">
                            {move || error_msg.get().unwrap_or_default()}
                        </div>
                    </Show>

                    <form on:submit=on_submit>
                        <div style="margin-bottom:16px">
                            <input type="number" placeholder="用户ID" required=true bind:value=user_id
                                style="width:100%;padding:12px;border:1px solid #e5e7eb;border-radius:8px" />
                        </div>
                        <div style="margin-bottom:16px">
                            <select bind:value=role style="width:100%;padding:12px;border:1px solid #e5e7eb;border-radius:8px">
                                <option value="viewer">"查看者"</option>
                                <option value="editor">"编辑者"</option>
                                <option value="admin">"管理员"</option>
                            </select>
                        </div>
                        <button type="submit" disabled=move || loading.get()
                            style="width:100%;padding:12px;background:#111;color:#fff;border:none;border-radius:8px">
                            {move || if loading.get() { "添加中..." } else { "添加" }}
                        </button>
                    </form>
                </div>
            </div>
        </Show>
    }
}