use crate::error_template::{AppError, ErrorTemplate};
use crate::pages::home::Home;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/chacha.css"/>

        <Title text="Chacha"/>

        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx, <ErrorTemplate outside_errors/> }.into_view(cx)
        }>
            <main>
                <Routes>
                    <Route path="/" view=Home/>
                </Routes>
            </main>
        </Router>
    }
}
