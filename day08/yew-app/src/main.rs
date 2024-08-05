use yew::{function_component, html, prelude::*, use_state};

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            counter.set(*counter + 1);
        }
    };
    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{*counter}</p>
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
