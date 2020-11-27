use chrono::prelude::*;
use chrono::Duration;
use chrono::DateTime;
use std::env;
#[derive(Debug)]
enum Text{
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
struct CheckList {
  items : Vec<ListItem>
}
impl CheckList{
  fn display(& self, level : usize){
    for item in &self.items {
      &item.display(level + 1);
    }
  }
}

#[derive(Debug)]
struct ListItem {
  content : Vec<Text>
}
impl ListItem{
  fn from_text(content : Text) -> ListItem{
    ListItem{
      content : vec![content]
    }
  }
  fn display(& self, level : usize){
    let indent = std::iter::repeat("  ").take(level).collect::<String>();
    print!("\n{}- [ ] ",indent);
    for segment in &self.content{
      &segment.display(level);
    }
  }
}
#[derive(Debug)]
struct Node {
  title : String,
  scheduled : Option<DateTime<Utc>>,
  children : Vec<Node>,
  content : Vec<Text>
}
impl Node{
  fn display (self : & Node, level: usize){
    let stars = std::iter::repeat("*").take(level).collect::<String>();
    println!("{} {}",stars,self.title);
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
struct NodeBuilder {
  title : String,
  scheduled : Option<DateTime<Utc>>,
  children : Vec<Node>
}
impl NodeBuilder {
  fn new(title : String) -> NodeBuilder{
    NodeBuilder{
      title : title,
      scheduled : None,
      children : Vec::new()
    }
  }
  fn add_children(mut self  , children : Vec<Node>)-> NodeBuilder{
    self.children = children;
    self
  }
  fn build(self) -> Node{
    Node{
      title : self.title,
      scheduled : self.scheduled,
      children : self.children,
      content : Vec::new()
    }
  }
}

fn generate_list() -> CheckList{
  CheckList{
    items : (vec![
      "insert new activities",
      "update jira",
      "read emails"
    ]).iter().map(
      |x| ListItem::from_text(Text::PlainText(x.to_string()))
    ).collect()
  }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("received args: {:?}", args);
    if let Some(value) = args.get(1) {
      let rdt : Result<DateTime<Utc>,_> = Utc.datetime_from_str(value, "%Y-%m-%d %H:%M:%S");
      match rdt{
        Ok(dt) => {
          println!("starting date: {}",dt.to_string());
          let nodes : Vec<Node> = (0..30).
            map(|i| 
            Node {
              title : String::from("Daily planning"),
              scheduled : Some(dt + Duration::days(i)),
              children : Vec::new(),
              content : vec![Text::PlainText("Plan, Do, Check, Act".into()), 
                             Text::ChecklistText(generate_list())
                             ]
            }).collect();
          let month = dt.format("%B %Y").to_string();
          let month_node = NodeBuilder::new(String::from(month)).add_children(nodes).build();
          let planning_node = NodeBuilder::new("Planning".to_string()).add_children(vec![month_node]).build();
          let root = NodeBuilder::new("Group".to_string()).add_children(vec![planning_node]).build();
          root.display(1);
        },
        Err(m) => println!("parse failed of '{}': {}",value,m)
      }
    } else {
      println!("need at least one input")
    }
}
