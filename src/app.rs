use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/pokeflip-leptos.css"/>

        // sets the document title
        <Title text="Poke Flip Game"/>
        <Meta name="description" content="A simple memory game"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="w-screen h-screen flex flex-col items-center py-12 bg-gradient-to-br from-blue-700 to-blue-500">
            <h1 class="text-7xl font-pokemon text-yellow-header drop-shadow-header tracking-wider">"Poke Flip"</h1>
            <CardsGrid />
        </div>
    }
}

#[component]
fn CardsGrid(cx: Scope) -> impl IntoView {
    let cards_range = 0..16;
    view! { cx,
        <div class="flex-1 grid grid-cols-4 gap-4 mt-8">
            {cards_range.map(|_| view ! {
                cx,
                <Card />
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[component]
fn Card(cx: Scope) -> impl IntoView {
    view! { cx,
        <div class="w-40 h-full bg-gray-200 rounded-lg"></div>
    }
}
