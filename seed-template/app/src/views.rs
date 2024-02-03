use seed::prelude::*;
use seed::*;

use crate::states::messages::Msg;
use crate::states::models::Model;

pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["container", "mt-4"],
        h1!["Seed Template"],
        span![C!["m-2"], model.count],
        div![
            C!["m-2"],
            button![
                C!["btn btn-primary"],
                "Click Me!",
                ev(Ev::Click, |_| Msg::DoSomething)
            ]
        ]
    ]
}
