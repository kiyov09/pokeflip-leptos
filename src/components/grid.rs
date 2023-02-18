use leptos::*;

#[component]
pub fn CardsGrid(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="flex-1 grid grid-cols-4 gap-4 h-full">
            { children(cx) }
        </div>
    }
}
