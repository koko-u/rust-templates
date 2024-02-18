use yew::prelude::*;

pub struct App {
    count: i32,
}

pub enum Msg {
    Up,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <main>
                <form>
                    <label>{"Username"}</label>
                    <input type="text" value="" />
                    <label>{"Password"}</label>
                    <input type="password" value="" />
                    <button type="submit">{"Login"}</button>
                </form>
            </main>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Up => {
                self.count += 1;
            }
        }
        true
    }
}
