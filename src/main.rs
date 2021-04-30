use dialoguer::Input;
use regex::Regex;
use reqwest;
use serde_json::{json, Map, Value};
use std::{env, io, process::Command};
use structopt::StructOpt;

#[derive(Debug, StructOpt, PartialEq)]
#[structopt(
    name = "Jirard",
    about = "Your personal Jira butler. An easy to use Jira CLI."
)]
struct Opt {
    /// Provide the Jira issue key for issue actions
    #[structopt(short, long, default_value)]
    issue: String,

    /// Comment to be added to jira issue provided
    #[structopt(short, long, default_value)]
    comment: String,
}

#[derive(Debug)]
struct JiraClient {
    user: String,
    pass: String,
    api: String,
}

impl JiraClient {
    fn comment(
        &self,
        issue: String,
        comment: String,
    ) -> Result<Map<String, Value>, reqwest::Error> {
        let comment_body = json!({
            "body": comment,
            "visibility":  {}
        });

        let request_url = format!("{}/issue/{}/comment", self.api, issue,);
        reqwest::blocking::Client::new()
            .post(request_url)
            .basic_auth(&self.user, Some(&self.pass))
            .json(&comment_body)
            .send()?
            .json::<Map<String, Value>>()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let client = JiraClient {
        user: get_env_prompt("JIRA_USER")
            .expect("Could not parse Jira username")
            .to_string(),
        pass: get_env_prompt("JIRA_PASS")
            .expect("Could not parse Jira password")
            .to_string(),
        api: get_env_prompt("JIRA_API")
            .expect("Could not parse Jira api")
            .to_string(),
    };

    println!("{:#?}", client);

    if opt.comment.len() > 0 {
        let issue = if opt.issue.len() > 0 {
            opt.issue
        } else {
            let branch_name = String::from_utf8(
                Command::new("git")
                    .arg("branch")
                    .arg("--show-current")
                    .output()
                    .expect("git command failed to start")
                    .stdout,
            )?;
            parse_jira_issue(branch_name).expect("Could not parse Jira issue key from branch name. Please provide issue key in arguments.")
        };
        client.comment(issue, opt.comment)?;
    };

    Ok(())
}

fn parse_jira_issue(branch_name: String) -> Result<String, Box<dyn std::error::Error>> {
    let pattern = Regex::new("([A-Z][A-Z0-9]+-[0-9]+)")?;
    let issue_key = pattern.captures(&branch_name).ok_or("Could not parse Jira issue key from branch name. Please provide issue key in arguments.")?.get(1).map_or("", |m| m.as_str());
    Ok(String::from(&*issue_key))
}

fn get_env_prompt(var: &str) -> io::Result<String> {
    if let Ok(value) = env::var(var) {
        Ok(value)
    } else {
        Ok(Input::new().with_prompt(var).interact()?)
    }
}
