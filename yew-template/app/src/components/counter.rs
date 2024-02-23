use std::rc::Rc;

use yew::Component;
use yew::Context;
use yew::Html;
use yew::Properties;
use yew_html_ext::html;
use yewdux::Dispatch;

use crate::store::AppStore;

pub struct CounterComponent {
    dispatch: Dispatch<AppStore>,
}

pub enum Msg {
    Store(Rc<AppStore>),
}

#[derive(Properties, PartialEq)]
pub struct Props;

impl Component for CounterComponent {
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
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let store = self.dispatch.get();

        html! {
            <div>
                {"Current Count: "}{store.count.clone()}
            </div>
        }
    }

}