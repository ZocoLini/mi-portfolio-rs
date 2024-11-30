use stylist::css;
use yew::prelude::*;

#[function_component(LoadingSpinner)]
pub fn loading_icon() -> Html {
    let css = css!("
        display: flex;
        justify-content: center;
        align-items: center;
        height: 100vh;
        background-color: #f0f0f0; /* Fondo opcional */
        
        .spinner {
            width: 50px;
            height: 50px;
            border: 5px solid rgba(0, 0, 0, 0.2);
            border-top-color: #3498db;
            border-radius: 50%;
            animation: spin 1s linear infinite;
        }

        @keyframes spin {
            0% {
                transform: rotate(0deg);
            }
            100% {
                transform: rotate(360deg);
            }
        }"
    );

    html! {
        <div class={css}>
            <div class="spinner"></div>
        </div>
    }
}
