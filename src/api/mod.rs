use cfg_if::cfg_if;
use leptos::*;
use serde::{Deserialize, Serialize};

use crate::models::{PokeCard, Pokemon};

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

        pub fn register_server_functions() {
            _ = GetPokemons::register();
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ListPokemonsResponse {
    count: u32,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<Pokemon>,
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

pub async fn get_poke_cards(cx: Scope) -> Vec<PokeCard> {
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
