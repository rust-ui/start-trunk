use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-2 p-4">
            <h1 class="text-2xl font-bold">"Rust UI Starters — Trunk Tailwind"</h1>
            <p>"Starter template built with Leptos, Trunk and Tailwind CSS."</p>
        </div>
    }
}
