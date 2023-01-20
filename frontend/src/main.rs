use yew::prelude::*;

enum Msg {
    AddOne,
}
struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1>{"test"}</h1>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
