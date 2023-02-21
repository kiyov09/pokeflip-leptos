use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::card::*;
use crate::components::grid::*;
use crate::models::Game;
use crate::models::PokeCard;

#[component]
pub fn PokeflipApp(cx: Scope) -> impl IntoView {
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
            <main class="w-full h-full">
                <Routes>
                    <Route path="" view=move |cx| view! { cx, <MainPage /> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
pub fn MainPage(cx: Scope) -> impl IntoView {
    let (game, set_game) = create_signal(cx, Game::new(cx));

    provide_context(cx, set_game);
    provide_context(cx, game);

    let poke_cards = move || game.with(|game| game.poke_cards);
    let poke_cards_view = move || {
        poke_cards().read(cx).map(|cards| {
            view! { cx,
                <div class="flex-1 mt-8 w-full max-w-2xl px-4">
                    <CardsGrid>
                        <For
                            each=move || cards.clone()
                            key=|poke| poke.id
                            view=move |cx, poke: PokeCard| {
                                view! {
                                    cx,
                                    <Card card=poke/>
                                }
                            }
                        />
                    </CardsGrid>
                </div>
            }
            .into_any()
        })
    };

    view! { cx,
        <div class="w-full h-full flex flex-col items-center py-12 bg-gradient-to-br from-blue-700 to-blue-500">
            <h1 class="text-5xl md:text-7xl font-pokemon text-yellow-header drop-shadow-header tracking-wider">"Poke Flip"</h1>
            <Transition
                fallback=move || view!{ cx, <p>"Loading cards..."</p>}
            >
                { poke_cards_view }
            </Transition>
        </div>
    }
}
