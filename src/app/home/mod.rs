use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Home" />
        <h1 class="uppercase text-3xl pb-4">"Home"</h1>
        <p>"Welcome to the Home Page!"</p>
    }
}