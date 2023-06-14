use leptos::*;
use leptos_meta::*;

use crate::pages::projects::Projects;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="card p-6 w-96 bg-base-100/25 backdrop-blur-lg shadow-xl text-secondary">
            <div class="card-content flex flex-col items-center">
                <h1 class="text-5xl font-bold text-primary">"Liran Piade"</h1>
                <div class="avatar m-4">
                    <div class="w-24 rounded-full">
                        <img src="me.jpg"/>
                    </div>
                </div>
                <p class="text-lg text-center">
                    "A software developer - specialising in cloud systems, system administration, systems programming, and backends"
                </p>
            </div>
        </div>
        <div class="w-11/12 collapse collapse-plus bg-base-200/25 backdrop-blur">
            <input type="checkbox" /> 
            <div class="collapse-title text-xl font-medium">"Projects"</div>
            <div class="collapse-content">
                <Projects/>
            </div>
        </div>
    }
}