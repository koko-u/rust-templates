use seed::app::Orders;
use seed::Url;

use crate::states::messages::Msg;
use crate::states::models::Model;

pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { count: 0 }
}
