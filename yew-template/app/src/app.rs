use yew::function_component;
use yew::Html;
use yew_html_ext::html;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="container">
            <CounterComponent />
            <ButtonComponent />
        </div>
    }
}
