
use yew::prelude::*;

use crate::text_input::TextInput;

pub enum Msg {
    SetPassword(String),
    RegeneratePassword,
}

#[derive(Debug, Default)]
pub struct App {
    password: String,
}

impl App {
    fn redout_top_row_text(&self) -> String {
        let answers = return_answers(   );
        println!("{:?}", answers);
        //let l = in_answers(&self.password, &answers.0);

        //if let Some(answer) = l
        //{
        //    return format!("{}★", answer.len()).to_string();
        //}

        format!("{}", self.password.len())
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetPassword(next_password) => self.password = next_password,
            Msg::RegeneratePassword => self.password = String::from(""),
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change = ctx.link().callback(Msg::SetPassword);
        let onclick = ctx.link().callback(|_| Msg::RegeneratePassword);
        html! {
            <main>
                <div class="entry">
                    <div>
                        {"Enter a password below:"}
                        <div class="footnote">
                            {"(Will show in clear text)"}
                        </div>
                    </div>
                    <div>
                        <TextInput {on_change} value={self.password.clone()} />
                    </div>
                </div>
                <div class="readout">
                    <div>
                        {self.redout_top_row_text()}
                    </div>
                    <button {onclick}>
                        {"Generate new password *"}
                    </button>
                    <div class="footnote">
                        {"* Note: generated passwords are not actually cryptographically secure"}
                    </div>
                </div>
            </main>
        }
    }
}