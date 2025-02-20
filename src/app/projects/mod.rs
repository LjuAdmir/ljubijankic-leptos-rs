use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <Title text="Projects" />
        <h1 class="uppercase text-3xl pb-4">"Projects"</h1>
        <p>"Welcome to the Projects Page!"</p>
    }
}