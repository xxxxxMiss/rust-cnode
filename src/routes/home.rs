use web_sys::HtmlLiElement;
use yew::{function_component, html, use_state, Callback, MouseEvent, Properties, TargetCast};

#[derive(PartialEq, Properties)]
pub struct HomeProps {}

// const tabs: [(String, String); 5] = [
//     ("all".to_string(), "全部".to_string()),
//     ("good".to_string(), "精华".to_string()),
//     ("share".to_string(), "分享".to_string()),
//     ("ask".to_string(), "问答".to_string()),
//     ("job".to_string(), "招聘".to_string()),
// ];

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    let HomeProps {} = props;
    let active_tab_key = use_state(|| "all".to_string());

    let onclick = {
        let active_tab_key = active_tab_key.clone();
        Callback::from(move |e: MouseEvent| {
            let el = e.target_dyn_into::<HtmlLiElement>();
            if let Some(li) = el {
                let dataset = li.dataset();
                active_tab_key.set(dataset.get(&"key").unwrap());
            }
        })
    };

    log::debug!("state: {:?}", *active_tab_key.clone());

    html! {
      <ul class="tab-container">
        <li class="tab" data-key="all" onclick={&onclick}>{"全部"}</li>
        <li class="tab" data-key="good" onclick={&onclick}>{"精华"}</li>
        <li class="tab" data-key="share" onclick={&onclick}>{"分享"}</li>
        <li class="tab" data-key="ask" onclick={&onclick}>{"问答"}</li>
        <li class="tab" data-key="job" onclick={&onclick}>{"招聘"}</li>
      </ul>
    }
}
