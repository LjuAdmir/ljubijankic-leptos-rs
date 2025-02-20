use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About" />
        <h1 class="uppercase text-3xl pb-4">"About"</h1>
        <p>"Welcome to the About Page!"</p>
    }
}