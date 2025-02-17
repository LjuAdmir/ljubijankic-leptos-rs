mod about;
mod contact;
mod home;
mod projects;

use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    // Prepare initial state
    let location = location_pathname();
    let (selected, set_selected) = signal(
        location
            .map(|path| {
                if path == "/" {
                    "home".to_string()
                } else {
                    path.trim_start_matches("/").to_string()
                }
            })
            .unwrap(),
    );

    view! {
        <Router>
            <nav>
                <h1><a href="/"
                    on:click=move |_| set_selected.set("home".to_string())
                >"Admir Ljubijankić"</a></h1>
                <ul>
                    <li
                        on:click=move |_| set_selected.set("home".to_string())
                        class:selected=move || selected.get() == "home"
                    ><a href="/">"Home"</a></li>
                    <li
                        on:click=move |_| set_selected.set("about".to_string())
                        class:selected=move || selected.get() == "about"
                    ><a href="/about">"About"</a></li>
                    <li
                        on:click=move |_| set_selected.set("contact".to_string())
                        class:selected=move || selected.get() == "contact"
                    ><a href="/contact">"Contact"</a></li>
                    <li
                        on:click=move |_| set_selected.set("projects".to_string())
                        class:selected=move || selected.get() == "projects"
                    ><a href="/projects">"Projects"</a></li>
                </ul>
                <footer>
                    <div class="connetions">
                        <span class="fa6-brands--square-github page-icon"></span>
                    </div>
                    <div class="copyright">
                        <p>"© 2025 Admir Ljubijankić"</p>
                    </div>
                </footer>
            </nav>
            <Routes fallback=|| view!{"404"}>
                <Route path=path!("/") view={home::Home} />
                <Route path=path!("/about") view={about::About} />
                <Route path=path!("/contact") view={contact::Contact} />
                <Route path=path!("/projects") view={projects::Projects} />
            </Routes>
        </Router>
    }
}
