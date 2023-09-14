use leptos::*;

pub fn main() {
    mount_to_body(|| {
        view! {
            <AnswerInput/>
        }
    })
}

#[component]
pub fn AnswerInput() -> impl IntoView {
    let (value, set_value) = create_signal("");

    view! {
        <div>
            <button on:click=move |_| set_value("pluh")>"Clear"</button>
            <button on:click=move |_| set_value("pluh!")>"-1"</button>
            <span>"Value: " {value} "!"</span>
            <button on:click=move |_| set_value("pluh!!")>"+1"</button>
        </div>
    }
}