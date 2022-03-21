use chrono::prelude::{Datelike, TimeZone, Utc, Weekday};
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
        ).flat_map(|d| {
            vec![
                org::create_meeting("Software Weekly", d, 14, 15, "agrossi,  acalloni", "zoom"),
                org::create_meeting("Software Update", d, 15, 16, "snygard, acalloni,  ksalk, naveen, avaranasi", "zoom")
            ].into_iter()
        }).collect();
    let mut nodes1 : Vec<org::Node> = (0..30).
        map(|i|
            dt + Duration::days(i)
        ).filter(|d|
                 d.weekday() == Weekday::Mon
        ).flat_map(|d| {
            vec![
                org::Node {
                    title : String::from("Send Accountability"),
                    todo : Some("TODO".into()),
                    priority : Some(org::Priority::B),
                    properties : HashMap::new(),
                    scheduled : Some(d),
                    interval : None,
                    children : Vec::new(),
                    content : vec![]
                }
            ].into_iter()
        }).collect();
    let mut nodes4 : Vec<org::Node> = (0..30).
        map(|i|
            dt + Duration::days(i)
        ).filter(|d|
                 d.weekday() == Weekday::Thu
        ).map(|d| {
            org::create_meeting("Technical Staff", d, 11, 12, "agrossi, lvendram, abenvenu, aghetti, friva", "zoom")
        }).collect();
    let mut nodes6 : Vec<org::Node> = (0..30).
        map(|i|
            dt + Duration::days(i)
        ).filter(|d|
                 d.weekday() == Weekday::Fri
        ).flat_map(|d| { 
            vec![
                org::create_meeting("Gel/TD", d, 10, 11,"pfilini; acalloni; friva", "zoom")
            ].into_iter()
        }).collect();
    let mut nodes8 : Vec<org::Node> = (0..30).
        map(|i|
            dt + Duration::days(i)
        ).filter(|d|
                 d.weekday() == Weekday::Wed &&
                 d.iso_week().week() % 2 == 0
        ).map(|d| { 
            org::create_meeting("SW Reliability", d, 10, 11,"mvezzoli; dventric; ngalbiat; rbottini; lvendram; agrossi; acalloni; svigano; lbortesi; trossi", "zoom")
        }).collect();
    let mut nodes9 : Vec<org::Node> = (0..30).
        map(|i|
            dt + Duration::days(i)
        ).filter(|d|
                 d.weekday() == Weekday::Wed &&
                 d.iso_week().week() % 2 == 1
        ).map(|d| { 
            org::create_meeting("TD/IT Review", d, 11, 12,"pmancini; friva; agrossi; mvezzoli; ppezzimenti;dspiniel; aattina; bbonini; pfilini", "zoom")
        }).collect();
    let mut nodes10 = vec! [
        org::NodeBuilder::new("Insert time leave").
            set_schedule(dt).
            set_priority(org::Priority::C).
            build()
    ];
    let mut nodes11 : Vec<org::Node> = (0..30).
        map(|i|
            dt + Duration::days(i)
        ).filter(|d|
                 (d.weekday() != Weekday::Sun) && 
                 (d.weekday() != Weekday::Sat)
        ).flat_map(|d| {
            vec![
                org::Node {
                    title : String::from("learn Data Science"),
                    todo : Some("TODO".into()),
                    priority : Some(org::Priority::A),
                    properties : HashMap::new(),
                    scheduled : Some(d),
                    interval : None,
                    children : Vec::new(),
                    content : vec![]
                }
            ].into_iter()
        }).collect();
    nodes.append(&mut nodes1);
    nodes.append(&mut nodes2);
    nodes.append(&mut nodes4);
    nodes.append(&mut nodes6);
    nodes.append(&mut nodes8);
    nodes.append(&mut nodes9);
    nodes.append(&mut nodes10);
    nodes.append(&mut nodes11);
    let month = dt.format("%Y %B Planning [/]").to_string();
    let month_node = org::NodeBuilder::new(month).add_children(nodes).set_todo("TODO").build();
    let planning_node = org::NodeBuilder::new("Planning").add_children(vec![month_node])
        .add_property("CATEGORY","Planning")
        .build();
    let root = org::NodeBuilder::new("Group").add_children(vec![planning_node]).build();
    root
}

