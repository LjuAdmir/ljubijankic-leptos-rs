use leptos::prelude::*;
use leptos_meta::Title;

use serde::{Deserialize, Serialize};

use send_wrapper::SendWrapper;

use std::error::Error;

#[derive(Serialize, Deserialize, Default, Clone)]
struct ProjectCategory {
    id: u32,
    name: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
struct Project {
    name: String,
    description: String,
    category: u32,
    url: String,
    status: String,
}

#[component]
pub fn Projects() -> impl IntoView {
    let categories = LocalResource::new(move || async move { fetch_categories().await });
    let projects = LocalResource::new(move || async move { fetch_projects().await });

    let (selected_category, set_selected_category) = signal(1);

    view! {
        <Title text="Projects" />
        <h1 class="uppercase text-3xl pb-4">"Projects"</h1>
        <Suspense
            fallback= move || view! { <div class="text-center">"Loading..."</div> }
        >
        {move || Suspend::new(async move {
            // @todo: Handle error
            let categories = categories.await.unwrap();
            view! {
                <div class="pb-2">
                    <ul class="flex flex-wrap gap-2 text-white">
                        {categories.into_iter()
                            .map(|category| view! {
                                <li
                                    class="border-zinc-700 border-0 border-b hover:text-sky-700 font-bold py-2 px-4 mt-4 cursor-pointer"
                                    class=(["text-sky-600", "text-black"], move || category.id == selected_category.get())
                                    on:click=move |_| set_selected_category.set(category.id)
                                >
                                    {category.name}
                                </li>
                            })
                            .collect::<Vec<_>>()
                        }
                    </ul>
                </div>
            }
        })}
        </Suspense>
        <Suspense
            fallback= move || view! { <div class="text-center">"Loading..."</div> }
        >
        {move || Suspend::new(async move {
            // @todo: Handle error
            let projects = projects.await.unwrap();
            view! {
                <div>
                    <ul class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
                        {projects.into_iter()
                            .filter(|project| project.category == selected_category.get())
                            .map(|project| view! {
                                <li>
                                    <h2 class="text-xl font-[800]">{project.name}</h2>
                                    <p>{project.description}</p>
                                </li>
                            })
                            .collect::<Vec<_>>()
                        }
                    </ul>
                </div>
            }
        })}
        </Suspense>
    }
}

async fn fetch_data<T>(url: &str) -> Result<T, Box<dyn Error>>
where
    T: for<'de> Deserialize<'de>,
{
    let abort_controller = SendWrapper::new(web_sys::AbortController::new().ok());
    let abort_signal = abort_controller.as_ref().map(|ac| ac.signal());

    on_cleanup(move || {
        if let Some(abort_controller) = abort_controller.as_ref() {
            abort_controller.abort();
        }
    });

    let request = gloo_net::http::Request::get(url)
        .abort_signal(abort_signal.as_ref())
        .send()
        .await;

    let response = match request {
        Ok(response) => {
            let json = response.json::<T>().await;
            match json {
                Ok(data) => Ok(data),
                Err(_) => Err("Failed to parse JSON".into()),
            }
        }
        Err(_) => Err("Failed to fetch data".into()),
    };

    response
}

async fn fetch_categories() -> Option<Vec<ProjectCategory>> {
    let url = "/data/project-cat.json";
    let data = fetch_data::<Vec<ProjectCategory>>(url).await;
    
    match data {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}

async fn fetch_projects() -> Option<Vec<Project>> {
    let url = "/data/projects.json";
    let data = fetch_data::<Vec<Project>>(url).await;
    
    match data {
        Ok(data) => Some(data),
        Err(_) => None,
    }
}