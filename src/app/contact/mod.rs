use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Title text="Contact" />
        <h1 class="uppercase text-3xl pb-4">"Contact"</h1>
        <p>"Welcome to the Contact Page!"</p>
    }
}