fn data_analysis(dt:DateTime<Utc>) -> org::Node{
    let arda = org::NodeBuilder::new(dt.format("Arda %B %Y [%%]"))
        .set_todo("NEXT")
        .add_children(vec![
            org::NodeBuilder::new(dt.format("Arda Maintenance %B [/]")).set_todo("NEXT")
                .build(),
            org::NodeBuilder::new(dt.format("Arda Meetings %B [/]")).set_todo("NEXT")
                .build()
        ])
        .add_property("CATEGORY","Arda")
        .build();
    let webcalc = org::NodeBuilder::new(dt.format("WebCalc %B %Y [%%]"))
        .set_todo("NEXT")
        .add_children(vec![
            org::NodeBuilder::new(dt.format("Web Calculators Maintenance %B [/]"))
                .set_todo("NEXT")
                .build(),
            org::NodeBuilder::new(dt.format("Web Calculators Meetings %B [/]"))
                .set_todo("NEXT")
                .build()
        ])
        .add_property("CATEGORY","WebCalc")
        .build();
    let data_science = org::NodeBuilder::new(dt.format("Data Science %B %Y [%%]"))
        .set_todo("Next")
        .add_children(vec![
            org::NodeBuilder::new(dt.format("Data Science Development %B [/]"))
                .set_todo("NEXT").build(),
            org::NodeBuilder::new(dt.format("Data Science Meetings %B [/]"))
                .set_todo("NEXT").build()
        ])
        .add_property("CATEGOTY","DS")
        .build();
    let mut gdw_meetings : Vec<org::Node> = (0..30).
        map(|i|
            dt + Duration::days(i)
        ).filter(|d|
                 d.weekday() == Weekday::Fri
        ).map(|d| {
            org::create_meeting("GDW Working", d, 16, 17, "ksalk; deeabbott; stamboli; ijdembi; mvezzoli; ccardon; avaranasi", "zoom")
        }).collect();
    
    let mut gdw_meetings1 : Vec<org::Node> = (0..30).
        map(|i|
            dt + Duration::days(i)
        ).filter(|d|
                 d.weekday() == Weekday::Tue
        ).map(|d| { 
            org::create_meeting("Param Crunch in DFS", d, 9, 10,"scottlin; mvezzoli; avaranasi", "zoom")
        }).collect();

    let mut gdw_meetings2 : Vec<org::Node> = (0..30).
        map(|i|
            dt + Duration::days(i)
        ).filter(|d|
                 d.weekday() == Weekday::Wed
        ).map(|d| {
            org::create_meeting("Auto Plotter Working Meeting", d, 9, 10,"mvezzoli; avaranasi", "zoom")
        }).collect();
    let mut gdw_meetings3 : Vec<org::Node> = (0..30).
        map(|i|
            dt + Duration::days(i)
        ).filter(|d|
                 d.weekday() == Weekday::Wed
        ).map(|d| { 
            org::create_meeting("GDW Cruncher and Reporter", d, 16, 17,"snygard; mvezzoli; avaranasi", "zoom")
        }).collect();

    gdw_meetings.append(&mut gdw_meetings1);
    gdw_meetings.append(&mut gdw_meetings2);
    gdw_meetings.append(&mut gdw_meetings3);
    
    let gdw = org::NodeBuilder::new(dt.format("GDW %B %Y [%%]"))
        .set_todo("NEXT")
        .add_children(vec![
            org::NodeBuilder::new(dt.format("GDW Maintenance %B [/]"))
                .set_todo("NEXT")
                .build(),
            org::NodeBuilder::new(dt.format("GDW Meetings %B [/]"))
                .set_todo("NEXT")
                .add_children(gdw_meetings)
                .build()
        ])
        .add_property("CATEGORY","GDW")
        .build();
    let root = org::NodeBuilder::new("Data Analysis")
        .add_children(vec![arda, data_science, webcalc, gdw]).build();
    root
}

