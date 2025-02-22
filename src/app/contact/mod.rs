use leptos::prelude::*;
use leptos_meta::Title;
use serde::{Deserialize, Serialize};

use send_wrapper::SendWrapper;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
struct ContactFormData {
    name: String,
    email: String,
    message: String,
}

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <Title text="Contact" />
        <h1 class="uppercase text-3xl font-[1000]">"Let's Connect!"</h1>
        <p>"Whether you’d like to discuss a project, share ideas, or just chat about tech, I’d love to hear from you."</p>
        <hr class="pb-8" />

        <ContactForm />
    }
}

#[component]
fn ContactForm() -> impl IntoView {
    let form_data = RwSignal::new(ContactFormData::default());
    let is_loading = RwSignal::new(false);
    let is_sent = RwSignal::new(false);
    let has_error = RwSignal::new(false);


    let submit_action = Action::new(move |data: &ContactFormData| {
        let data = data.clone();

        // We need to wrap the async function in a SendWrapper to ensure it can be sent between threads.
        SendWrapper::new(async move {
            send_email(data, is_loading.clone(), is_sent.clone(), has_error.clone()).await;
        })
    });

    view! {
        <div>
            <Show when=move || is_sent.get()>
                <div class="border border-green-400 text-green-700 px-4 py-3 rounded relative" role="alert">
                    <strong class="font-bold">"Success!"</strong>
                    <span class="block sm:inline">"Your message has been sent successfully."</span>
                </div>
            </Show>

            <Show when=move || has_error.get()>
                <div class="border border-red-400 text-red-700 px-4 py-3 rounded relative" role="alert">
                    <strong class="font-bold">"Error!"</strong>
                    <span class="block sm:inline">"An error occurred while sending your message. Please try again later."</span>
                </div>
            </Show>

            <form
                on:submit=move |e| {
                    e.prevent_default();
                    let input = form_data.get();
                    submit_action.dispatch(input);
                }
            >
                <div class="pb-2">
                    <label for="name" class="block font-bold mb-2">"Name"</label>
                    <input
                        type="text"
                        id="name"
                        class="w-full px-3 border border-zinc-700 py-2 rounded-lg focus:outline-none focus:ring-2 focus:ring-zinc-500 bg-zinc-900"
                        prop:value=move || form_data.get().name
                        on:input:target=move |e| form_data.update(|data| data.name = e.target().value())
                        required
                    />
                </div>

                <div class="block font-bold mb-2">
                    <label for="email" class="block font-bold mb-2">"Email"</label>
                    <input
                        type="email"
                        id="email"
                        class="w-full px-3 border border-zinc-700 py-2 rounded-lg focus:outline-none focus:ring-2 focus:ring-zinc-500 bg-zinc-900"
                        prop:value=move || form_data.get().email
                        on:input:target=move |e| form_data.update(|data| data.email = e.target().value())
                        required
                    />
                </div>

                <div>
                    <label for="message" class="block font-bold mb-2">"Message"</label>
                    <textarea
                        id="message"
                        class="w-full px-3 border border-zinc-700 py-2 rounded-lg focus:outline-none focus:ring-2 focus:ring-zinc-500 bg-zinc-900"
                        rows="6"
                        prop:value=move || form_data.get().message
                        on:input:target=move |e| form_data.update(|data| data.message = e.target().value())
                        required
                    />
                </div>

                <button
                    type="submit"
                    class="bg-zinc-900 border-zinc-700 border hover:bg-sky-700 text-white font-bold py-2 px-4 rounded mt-4"
                    disabled=move || is_loading.get()
                >
                    <Show when=move || is_loading.get()>
                        <span>"Sending..."</span>
                    </Show>
                    <Show when=move || !is_loading.get()>
                        <span>"Send Message"</span>
                    </Show>
                </button>
            </form>
        </div>
    }
}

/// Send an email using the provided data.
async fn send_email(data: ContactFormData, loading: RwSignal<bool>, sent: RwSignal<bool>, error: RwSignal<bool>) -> () {
    let abort_controller = SendWrapper::new(web_sys::AbortController::new().ok());
    let abort_signal = abort_controller.as_ref().map(|ac| ac.signal());

    loading.set(true);
    error.set(false);

    // Abort in-flight requests if, e.q., we navigate away
    on_cleanup(move || {
        if let Some(abort_controller) = abort_controller.as_ref() {
            abort_controller.abort();
        }
    });

    // Prepare the json request.
    let request = gloo_net::http::Request::post("https://formspree.io/f/xqaejgna")
        .abort_signal(abort_signal.as_ref())
        .json(&data);

    // Ensure the request is valid.
    let request = match request {
        Ok(request) => request,
        Err(_) => {
            loading.set(false);
            error.set(true);
            return;
        }
    };

    // Send the request and handle the response.
    let response = request.send().await;

    loading.set(false);

    // Handle the response.
    match response {
        Ok(response) => {
            if response.ok() {
                sent.set(true);
            } else {
                loading.set(false);
            }
        }
        Err(_) => {
            loading.set(false);
        }
    }
}