use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};
use yew::Properties;

#[derive(Properties, Clone)]
pub struct Props {
  pub icon: String,
}

pub struct Icon {
  props: Props,
}

pub enum Msg {}

impl Component for Icon {
  type Message = Msg;
  type Properties = Props;
  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Icon { props }
  }
  fn update(&mut self, _: Self::Message) -> ShouldRender {
    todo!()
  }
  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    todo!()
  }
  fn view(&self) -> Html {
    html! {
      <img
        class="h-10 w-10 rounded-full"
        src=&self.props.icon
      />
    }
  }
}
