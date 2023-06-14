use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Header(cx:Scope) -> impl IntoView {
    view! { cx, <Navbar/> }
}

#[component]
fn Tab(cx: Scope, target: String, children: Children) -> impl IntoView {
    let url = use_location(cx);
    let tab_classes = {
        let target = target.clone();
        move || format!("tab tab-bordered{}", 
        // Handle the root path
        if url.pathname.get() == target || 
        // Handle the rest
        (target != "/" && url.pathname.get().starts_with(&target)) 
        {" tab-active"} else {""})
    };

    view! { cx,
        <a href=target class=tab_classes>
            {children(cx)}
        </a>
    }
}

#[component]
fn Navbar(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="tabs bg-neutral/50 backdrop-blur-sm mx-auto w-fit mb-2 text-accent">
            <Tab target="/".to_string()>"Home"</Tab>
            <Tab target="/projects".to_string()>"Projects"</Tab>
        </div>
    }
}