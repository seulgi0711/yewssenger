use crate::components::icon::Icon;
use crate::types::User;
use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};
use yew::Properties;

#[derive(Properties, Clone)]
pub struct Props {
  pub user: User,
}

pub struct UserIcon {
  props: Props,
}

impl Component for UserIcon {
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
    let User { icon, id, .. } = &self.props.user;

    html! {
      <Icon icon=format!("{}?{}", icon, id)/>
    }
  }
}
