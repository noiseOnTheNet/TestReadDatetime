use chrono::prelude::*;
use chrono::Duration;
use chrono::DateTime;
use std::collections::HashMap;
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
              properties : HashMap::new(),
              scheduled : Some(dt + Duration::days(i)),
              interval : None,
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
            ).map(|d| {
              let mut properties = HashMap::new();
              properties.insert(
                "LOCATION".to_string(),
                "zoom".to_string()
              );
              properties.insert(
                "ATTENDEES".to_string(),
               "agrossi, odonzel, acalloni".to_string()
              );
              org::Node {
                title : String::from("Software Weekly"),
                todo : None,
                priority : None,
                properties : properties,
                scheduled : Some(d),
                interval : None,
                children : Vec::new(),
                content : vec![]
              }
            }
            ).collect();
          let mut nodes3 : Vec<org::Node> = (0..30).
            map(|i|
              dt + Duration::days(i)
            ).filter(|d|
            d.weekday() == Weekday::Tue
            ).map(|d| {
              let mut properties = HashMap::new();
              properties.insert(
                "LOCATION".to_string(),
                "zoom".to_string()
              );
              properties.insert(
                "ATTENDEES".to_string(),
                "snygard, acalloni, odonzel, ksalk, ankushc, avaranasi".to_string()
              );
              org::Node {
                title : String::from("Software Update"),
                todo : None,
                priority : None,
                properties : properties,
                scheduled : Some(d),
                interval : None,
                children : Vec::new(),
                content : vec![]
              }
            }
            ).collect();
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
              properties : HashMap::new(),
              scheduled : Some(d),
              interval : None,
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
              properties : HashMap::new(),
              scheduled : Some(d),
              interval : None,
              children : Vec::new(),
              content : vec![]
            }).collect();
          let mut nodes5 : Vec<org::Node> = (0..30).
            map(|i|
              dt + Duration::days(i)
            ).filter(|d|
            d.weekday() == Weekday::Mon
            ).map(|d| 
            org::Node {
              title : String::from("Web Services"),
              todo : None,
              priority : None,
              properties : HashMap::new(),
              scheduled : None,
              interval : Some((d,d)),
              children : Vec::new(),
              content : vec![]
            }).collect();
          nodes.append(&mut nodes1);
          nodes.append(&mut nodes2);
          nodes.append(&mut nodes3);
          nodes.append(&mut nodes4);
          nodes.append(&mut nodes5);
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
