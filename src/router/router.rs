use yew::prelude::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::{router::Router, Switch};

use crate::pages::Home;

pub struct YewssengerRouter {}

pub enum Msg {}

#[derive(Switch, Clone)]
pub enum AppRoute {
  #[to = "/"]
  Index,
}

impl Component for YewssengerRouter {
  type Message = Msg;
  type Properties = ();
  fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
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
      <Router<AppRoute, ()>
        render = Router::render(|switch: AppRoute| {
          match switch {
            AppRoute::Index => html! {<Home/>}
          }
        })
      />
    }
  }
}
