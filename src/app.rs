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
    let (flipped, set_flipped) = create_signal(cx, false);

    let toggle_flipped = move |_| set_flipped(!flipped());

    view! { cx,
        <div
            class="relative w-40 h-full rounded-md hover:cursor-pointer hover:scale-105
                   transition-transform duration-300 overflow-hidden shadow-md
                   [perspective:1000px]"
            on:click=toggle_flipped
        >
            <div
                class="h-full w-full transition-transform duration-500 [transform-style:preserve-3d]"
                class=("[transform:rotateY(180deg)]", flipped)
            >
                // Back side of the card
                <div class="absolute w-full h-full bg-blue-700 border-solid border-white border-4 rounded-lg [backface-visibility:hidden]">
                    <img
                        src="/images/poke_ball.png"
                        alt="Pokeball"
                        class="w-auto h-full aspect-square object-contain"
                    />
                </div>
                // Front side of the card
                <div
                    class="absolute w-full h-full bg-blue-100 border-solid border-white
                           border-4 rounded-lg [backface-visibility:hidden] [transform:rotateY(180deg)]"
                ></div>
            </div>
        </div>
    }
}
