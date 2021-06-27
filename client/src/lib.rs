#![recursion_limit="10000"]

use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct Model {
    link: ComponentLink<Self>,
    value: i64,
}

enum Msg {
    AddOne,
    MinusOne,
    ConnectClick,
}

fn handle_connect(){

}

impl Component for Model {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddOne => self.value += 1,
            Msg::MinusOne => self.value -= 1,
            Msg::ConnectClick => handle_connect(),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div>
                    <button onclick=self.link.callback(|_| Msg::AddOne)>{ "+1" }</button>
                    <button onclick=self.link.callback(|_| Msg::MinusOne)>{ "-1" }</button>
                    <p>{ self.value }</p>
                </div>
                
                <div>
                    <button onclick=self.link.callback(|_| Msg::ConnectClick)>{ "Connect" }</button>
                    <button id="disconnect">{ "Disconnect" }</button>
                    <button id="send">{ "Send" }</button>
            
                    <input id="text" type="text"/>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
