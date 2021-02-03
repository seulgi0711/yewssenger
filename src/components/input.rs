use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Input {
  link: ComponentLink<Self>,
}

pub enum Msg {
  None,
}

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
  fn view(&self) -> yew::virtual_dom::VNode {
    todo!()
  }
}
