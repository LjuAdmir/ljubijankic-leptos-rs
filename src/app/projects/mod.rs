use leptos::prelude::*;
use leptos_meta::Title;

struct ProjectCategory {
    id: u32,
    name: String,
}

struct Project {
    id: u32,
    name: String,
    description: String,
    category: ProjectCategory,
}

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <Title text="Projects" />
        <h1 class="uppercase text-3xl pb-4">"Projects"</h1>
        <p>"Welcome to the Projects Page!"</p>
    }
}