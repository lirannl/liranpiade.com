use leptos::*;
use leptos_meta::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! {cx, 
        <footer class="footer p-10 bg-base-100/25 text-secondary backdrop-blur">
            <a class="link" href="https://www.freepik.com">
                "Background from freepik"
            </a>
        </footer>
    }
}