use crate::components::icon::Icon;
use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};
use yew::Properties;

use crate::types::Chat;

#[derive(Properties, Clone, Debug)]
pub struct Props {
  pub chat: Chat,
}

pub struct ChatIcon {
  props: Props,
}

impl Component for ChatIcon {
  type Message = ();
  type Properties = Props;
  fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
    Self { props }
  }
  fn update(&mut self, _: Self::Message) -> ShouldRender {
    todo!()
  }
  fn change(&mut self, _: Self::Properties) -> ShouldRender {
    todo!()
  }
  fn view(&self) -> Html {
    let Chat { icon, id, .. } = &self.props.chat;

    html! {
      <Icon icon=format!("{}?{}", icon, id)/>
    }
  }
}
