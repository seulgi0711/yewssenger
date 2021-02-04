#[derive(Clone, Debug)]
pub enum ChatType {
  // Me,
  // Private,
  Direct,
  // Group,
}

#[derive(Clone, Debug)]
pub struct Chat {
  pub id: String,
  pub icon: String,
  pub title: String,
  pub members: Vec<String>,
  pub r#type: ChatType,
}
