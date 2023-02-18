use leptos::*;
use serde::{Deserialize, Serialize};

use crate::api::get_poke_cards;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pokemon {
    pub name: String,
    pub url: String,
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

#[derive(Debug, Clone)]
pub struct PokeCard {
    pub poke: Pokemon,
    pub id: ReadSignal<u8>,
    pub flipped: RwSignal<bool>,
    pub disabled: RwSignal<bool>,
}

impl PokeCard {
    pub fn flip(&self) {
        self.flipped.update(|is_flipped| *is_flipped = !*is_flipped);
    }

    pub fn disable(&self) {
        self.disabled.set(true);
    }
}

#[derive(Clone)]
pub struct Game {
    pub poke_cards: Resource<bool, Vec<PokeCard>>,
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
