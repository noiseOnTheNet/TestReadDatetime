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

fn planning(dt:DateTime<Utc>) -> org::Node{
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
              org::NodeBuilder::new("Software Weekly")
              .add_property("ATTENDEES","agrossi, odonzel, acalloni")
              .add_property("LOCATION","zoom")
              .set_interval(
                d + Duration::hours(14),
                d + Duration::hours(15)
              )
              .build()
            }).collect();
          let mut nodes3 : Vec<org::Node> = (0..30).
            map(|i|
              dt + Duration::days(i)
            ).filter(|d|
            d.weekday() == Weekday::Tue
            ).map(|d| {
              org::NodeBuilder::new("Software Update")
              .add_property("ATTENDEES","snygard, acalloni, odonzel, ksalk, ankushc, avaranasi")
              .add_property("LOCATION","zoom")
              .set_interval(
                d + Duration::hours(15),
                d + Duration::hours(16)
              )
              .build()
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
            ).map(|d| {
              org::NodeBuilder::new("Technical Staff")
              .add_property("ATTENDEES","agrossi, lvendram, abenvenu, aghetti, friva")
              .add_property("LOCATION","zoom")
              .set_interval(
                d + Duration::hours(11),
                d + Duration::hours(12) + Duration::minutes(30)
              )
              .build()
            }).collect();
          let mut nodes5 : Vec<org::Node> = (0..30).
            map(|i|
              dt + Duration::days(i)
            ).filter(|d|
            d.weekday() == Weekday::Mon
            ).map(|d| { 
              org::NodeBuilder::new("Web Services")
              .add_property("ATTENDEES","snygard; avaranasi")
              .add_property("LOCATION","zoom")
              .set_interval(
                d + Duration::hours(16),
                d + Duration::hours(17)
              )
              .build()
            }).collect();
          let mut nodes6 : Vec<org::Node> = (0..30).
            map(|i|
              dt + Duration::days(i)
            ).filter(|d|
            d.weekday() == Weekday::Fri
            ).map(|d| { 
              org::NodeBuilder::new("GDW")
              .add_property("ATTENDEES","snygard; ksalk; deeabbott; stamboli; ijdembi; mvezzoli; ccardon; avaranasi; kaflorent")
              .add_property("LOCATION","zoom")
              .set_interval(
                d + Duration::hours(17),
                d + Duration::hours(18)
              )
              .build()
            }).collect();
          let mut nodes7 : Vec<org::Node> = (0..30).
            map(|i|
              dt + Duration::days(i)
            ).filter(|d|
            d.weekday() == Weekday::Fri
            ).map(|d| { 
              org::NodeBuilder::new("Gel/TD")
              .add_property("ATTENDEES","pfilini; odonzel; acalloni; friva")
              .add_property("LOCATION","zoom")
              .set_interval(
                d + Duration::hours(10),
                d + Duration::hours(11)
              )
              .build()
            }).collect();
          nodes.append(&mut nodes1);
          nodes.append(&mut nodes2);
          nodes.append(&mut nodes3);
          nodes.append(&mut nodes4);
          nodes.append(&mut nodes5);
          nodes.append(&mut nodes6);
          nodes.append(&mut nodes7);
          let month = dt.format("%Y %B Planning").to_string();
          let month_node = org::NodeBuilder::new(month).add_children(nodes).set_todo("TODO".into()).build();
          let planning_node = org::NodeBuilder::new("Planning").add_children(vec![month_node]).build();
          let root = org::NodeBuilder::new("Group").add_children(vec![planning_node]).build();
          root
}

fn data_analysis(dt:DateTime<Utc>) -> org::Node{
  let arda = org::NodeBuilder::new(dt.format("Arda %Y %B"))
  .add_children(vec![
    org::NodeBuilder::new(dt.format("Arda Maintenance %B")).build(),
    org::NodeBuilder::new(dt.format("Arda Meetings %B")).build()
  ]).build();
  let webcalc = org::NodeBuilder::new(dt.format("Web Calculators %Y %B"))
  .add_children(vec![
    org::NodeBuilder::new(dt.format("Web Calculators Maintenance %B")).build(),
    org::NodeBuilder::new(dt.format("Web Calculators Meetings %B")).build()
  ]).build();
  let root = org::NodeBuilder::new("Data Analysis").add_children(vec![arda, webcalc]).build();
  root
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("received args: {:?}", args);
    if let Some(value) = args.get(1) {
      let rdt : Result<DateTime<Utc>,_> = Utc.datetime_from_str(value, "%Y-%m-%d %H:%M:%S");
      match rdt{
        Ok(dt) => {
          println!("starting date: {}",dt.to_string());
          let plan_root = planning(dt);
          plan_root.display(1);
          let da_root = data_analysis(dt);
          da_root.display(1);
        },
        Err(m) => println!("parse failed of '{}': {}",value,m)
      }
    } else {
      println!("need at least one input")
    }
}
