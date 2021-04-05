use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use crate::components::chat_room::ChatRoom;
use crate::components::nav::Nav;

pub struct Home {}

pub enum Msg {}

impl Component for Home {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    log::info!("asdf");
    Self {}
  }
  fn update(&mut self, _: Self::Message) -> ShouldRender {
    false
  }
  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }
  fn view(&self) -> Html {
    html! {
      <div class="flex h-screen">
        <Nav/>
        <section class="w-full">
          <ChatRoom/>
        </section>
      </div>
    }
  }
}
