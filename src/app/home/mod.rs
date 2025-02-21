use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Title text="Home" />
        <h1 class="uppercase text-3xl font-[1000]">"Backend Developer | Linux Enthusiast | Problem Solver"</h1>
        <p>"Building robust web solutions with Drupal, PHP, and more. Passionate about clean code, open-source, and gaming on Linux."</p>
        <hr class="pb-8" />

        <p class="pb-8">"Hi, I'm Admir Ljubijankić. I'm a backend web developer specializing in Drupal and PHP, with a love for tinkering with general software development. When I'm not coding, you'll find me gaming on my Linux rig or exploring new open-source projects."</p>

        <h2 class="text-xl font-[800]">"Notable skills"</h2>
        <div class="grid grid-cols-1 gap-4 md:grid-cols-2">
            <div>
                <h3 class="text-lg font-[700]">"Python"</h3>
                <p>"I've been working with Python for over 5 years, building custom scripts, web applications, and automation tools. I'm also have experience in machine learning, machine vision, and web scraping."</p>
            </div>
            <div>
                <h3 class="text-lg font-[700]">"Django"</h3>
                <p>"I've been working with Django for over 3 years, building custom web applications, APIs, and integrations. I'm also experienced in object-oriented programming, design patterns, and testing."</p>
            </div>
            <div>
                <h3 class="text-lg font-[700]">"PHP"</h3>
                <p>"I've been working with PHP for over 5 years, building custom web applications, APIs, and integrations. I'm also experienced in object-oriented programming, design patterns, and testing."</p>
            </div>
            <div>
                <h3 class="text-lg font-[700]">"Drupal"</h3>
                <p>"I've been working with Drupal for over 2 years, building custom modules, themes, and distributions. I'm also experienced in site building, configuration, and maintenance."</p>
            </div>
            <div>
                <h3 class="text-lg font-[700]">"Linux"</h3>
                <p>"I've been using Linux for over 10 years, building custom desktop environments, servers, and containers. I'm also experienced in shell scripting, package management, and system administration."</p>
            </div>
        </div>
    }
}