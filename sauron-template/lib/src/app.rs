use sauron::{node, prelude::*};

use crate::model::Model;
use crate::msg::Msg;

#[derive(Debug)]
pub struct App {
    pub data: Model,
}

impl App {
    pub fn new() -> Self {
        Self {
            data: Model { count: 0 },
        }
    }
}

impl Application<Msg> for App {
    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Click => {
                self.data.count += 1;
            }
        }
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        node! {
            <main>
                <h1>Minimal Example</h1>
                <div>
                    <input
                        type="button"
                        value="Click Me!"
                        on_click=|_| Msg::Click
                    />
                    <div>
                        {text(format!("Clicked: {} times", self.data.count))}
                    </div>
                    <input type="text" value=self.data.count />
                </div>
            </main>
        }
    }
}
