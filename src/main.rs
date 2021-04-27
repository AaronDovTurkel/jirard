// use regex::{NoExpand, Regex};
// use reqwest::blocking::Client;
// use serde_json::{json, Map, Value};
use std::env;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let jira_user = env::var("JIRA_USER")?;
    // let jira_pass = env::var("JIRA_PASS")?;

    // println!("user: {}, pass: {}", jira_user, jira_pass);

    let branch_name = Command::new("git")
        .arg("branch")
        .output()
        .expect("ls command failed to start");

    String::from_utf8(branch_name.stdout)?.lines().for_each(|x| println!("{:#?}", x));
    
    

    // let comment_body = json!({
    //     "body": "This is a test comment",
    //     "visibility":  {}
    // });

    // let request_url = format!(
    //     "https://jira.fabuwood.com/rest/api/2/issue/{}/comment",
    //     String::from("TESLA-3425"),
    // );
    // let response = Client::new()
    //     .post(request_url)
    //     .basic_auth(jira_user.clone(), Some(jira_pass.clone()))
    //     .json(&comment_body)
    //     .send()?
    //     .json::<Map<String, Value>>()?;

    // println!("Added Comment {:#?}", response);

    Ok(())
}

// fn parse_jira_issue(branch_name: &str) -> Vec<&str> {
//     let re = Regex::new("([A-Z][A-Z0-9]+-[0-9]+)").unwrap();
//     re.split(branch_name).collect()
// }
