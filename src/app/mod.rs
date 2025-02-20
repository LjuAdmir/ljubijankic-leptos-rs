mod about;
mod contact;
mod home;
mod projects;

use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};
use leptos_meta::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    
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

    let (menu_open, set_menu_open) = signal(false);

    provide_context(set_selected);

    view! {
        <Title formatter=|text| format!("Admir Ljubijankić - {text}") />

        <Router>
            <nav class="flex-none w-full md:w-fit bg-zinc-800 p-8">
                <div class="flex justify-between items-center">
                    <HeaderTitle />
                    <button
                        on:click=move |_| set_menu_open.set(!menu_open.get())
                    >
                        <span class="iconamoon--menu-burger-horizontal-bold md:hidden"></span>
                    </button>
                </div>
                <Navigation selected=selected menu_open=menu_open />
                <CustomFooter class="hidden md:block md:absolute md:bottom-0" />
            </nav>
            <div class="flex-auto overflow-y-scroll bg-zinc-950 p-8 lg:p-20">
            <Routes fallback=|| view!{"404"}>
                <Route path=path!("/") view={home::Home} />
                <Route path=path!("/about") view={about::About} />
                <Route path=path!("/contact") view={contact::Contact} />
                <Route path=path!("/projects") view={projects::Projects} />
            </Routes>
            </div>
            <CustomFooter class="block md:hidden" />
        </Router>
    }
}


#[component]
fn HeaderTitle() -> impl IntoView {
    let set_selected = use_context::<WriteSignal<String>>()
        .expect("HeaderTitle must be used within a context provider");

    view! {
        <h1 class="font-black text-sky-600 text-2xl sm:text-3xl md:text-4xl">
            <a href="/"
                on:click=move |_| set_selected.set("home".to_string())
                class="flex items-center space-x-2 md:block md:space-x-0"
            >
                <span class="md:block">"Admir"</span>
                <span class="md:block">"Ljubijankić"</span>
            </a>
        </h1>
    }
}

#[component]
fn Navigation(
    selected: ReadSignal<String>,
    menu_open: ReadSignal<bool>,
) -> impl IntoView {
    let set_selected = use_context::<WriteSignal<String>>()
        .expect("Navigation must be used within a context provider");

    view! {
        <ul
            class="md:block pt-4 text-2xl text-sky-800"
            class:hidden=move || !menu_open.get()
        >
            <li
                on:click=move |_| set_selected.set("home".to_string())
                class=(["text-sky-600", "font-bold"], move || selected.get() == "home")
                class="hover:text-sky-700"
            >
                <a href="/" class="uppercase px-4 py-1 block">"Home"</a>
            </li>
            <li
                on:click=move |_| set_selected.set("about".to_string())
                class=(["text-sky-600", "font-bold"], move || selected.get() == "about")
                class="hover:text-sky-700"
            >
                <a href="/about" class="uppercase px-4 py-1 block">"About"</a>
            </li>
            <li
                on:click=move |_| set_selected.set("contact".to_string())
                class=(["text-sky-600", "font-bold"], move || selected.get() == "contact")
                class="hover:text-sky-700"
            >
                <a href="/contact" class="uppercase px-4 py-1 block">"Contact"</a>
            </li>
            <li
                on:click=move |_| set_selected.set("projects".to_string())
                class=(["text-sky-600", "font-bold"], move || selected.get() == "projects")
                class="hover:text-sky-700"
            >
                <a href="/projects" class="uppercase px-4 py-1 block">"Projects"</a>
            </li>
        </ul>
    }
}

#[component]
fn CustomFooter(
    class: &'static str,
) -> impl IntoView {
    view! {
        <footer
            class=class.to_owned() + " pb-2"
        >
            <div class="flex justify-left items-center space-x-2 pt-4 text-sky-700/50 pl-4 md:pl-0">
                <a rel="external" target="_blank" href="https://github.com/LjuAdmir">
                    <span class="fa6-brands--square-github w-[48px] h-[48px] hover:text-sky-600/50"></span>
                </a>
                <a rel="external" target="_blank" href="https://www.linkedin.com/in/admir-ljubijanki%C4%87-8746321a5/">
                    <span class="fa-brands--linkedin w-[48px] h-[48px] hover:text-sky-600/50"></span>
                </a>
            </div>
            <div>
                <p class="text-white/25 pl-4 md:pl-0">"© 2025 Admir Ljubijankić"</p>
            </div>
        </footer>
    }
}