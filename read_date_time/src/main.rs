use chrono::prelude::*;
use chrono::Duration;
use chrono::DateTime;
use std::env;
mod org;

fn generate_list() -> org::CheckList{
  org::CheckList{
    items : (vec![
      "insert new activities",
      "update jira",
      "read emails"
    ]).iter().map(
      |x| org::ListItem::from_text(org::Text::PlainText(x.to_string()))
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
          let mut nodes : Vec<org::Node> = (0..30).
            map(|i| 
            org::Node {
              title : String::from("Daily planning"),
              todo : Some("TODO".into()),
              priority : Some(org::Priority::A),
              scheduled : Some(dt + Duration::days(i)),
              children : Vec::new(),
              content : vec![org::Text::PlainText("Plan, Do, Check, Act".into()), 
                             org::Text::ChecklistText(generate_list())
                             ]
            }).collect();
          let mut nodes2 : Vec<org::Node> = (0..30).
            map(|i|
              dt + Duration::days(i)
            ).filter(|d|
            d.weekday() == Weekday::Tue
            ).map(|d| 
            org::Node {
              title : String::from("Software Weekly"),
              todo : None,
              priority : None,
              scheduled : Some(d),
              children : Vec::new(),
              content : vec![]
            }).collect();
          let mut nodes3 : Vec<org::Node> = (0..30).
            map(|i|
              dt + Duration::days(i)
            ).filter(|d|
            d.weekday() == Weekday::Tue
            ).map(|d| 
            org::Node {
              title : String::from("Software Update"),
              todo : None,
              priority : None,
              scheduled : Some(d),
              children : Vec::new(),
              content : vec![]
            }).collect();
          let mut nodes1 : Vec<org::Node> = (0..30).
            map(|i|
              dt + Duration::days(i)
            ).filter(|d|
            d.weekday() == Weekday::Mon
            ).map(|d| 
            org::Node {
              title : String::from("Send Accountability"),
              todo : Some("TODO".into()),
              priority : Some(org::Priority::B),
              scheduled : Some(d),
              children : Vec::new(),
              content : vec![]
            }).collect();
          let mut nodes4 : Vec<org::Node> = (0..30).
            map(|i|
              dt + Duration::days(i)
            ).filter(|d|
            d.weekday() == Weekday::Thu
            ).map(|d| 
            org::Node {
              title : String::from("Technical Staff"),
              todo : None,
              priority : None,
              scheduled : Some(d),
              children : Vec::new(),
              content : vec![]
            }).collect();
          nodes.append(&mut nodes1);
          nodes.append(&mut nodes2);
          nodes.append(&mut nodes3);
          nodes.append(&mut nodes4);
          let month = dt.format("%B %Y Planning").to_string();
          let month_node = org::NodeBuilder::new(String::from(month)).add_children(nodes).set_todo("TODO".into()).build();
          let planning_node = org::NodeBuilder::new("Planning".to_string()).add_children(vec![month_node]).build();
          let root = org::NodeBuilder::new("Group".to_string()).add_children(vec![planning_node]).build();
          root.display(1);
        },
        Err(m) => println!("parse failed of '{}': {}",value,m)
      }
    } else {
      println!("need at least one input")
    }
}
