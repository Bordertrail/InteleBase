//! Dashboard page - Knowledge base list

use leptos::prelude::*;
use leptos_router::*;
use crate::state::{AuthState, KbState, KnowledgeBase};
use crate::components::{Nav, CreateKbModal};
use crate::api;

#[component]
pub fn DashboardPage() -> impl IntoView {
    let auth_state = use_context::<AuthState>().unwrap();
    let kb_state = use_context::<KbState>().unwrap();
    let navigate = leptos_router::hooks::use_navigate();

    let show_create_modal = RwSignal::new(false);
    let loading = RwSignal::new(true);
    let error_msg = RwSignal::new(None::<String>);
    let current_page = RwSignal::new(1);

    Effect::new(move |_| {
        if !auth_state.is_authenticated() {
            navigate("/", NavigateOptions::default());
            return;
        }

        let token = auth_state.token.read();
        if let Some(t) = token.as_ref() {
            leptos::task::spawn_local(async move {
                #[cfg(feature = "ssr")]
                {
                    match api::list_kbs(current_page.get(), 10, t).await {
                        Ok(result) => {
                            kb_state.list.set(result.items);
                            kb_state.total_pages.set(result.total_pages);
                            error_msg.set(None);
                        }
                        Err(e) => {
                            if e.to_string().contains("Unauthorized") {
                                auth_state.clear();
                                navigate("/", NavigateOptions::default());
                            } else {
                                error_msg.set(Some(e.to_string()));
                            }
                        }
                    }
                }
                loading.set(false);
            });
        }
    });

    let delete_kb = move |kb_id: i64| {
        let token = auth_state.token.read();
        if let Some(t) = token.as_ref() {
            leptos::task::spawn_local(async move {
                #[cfg(feature = "ssr")]
                {
                    if api::delete_kb(kb_id, t).await.is_ok() {
                        if let Some(t) = auth_state.token.read().as_ref() {
                            match api::list_kbs(current_page.get(), 10, t).await {
                                Ok(result) => {
                                    kb_state.list.set(result.items);
                                    kb_state.total_pages.set(result.total_pages);
                                }
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
        <CreateKbModal show=show_create_modal />

        <main style="max-width:1200px;margin:24px auto;padding:0 24px">
            <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:24px">
                <h1 style="font-size:24px;font-weight:600">"知识库"</h1>
                <button on:click=move |_| show_create_modal.set(true)
                    style="padding:10px 20px;background:#111;color:#fff;border:none;border-radius:8px">
                    "创建知识库"
                </button>
            </div>

            <Show when=move || loading.get()>
                <div style="text-align:center;padding:40px;color:#888">"加载中..."</div>
            </Show>

            <Show when=move || error_msg.get().is_some()>
                <div style="text-align:center;padding:24px;color:#e53935">
                    {move || error_msg.get().unwrap_or_default()}
                </div>
            </Show>

            <Show when=move || !loading.get() && error_msg.get().is_none() && kb_state.list.read().is_empty()>
                <div style="text-align:center;padding:24px;color:#888">"暂无知识库"</div>
            </Show>

            <Show when=move || !loading.get() && !kb_state.list.read().is_empty()>
                <div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(300px,1fr));gap:16px">
                    {move || kb_state.list.get().into_iter().map(|kb| {
                        view! {
                            <div style="background:#fff;border-radius:12px;padding:20px;border:2px solid transparent">
                                <h3 style="font-size:16px;font-weight:600">{kb.name.clone()}</h3>
                                <p style="font-size:14px;color:#666;margin:8px 0">{kb.description.clone().unwrap_or_default()}</p>
                                <span style="font-size:12px;color:#888">"创建于 "{kb.created_at.clone()}</span>
                                <div style="display:flex;gap:8px;justify-content:flex-end;margin-top:16px">
                                    <button on:click=move |_| {
                                            kb_state.current_kb.set(Some(kb.clone()));
                                            navigate(&format!("/kb/{}", kb.id), NavigateOptions::default());
                                        }
                                        style="padding:6px 12px;background:#111;color:#fff;border:none;border-radius:6px">
                                        "查看"
                                    </button>
                                    <button on:click=move |_| {
                                            #[cfg(feature = "ssr")]
                                            {
                                                if web_sys::window().unwrap().confirm_with_message("确定删除？").unwrap() {
                                                    delete_kb(kb.id);
                                                }
                                            }
                                        }
                                        style="padding:6px 12px;background:#fee2e2;color:#dc2626;border:none;border-radius:6px">
                                        "删除"
                                    </button>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </Show>

            <Show when=move || kb_state.total_pages.get() > 1_i64>
                <div style="display:flex;justify-content:center;gap:8px;margin-top:24px">
                    <button disabled=move || current_page.get() <= 1
                        on:click=move |_| { current_page.update(|p| *p -= 1); loading.set(true); }
                        style="padding:8px 16px;background:#fff;border:1px solid #e5e7eb;border-radius:6px">
                        "上一页"
                    </button>
                    <span style="padding:8px 16px">
                        {move || format!("{} / {}", current_page.get(), kb_state.total_pages.get())}
                    </span>
                    <button disabled=move || current_page.get() >= kb_state.total_pages.get() as i32
                        on:click=move |_| { current_page.update(|p| *p += 1); loading.set(true); }
                        style="padding:8px 16px;background:#fff;border:1px solid #e5e7eb;border-radius:6px">
                        "下一页"
                    </button>
                </div>
            </Show>
        </main>
    }
}