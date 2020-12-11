use chrono::prelude::*;
use chrono::DateTime;

#[derive(Debug)]
pub enum Text{
  PlainText(String),
  ChecklistText(CheckList)
}
impl Text{
  fn display(& self, level: usize){
    match self{
      Text::PlainText(content) => print!(" {}",content),
      Text::ChecklistText(list) => list.display(level)
    }
  }
}
#[derive(Debug)]
pub enum Priority{
  A,
  B,
  C,
  D
}
impl Priority{
  pub fn display(& self) -> String{
    match self {
      Priority::A => "[#A]".into(),
      Priority::B => "[#B]".into(),
      Priority::C => "[#C]".into(),
      Priority::D => "[#D]".into(),
    }
  }
}
#[derive(Debug)]
pub struct CheckList {
  pub items : Vec<ListItem>
}
impl CheckList{
  pub fn display(& self, level : usize){
    for item in &self.items {
      &item.display(level + 1);
    }
  }
}

#[derive(Debug)]
pub struct ListItem {
  pub content : Vec<Text>
}
impl ListItem{
  pub fn from_text(content : Text) -> ListItem{
    ListItem{
      content : vec![content]
    }
  }
  pub fn display(& self, level : usize){
    let indent = std::iter::repeat("  ").take(level).collect::<String>();
    print!("\n{}- [ ] ",indent);
    for segment in &self.content{
      &segment.display(level);
    }
  }
}
#[derive(Debug)]
pub struct Node {
  pub title : String,
  pub todo : Option<String>,
  pub priority : Option<Priority>,
  pub scheduled : Option<DateTime<Utc>>,
  pub children : Vec<Node>,
  pub content : Vec<Text>
}
impl Node{
  pub fn display (self : & Node, level: usize){
    let stars = std::iter::repeat("*").take(level).collect::<String>();
    print!("{} ",stars);
    if let Some(todo) = &self.todo {
      print!("{} ",todo);
    }
    if let Some(priority) = &self.priority {
      print!("{} ",priority.display());
    }
    println!("{}",self.title);
    if let Some(sd)=self.scheduled {
      println!("SCHEDULED: <{}-{:02}-{:02}>",sd.year(),sd.month(),sd.day());
    }
    for segment in &self.content{
      &segment.display(0);
      println!("")
    }
    for c in &self.children{
      c.display(level + 1);
    }  
  }
}
pub struct NodeBuilder {
  title : String,
  todo : Option<String>,
  priority : Option<Priority>,
  scheduled : Option<DateTime<Utc>>,
  children : Vec<Node>
}
impl NodeBuilder {
  pub fn new(title : String) -> NodeBuilder{
    NodeBuilder{
      title : title,
      todo : None,
      priority : None,
      scheduled : None,
      children : Vec::new()
    }
  }
  pub fn add_children(mut self  , children : Vec<Node>)-> NodeBuilder{
    self.children = children;
    self
  }
  pub fn set_todo(mut self, todo : String) -> NodeBuilder {
    self.todo = Some(todo);
    self
  }
  pub fn set_priority(mut self, priority : Priority) -> NodeBuilder {
    self.priority = Some(priority);
    self
  }
  pub fn build(self) -> Node{
    Node{
      title : self.title,
      todo : self.todo,
      priority : self.priority,
      scheduled : self.scheduled,
      children : self.children,
      content : Vec::new()
    }
  }
}

