use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

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
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <HomePage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListPokemonsResponse {
    count: u32,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<Pokemon>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pokemon {
    name: String,
    url: String,
}

impl Pokemon {
    pub fn img_url(&self) -> String {
        let id = self
            .url
            .split('/')
            .rev()
            .nth(1)
            .unwrap()
            .parse::<i8>()
            .unwrap();

        format!("https://raw.githubusercontent.com/PokeAPI/sprites/master/sprites/pokemon/other/dream-world/{id}.svg")
    }
}

#[server(GetPokemons, "/api")]
pub async fn get_pokemons(_cs: Scope) -> Result<Vec<Pokemon>, ServerFnError> {
    let response = reqwest::get("https://pokeapi.co/api/v2/pokemon?limit=8&offset=8")
        .await
        .map_err(|e| log::error!("{e}"))
        .unwrap()
        .json::<ListPokemonsResponse>()
        .await;

    let pokemons = match response {
        Ok(data) => data.results,
        Err(e) => {
            vec![]
        }
    };

    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut pokemons = pokemons.iter().chain(pokemons.iter()).collect::<Vec<_>>();
    pokemons.shuffle(&mut thread_rng());

    Ok(pokemons.iter().map(|p| (*p).clone()).collect())
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub fn register_server_functions() {
            _ = GetPokemons::register();
        }
    }
}

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let pokemons = create_resource(cx, move || {}, move |_| get_pokemons(cx));

    view! { cx,
        <div class="w-screen h-screen flex flex-col items-center py-12 bg-gradient-to-br from-blue-700 to-blue-500">
            <h1 class="text-7xl font-pokemon text-yellow-header drop-shadow-header tracking-wider">"Poke Flip"</h1>
            <Suspense
                fallback=move || view! { cx, <p>"Loading..."</p>}
            >
                {move || {
                    pokemons.read().map(move |data| match data {
                        Err(_) => view! { cx,  <div>"Error"</div> },
                        Ok(pokes) => view! { cx,
                            <div class="h-full w-auto">
                            <CardsGrid>
                                <For
                                    each=move || pokes.clone()
                                    key=|poke| poke.name.clone()
                                    view=move |cx, poke: Pokemon| {
                                        view! {
                                            cx,
                                            <Card poke=poke />
                                        }
                                    }
                                />
                            </CardsGrid>
                            </div>
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}

#[component]
pub fn CardsGrid(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="flex-1 grid grid-cols-4 gap-4 mt-8 h-full">
            { children(cx) }
        </div>
    }
}

#[component]
pub fn Card(cx: Scope, poke: Pokemon) -> impl IntoView {
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
                >
                    <img
                        src=poke.img_url()
                        alt=poke.name
                        class="w-auto h-full aspect-square object-contain"
                    />
                </div>
            </div>
        </div>
    }
}
