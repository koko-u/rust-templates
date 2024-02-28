use std::rc::Rc;

use yew::Component;
use yew::Context;
use yew::Html;
use yew::Properties;
use yew_html_ext::html;
use yewdux::Dispatch;

use crate::store::AppStore;

pub struct ButtonComponent {
    dispatch: Dispatch<AppStore>,
}

pub enum Msg {
    Store(Rc<AppStore>),
    Up,
    Reset,
}

#[derive(Properties, PartialEq)]
pub struct Props;

impl Component for ButtonComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let dispatch =
            Dispatch::<AppStore>::global().subscribe(ctx.link().callback(Self::Message::Store));
        Self {
            dispatch,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Store(_) => true,
            Self::Message::Up => {
                self.dispatch.reduce(|store| AppStore { count: store.count + 1 }.into());
                true
            }
            Self::Message::Reset => {
                self.dispatch.set(AppStore::default());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let store = self.dispatch.get();

        html! {
            <div class="d-flex justify-content-around align-items-center">
                <button
                    class="btn btn-primary"
                    onclick={ctx.link().callback(|_| Self::Message::Up)}
                >
                    {"Up"}
                </button>
                <button
                    class="btn btn-outline-secondary"
                    onclick={ctx.link().callback(|_| Self::Message::Reset)}
                >
                    {"Reset"}
                </button>
            </div>
        }
    }

}
