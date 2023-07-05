use leptos::*;
use questions::*;

// #[server(CheckAnswer, "/api")]
// pub async fn check_answer(input: String) -> Result<String, ServerFnError> {
//     let (answers, longest) = questions::load("worldcities.csv");
//
//     Ok(String::from("moose"))
// }

async fn server_check_answer(value: String) -> String {
    let (answers, longest) = load();

    check_answer(&*value, &answers, longest).to_string()
    //Answer::Longest(world_cities.to_string())
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Controlled".to_string());

    let async_data = create_resource(
        cx,
        name,
        |value| async move { server_check_answer(value).await },
    );

    let async_result = move || {
        async_data
            .read(cx)
            .map(|value| format!("Server returned {value:?}"))
            .unwrap_or_else(|| "Loading...".into())
    };

    let loading = async_data.loading();
    let is_loading = move || if loading() { "Loading..." } else { "Idle." };

    view! { cx,
        <input type="text"
        on:input=move |ev| {
            ev.prevent_default();
            set_name(event_target_value(&ev));
        }/>
        <p>
            <code>"count"</code>": " {name}
        </p>
        <p>
            <code>"async_value"</code>": "
            {async_result}
            <br/>
            {is_loading}
        </p>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
