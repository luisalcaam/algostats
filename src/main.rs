mod domain;
mod service;

use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use std::error::Error;
use domain::question::Question;
use domain::submission::Submission;
use service::response::CodeforcesApiResponse;

fn main() -> Result<(), Box<dyn Error>> {

    let url = "https://codeforces.com/api/user.status?handle=varnothing&from=1&count=10";
    
    let http_client = Client::new();

    let api_response: CodeforcesApiResponse<Vec<Submission>> = http_client.get(url).send()?.json()?;
    let submissions : Vec<Submission> = api_response.result;

    for submission in &submissions {
        
        println!("{:?}",&submission.id);
        println!("Question: ");
        println!("{:?}",&submission.question.contestId);
        println!("Name: {:?}",&submission.question.name);
        println!("Rating: {:?}",&submission.question.rating);
        println!("Tags: {:?}",&submission.question.tags);
        println!("{:?}",&submission.verdict);
    }
    //println!("{:?}",submissions);


    Ok(())
}
