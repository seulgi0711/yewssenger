use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Input {}

pub enum Msg {}

impl Component for Input {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
    todo!()
  }
  fn update(&mut self, _: Self::Message) -> ShouldRender {
    todo!()
  }
  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    todo!()
  }
  fn view(&self) -> Html {
    html! {}
  }
}
