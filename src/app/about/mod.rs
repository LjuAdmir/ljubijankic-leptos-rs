use leptos::prelude::*;
use leptos_meta::Title;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About" />
        <h1 class="uppercase text-3xl font-[1000]">"About"</h1>
        <p>"Developer, Linux Enthusiast, and Problem Solver"</p>
        <hr class="pb-8" />

        <p class="pb-8">"Hi, I'm Admir Ljubijankić. I'm a backend web developer with a passion for building robust, scalable solutions and exploring new technologies. I currently work with Drupal and PHP, but I’ve also spent years working with Python, C/C++, and Rust. When I’m not coding, you’ll find me gaming on my Linux setup or contributing to open-source projects"</p>

        <div class="pb-8">
            <h2 class="text-xl font-[800] pb-4">"My Journey"</h2>
            <h3 class="text-lg font-[700]">"Professional Background"</h3>
            <p class="pb-4">"I’ve been working as a developer for 10 years, specializing in backend development. My current role focuses on building and maintaining Drupal-based web applications, but I’ve also worked on a variety of projects involving Python, C/C++, and Rust."</p>
            <p class="pb-8">"Outside of development, I have extensive experience in networking, web hosting, and Linux system maintenance. Over the years, I’ve configured and managed networking infrastructure using MikroTik and Ubiquiti equipment, ensuring reliable and secure connectivity for various environments. Additionally, I’ve provided technical support and troubleshooting for a wide range of systems, further honing my problem-solving skills and deepening my understanding of IT ecosystems."</p>
            <h3 class="text-lg font-[700]">"Skills and Expertise"</h3>
            <ul class="list-disc pl-8">
                <li>"Backend: PHP, Python, Django, Flask, Drupal, Rust"</li>
                <li>"Frontend: HTML, CSS, JavaScript, Vue.js"</li>
                <li>"Databases: MySQL, SQLite"</li>
                <li>"Tools: Git, Docker, Composer, Pip, NPM, Yarn"</li>
                <li>"Operating Systems: Linux, Windows, macOS"</li>
            </ul>
        </div>

        <div class="pb-8">
            <h2 class="text-xl font-[800] pb-4">"My Passions"</h2>
            <h3 class="text-lg font-[700]">"Linux and Open Source"</h3>
            <p class="pb-4">"I’m a huge advocate for open-source software and have been using Linux for 10. I love customizing my setup, contributing to open-source projects, and exploring the endless possibilities of the Linux ecosystem."</p>
            <h3 class="text-lg font-[700]">"Gaming on Linux"</h3>
            <p class="pb-4">"As a Linux gamer, I’ve spent countless hours tweaking my system to run games smoothly. It’s a challenge I enjoy, and it’s taught me a lot about problem-solving and optimization"</p>
            <h3 class="text-lg font-[700]">"Continuous Learning"</h3>
            <p>"I’m always learning new technologies and improving my skills. Whether it’s diving into a new programming language or experimenting with a new framework, I thrive on challenges and growth."</p>
        </div>

        <div class="pb-8">
            <h2 class="text-xl font-[800] pb-4">"Fun facts"</h2>
            <p class="pb-2">"My programming journey began when I was just a kid, inspired by a dream to create my own game. I started by tinkering with my brother’s Adobe Flash and built a small platformer using ActionScript. That spark of curiosity has stayed with me ever since."</p>
            <p class="pb-2">"When I’m not immersed in the stunning worlds of Guild Wars 2 or Skyrim, I enjoy exploring the real world with my Fujifilm X-T30. Capturing moments through photography helps me see the world from a different perspective."</p>
            <p class="pb-2">"I’m a proud Arch Linux user (yes, I use Arch, BTW). My current setup is powered by a fully AMD build Ryzen 5 5600X and Radeon RX 6800 XT, because why not have a little fun with hardware too?"</p>
            <p class="pb-2">
                "Currently, I’m employed as a backend developer at "
                <a rel="external" target="_blank" href="https://www.agiledrop.com/" class="text-sky-600 hover:underline">"Agiledrop"</a>
                ", where I focus on building and maintaining Drupal-based solutions. While I’m not actively searching for new opportunities, I’m always open to conversations about technology, collaboration, or shared interests—whether it’s development, Linux, gaming, or photography. Feel free to reach out!"
                </p>
        </div>
    }
}