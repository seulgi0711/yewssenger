use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

use crate::router::YewssengerRouter;

pub struct Yewssenger {}

pub enum Msg {}

impl Component for Yewssenger {
  type Message = Msg;
  type Properties = ();

  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    Yewssenger {}
  }

  fn update(&mut self, _: Self::Message) -> ShouldRender {
    true
  }
  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    false
  }

  fn view(&self) -> Html {
    html! {
      <main id="yewssenger">
        <YewssengerRouter/>
      </main>
    }
  }
}
