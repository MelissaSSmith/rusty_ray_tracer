use gloo::console;
use js_sys::Date;
use yew::{html, Component, Context, Html};

pub struct App {
    world: Canvas
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        todo!()
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}