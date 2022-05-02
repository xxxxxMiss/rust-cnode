pub mod all;
pub mod ask;
pub mod home;

use all::All;
use ask::Ask;
use home::Home;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[at("/")]
    Home,
    #[at("/all")]
    All,
    #[at("/ask")]
    Ask,
}

pub fn switch(routes: &AppRoute) -> Html {
    match routes {
        AppRoute::Home => html! {<Home /> },
        AppRoute::All => html! {<All /> },
        AppRoute::Ask => html! {<Ask /> },
    }
}
