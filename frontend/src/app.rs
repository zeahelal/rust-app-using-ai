use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use anyhow::Result;

#[derive(Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
    pub fn css_class(&self) -> &'static str {
        match self {
            Theme::Light => "theme-light",
            Theme::Dark => "theme-dark",
        }
    }
}

#[hook]
pub fn use_theme() -> (Theme, Callback<MouseEvent>) {
    let theme = use_state(|| Theme::Light);

    let toggle_theme = {
        let theme = theme.clone();
        Callback::from(move |_| {
            theme.set(match *theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
            });
        })
    };

    ((*theme).clone(), toggle_theme)
}

// Add the missing fetch_hello function
async fn fetch_hello() -> Result<String, String> {
    // Example: fetch from backend API
    let url = "/api/hello";
    let resp = reqwest::get(url)
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    let text = resp.text()
        .await
        .map_err(|e| format!("Read error: {}", e))?;

    Ok(text)
}

#[function_component(App)]
pub fn app() -> Html {
    let (theme, toggle_theme) = use_theme();
    let message = use_state(|| String::new());
    let error = use_state(|| String::new());

    let on_fetch_hello = {
        let set_message = message.clone();
        let set_error = error.clone();
        
        Callback::from(move |_| {
            set_message("Loading...".to_string());
            set_error(String::new());
            
            let set_message = set_message.clone();
            let set_error = set_error.clone();
            
            spawn_local(async move {
                match fetch_hello().await {
                          set_message.set(text);
             set_message(text);
                    }
                             set_error.set(format!("Error: {}", e));
                        set_message.set(String::new());
et_message(String::new());
                    }
                }
            });
        })
    };

    html! {
        <div class={classes!("app-container", theme.css_class())}>
            <header class="card">
                <h1>{"App Using AI"}</h1>
                <button class="btn-primary" onclick={toggle_theme}>{"Toggle Theme"}</button>
            </header>
            <main class="card">
                <p>{"Click the button below to fetch a greeting from the backend:"}</p>
                <button class="btn-primary" onclick={on_fetch_hello}>
                    {"Fetch Greeting"}
                if !(*message).is_empty() {
                    <p class    ="message">{ &*message }</p>    
                }

                if !(*error).is_empty() {
                    <p class="error">{ &*error }</p>
                }
 <p class="error">{ &error }</p>
                }
            </main>
        </div>
    }
}
