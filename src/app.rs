use rust_cnode::routes::{switch, AppRoute};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
      // <main>
      //     <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
      //     <h1>{ "Hello World!!!" }</h1>
      //     <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
      // </main>
      <BrowserRouter>
        <Switch<AppRoute> render={Switch::render(switch)} />
      </BrowserRouter>
    }
}
