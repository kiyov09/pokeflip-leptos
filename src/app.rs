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

cfg_if! {
    if #[cfg(feature = "ssr")] {
        fn get_random_offset() -> u32 {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            rng.gen_range(0..50)
        }

        fn duplicate_and_shuffle_vec<T: Clone>(v: Vec<T>) -> Vec<T> {
            use rand::seq::SliceRandom;
            use rand::thread_rng;

            let mut v = v.iter().chain(v.iter()).collect::<Vec<_>>();
            v.shuffle(&mut thread_rng());
            v.iter().map(|p| (*p).clone()).collect()
        }
    }
}

#[server(GetPokemons, "/api")]
pub async fn get_pokemons(_cs: Scope) -> Result<Vec<Pokemon>, ServerFnError> {
    let pokemons = reqwest::get(format!(
        "https://pokeapi.co/api/v2/pokemon?limit=8&offset={}",
        get_random_offset()
    ))
    .await
    .map_err(|e| log::error!("{e}"))
    .unwrap()
    .json::<ListPokemonsResponse>()
    .await
    .and_then(|data| Ok(data.results))
    .unwrap_or_default();

    Ok(duplicate_and_shuffle_vec(pokemons))
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub fn register_server_functions() {
            _ = GetPokemons::register();
        }
    }
}

async fn get_poke_cards(cx: Scope) -> Vec<PokeCard> {
    get_pokemons(cx)
        .await
        .map(|v| {
            v.iter()
                .enumerate()
                .map(|(idx, poke)| PokeCard {
                    poke: poke.clone(),
                    id: create_signal(cx, idx as u8).0,
                    flipped: create_rw_signal(cx, false),
                    disabled: create_rw_signal(cx, false),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or(vec![])
}

#[derive(Clone)]
pub struct Game {
    poke_cards: Resource<bool, Vec<PokeCard>>,
}

impl Game {
    pub fn new(cx: Scope) -> Self {
        let done = create_rw_signal(cx, true);
        let poke_cards = create_local_resource(cx, done, move |_| get_poke_cards(cx));

        create_effect(cx, move |_| {
            poke_cards.with(|cards| {
                let flipped_cards = cards
                    .iter()
                    .filter(|card| card.flipped.get())
                    .filter(|card| !card.disabled.get())
                    .cloned()
                    .collect::<Vec<_>>();

                if flipped_cards.len() == 2 {
                    set_timeout(
                        move || {
                            if flipped_cards[0].poke.name == flipped_cards[1].poke.name {
                                flipped_cards.iter().for_each(|card| card.disable())
                            } else {
                                flipped_cards.iter().for_each(|card| card.flip())
                            }
                        },
                        std::time::Duration::from_millis(500),
                    )
                }
            })
        });

        create_effect(cx, move |_| {
            poke_cards.with(|cards| {
                if cards.iter().all(|card| card.flipped.get()) {
                    set_timeout(
                        move || poke_cards.refetch(),
                        std::time::Duration::from_secs(1),
                    )
                }
            })
        });

        Self { poke_cards }
    }

    pub fn flip_card(&self, id: u8) {
        if let Some(card) = self
            .poke_cards
            .read()
            .unwrap_or(vec![])
            .iter()
            .find(|c| c.id.get() == id)
        {
            card.flip()
        };
    }
}

#[derive(Debug, Clone)]
pub struct PokeCard {
    poke: Pokemon,
    id: ReadSignal<u8>,
    flipped: RwSignal<bool>,
    disabled: RwSignal<bool>,
}

impl PokeCard {
    pub fn flip(&self) {
        self.flipped.update(|is_flipped| *is_flipped = !*is_flipped);
    }

    pub fn disable(&self) {
        self.disabled.set(true);
    }
}

impl PartialEq for PokeCard {
    fn eq(&self, other: &Self) -> bool {
        self.poke.name == other.poke.name
    }
}

impl Eq for PokeCard {}

/// Renders the home page of your application.
#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    let (game, set_game) = create_signal(cx, Game::new(cx));

    provide_context(cx, set_game);
    provide_context(cx, game);

    let poke_cards = move || game.get().poke_cards;

    let poke_cards_view = move || {
        poke_cards().with(|cards| {
            let cards = cards.clone();
            view! { cx,
                <div class="flex-1 mt-8 w-auto">
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
        <div class="w-screen h-screen flex flex-col items-center py-12 bg-gradient-to-br from-blue-700 to-blue-500">
            <h1 class="text-7xl font-pokemon text-yellow-header drop-shadow-header tracking-wider">"Poke Flip"</h1>
                { poke_cards_view }
        </div>
    }
}

#[component]
pub fn CardsGrid(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="flex-1 grid grid-cols-4 gap-4 h-full">
            { children(cx) }
        </div>
    }
}

#[component]
pub fn Card(cx: Scope, card: PokeCard) -> impl IntoView {
    let game = use_context::<ReadSignal<Game>>(cx).unwrap();

    let flip_card = move |_| game.with(|game| game.flip_card(card.id.get()));

    view! { cx,
        <button
            class="relative w-40 h-full rounded-md hover:cursor-pointer hover:scale-105
                   transition-transform duration-300 overflow-hidden shadow-md
                   [perspective:1000px] disabled:opacity-50"
            on:click=flip_card
            prop:disabled=move || card.disabled.get()
        >
            <div
                class="h-full w-full transition-transform duration-300 [transform-style:preserve-3d]"
                class=("[transform:rotateY(180deg)]", move || card.flipped.get())
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
                    class="p-4 absolute w-full h-full bg-blue-100 border-solid border-white
                           border-4 rounded-lg [backface-visibility:hidden] [transform:rotateY(180deg)]"
                >
                    <img
                        src=card.poke.img_url()
                        alt=card.poke.name
                        class="w-auto h-full aspect-square object-contain"
                    />
                </div>
            </div>
        </button>
    }
}
