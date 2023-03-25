use dotenv;
use std::error::Error;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Population {
    pub total: Option<u32>,
    pub median_age: Option<u32>,
    pub hisp_pop: Option<u32>,
    pub non_hisp_pop: Option<u32>,
}

#[derive(Debug)]
pub struct Economic {
    test: i32,
}

#[derive(Debug)]
pub struct State {
    pub name: String,
    pub pop: Option<Population>,
    pub econ: Option<Economic>,
}

#[tokio::main]
pub async fn get_states(api_key: String) -> Result<Vec<State>, Box<dyn Error>>{
    let request_url = format!("https://api.census.gov/data/2021/acs/acs5?get=NAME,B01001_001E,B01002_001E&for=state:*&key={}", api_key);
    let states = reqwest::get(&request_url)
        .await?
        .json::<Vec<Vec<String>>>()
        .await?[1..].iter()
        .map(|x| State{ 
            name: x[0].clone(),
            pop: Some(Population {
                total: Some(x[1].as_str().parse::<u32>().unwrap()),
                median_age: Some(x[2].as_str().parse::<f32>().unwrap() as u32),
                hisp_pop: None,
                non_hisp_pop: None,
            }),
            econ: None,
        })
        .collect::<Vec<State>>();
    
    Ok(states)
}

