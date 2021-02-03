use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use crate::components::nav::Nav;

pub struct Yewssenger {
  link: ComponentLink<Self>,
}

pub enum Msg {
  None,
}

impl Component for Yewssenger {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
    Yewssenger { link }
  }

  fn update(&mut self, msg: Self::Message) -> ShouldRender {
    match msg {
      Msg::None => (),
    }
    true
  }
  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <main id="yewssenger">
       <div class="flex h-screen">
        <Nav/>
        <section class="w-full">{"section"}</section>
       </div>
      </main>
    }
  }
}
