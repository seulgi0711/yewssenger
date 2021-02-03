use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};
use yew::Properties;

use crate::types::User;

pub struct ProfileImage {
  link: ComponentLink<Self>,
  user: User,
}

#[derive(Properties, Clone)]
pub struct Props {
  pub user: User,
}

pub enum Msg {
  None,
}

impl Component for ProfileImage {
  type Message = Msg;
  type Properties = Props;
  fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
    ProfileImage {
      link,
      user: props.user,
    }
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
        src=&self.user.image
      />
    }
  }
}
