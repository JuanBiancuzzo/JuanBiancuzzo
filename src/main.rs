use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <h2>{"🚧 Work in progress 🚧"}</h2>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
