use leptos::*;

use crate::models::{Game, PokeCard};

#[component]
pub fn Card(cx: Scope, card: PokeCard) -> impl IntoView {
    let game = use_context::<ReadSignal<Game>>(cx).unwrap();

    let flip_card = move |_| game.with(|game| game.flip_card(card.id.get()));

    view! { cx,
        <button
            class="relative w-auto h-full rounded-md hover:cursor-pointer hover:scale-105
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
                <div class="absolute w-full h-full bg-blue-700 border-solid border-white border-2 md:border-4 rounded-lg [backface-visibility:hidden]">
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