fn lab_infrastr(dt:DateTime<Utc>) -> org::Node{
    let gel = org::NodeBuilder::new(dt.format("GEL %B %Y [%%]"))
        .set_todo("NEXT")
        .add_children(vec![
            org::NodeBuilder::new(dt.format("GEL Maintenance %B [/]"))
                .set_todo("NEXT")
                .build(),
            org::NodeBuilder::new(dt.format("GEL Meetings %B [/]"))
                .set_todo("NEXT")
                .build()
        ])
        .add_property("CATEGORY","GEL")
        .build();
    let masterbook = org::NodeBuilder::new(dt.format("Masterbook %B %Y [%%]"))
        .set_todo("NEXT")
        .add_children(vec![
            org::NodeBuilder::new(dt.format("Masterbook Maintenance %B [/]"))
                .set_todo("NEXT")
                .build(),
            org::NodeBuilder::new(dt.format("Masterbook Meetings %B [/]"))
                .set_todo("NEXT")
                .build()
        ])
        .add_property("CATEGORY","MB2")
        .build();
    let pycron = org::NodeBuilder::new(dt.format("Pycron %B %Y [%%]"))
        .set_todo("NEXT")
        .add_children(vec![
            org::NodeBuilder::new(dt.format("Pycron Maintenance %B [/]"))
                .set_todo("NEXT")
                .build(),
            org::NodeBuilder::new(dt.format("Pycron Meetings %B [/]"))
                .set_todo("NEXT")
                .build()
        ])
        .add_property("CATEGORY","PYCRON")
        .build();
    let root = org::NodeBuilder::new("Infrastructure")
        .add_children(vec![gel, masterbook, pycron])
        .build();
    root
}

fn division_support(dt:DateTime<Utc>) -> org::Node{
    let patm = org::NodeBuilder::new(dt.format("PATM %B %Y [%%]"))
        .set_todo("NEXT")
        .add_children(vec![
            org::NodeBuilder::new(dt.format("PATM Maintenance %B [/]"))
                .set_todo("NEXT")
                .build(),
            org::NodeBuilder::new(dt.format("PATM Meetings %B [/]"))
                .set_todo("NEXT")
                .build()
        ])
        .add_property("CATEGORY","PATM")
        .build();
    let root = org::NodeBuilder::new("TPG").add_children(vec![patm]).build();
    root
}

fn personal(dt:DateTime<Utc>) -> org::Node{
    let mut alice1 : Vec<org::Node> = (0..30).
        map(|i|
            dt + Duration::days(i)
        ).filter(|d|
                 d.weekday() == Weekday::Thu
        ).map(|d| { 
            org::NodeBuilder::new("Fisica")
                .add_property("ATTENDEES","Alice Filisetti; Andrea Cenati")
                .add_property("LOCATION","zoom")
                .set_interval(
                    d + Duration::hours(20) + Duration::minutes(30),
                    d + Duration::hours(21) + Duration::minutes(30)
                )
                .build()
        }).collect();
    let mut alice2 : Vec<org::Node> = (0..30).
        map(|i  |
            dt + Duration::days(i)
        ).filter(|d|
                 d.weekday() == Weekday::Sat
        ).map(|d| { 
            org::NodeBuilder::new("Fisica")
                .add_property("ATTENDEES","Alice Filisetti; Andrea Cenati")
                .add_property("LOCATION","zoom")
                .set_interval(
                    d + Duration::hours(16),
                    d + Duration::hours(18)
                )
                .build()
        }).collect();
    let mut anna : Vec<org::Node> = (0..30).
        map(|i|
            dt + Duration::days(i)
        ).filter(|d|
                 d.weekday() == Weekday::Sat
        ).map(|d| { 
            org::NodeBuilder::new("Lezione")
                .add_property("ATTENDEES","Anna Vezzoli")
                .add_property("LOCATION","casa")
                .set_interval(
                    d + Duration::hours(20),
                    d + Duration::hours(22)
                )
                .build()
        }).collect();
    alice1.append(&mut alice2);
    alice1.append(&mut anna);
    let lesson_node = org::NodeBuilder::new(dt.format("Lezioni %B %Y [/]")).add_children(alice1)
        .add_property("CATEGORY","Lezioni")
        .build();
    let lessons : Vec<org::Node> = vec![lesson_node];
    let root = org::NodeBuilder::new(dt.format("Personale %B %Y [/]")).add_children(lessons)
        .build();
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
                let if_root = lab_infrastr(dt);
                if_root.display(1);
                let dv_supp = division_support(dt);
                dv_supp.display(1);
                let not_work = personal(dt);
                not_work.display(1);
            },
            Err(m) => println!("parse failed of '{}': {}",value,m)
        }
    } else {
        println!("need at least one input")
    }
}
