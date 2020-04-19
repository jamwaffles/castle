use yew::prelude::*;

pub struct NoteEditor {
    link: ComponentLink<Self>,
    note: String,
}

pub enum Msg {
    Change(String),
}

impl Component for NoteEditor {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link: link,
            note: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Change(note) => {
                log::debug!("Note: {:?}", note);
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <textarea
                    value=&self.note
                    oninput=&self.link.callback(|e: InputData| Msg::Change(e.value))
                />
            </div>
        }
    }
}
