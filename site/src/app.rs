use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::{
    pages::{projects::Projects, home::HomePage}, 
    components::{header::Header, footer::Footer}
};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);
    
    view! { cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>
        <Title text="Liran"/>
        <Router>
            <Header/>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|cx| {
                            view! { cx, <HomePage/> }
                        }
                    />
                    <Route
                        path="projects"
                        view=|cx| {
                            view! { cx, <Projects/> }
                        }
                    />
                </Routes>
                </main>
                <Footer/>
        </Router>
    }
}