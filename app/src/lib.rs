mod components;

use components::NoteEditor;
use console_error_panic_hook as _;
use wasm_bindgen::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

struct App {
    clicked: bool,
    link: ComponentLink<Self>,
}

enum Msg {
    Click,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            link: link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = !self.clicked;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked {
            "Clicked!"
        } else {
            "Click me!"
        };

        html! {
            <>
                <button onclick=&self.link.callback(|_| Msg::Click)>{ button_text }</button>

                <NoteEditor />
            </>
        }
    }
}

#[wasm_bindgen]
pub fn start() {
    console_log::init_with_level(log::Level::Trace).ok();

    yew::start_app::<App>();
}
