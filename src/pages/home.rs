use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

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
    true
  }
  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    todo!()
  }
  fn view(&self) -> Html {
    html! {
      <div class="flex h-screen">
        <Nav/>
        <section class="w-full">{"section"}</section>
      </div>
    }
  }
}
