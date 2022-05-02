use yew::{function_component, html, Properties};
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::services::article::get_articles;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {}

#[function_component(All)]
pub fn all(props: &Props) -> Html {
    let state = use_async_with_options(
        async move { get_articles().await },
        UseAsyncOptions::enable_auto(),
    );

    log::info!("{:?}", state.data);

    if let Some(data) = &state.data {
        html! {
          <ul class="list-container">
          {for data.data.iter().map(|item| {
            html! {
              <li class="item">
              // <img class="avatar" src={&*(item.author).avatar_url} alt="" />
                <div class="count-cell">
                  <span class="reply">{item.reply_count}</span>
                  <span class="seperator">{"/"}</span>
                  <span class="visit">{item.visit_count}</span>
                </div>
                <div class="title">{&item.title}</div>
              </li>
            }
          })}
          </ul>
        }
    } else {
        html! {
          <div>{"Loading..."}</div>
        }
    }
}
