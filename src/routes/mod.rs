pub mod all;
pub mod ask;

use all::All;
use ask::Ask;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/all")]
    All,
    #[at("/ask")]
    Ask,
}

pub fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::All => html! {<All /> },
        AppRoute::Ask => html! {<Ask /> },
    }
}
