use std::error::Error;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Population {
    pub total: Option<u32>,
    pub median_age: Option<u32>,
    pub hisp_pop: Option<u32>,
}

#[derive(Debug)]
pub struct Economic {
    pub median_household_income: Option<u32>,
    pub median_gross_rent: Option<u32>,
    pub median_home_value: Option<u32>,
    pub percentage_poor: Option<f32>,
}

#[derive(Debug)]
pub struct State {
    pub name: String,
    pub pop: Option<Population>,
    pub econ: Option<Economic>,
}

#[tokio::main]
pub async fn get_states_acs(api_key: &str, year: &str) -> Result<Vec<State>, Box<dyn Error>>{
    let request_url = format!("https://api.census.gov/data/{}/acs/acs5?get=NAME,{},{},{},{},{},{},{}&for=state:*&key={}",
                              year,
                              "B01003_001E", //total population or B01001_001E
                              "B01002_001E", //median age
                              "B03001_001E", //hispanic population
                              "B19013_001E", //median household income
                              "B25024_001E", //median gross rent
                              "B25035_001E", //median home value
                              "B17001_002E", //total below poverty line
                              api_key);
    let states = reqwest::get(&request_url)
        .await?
        .json::<Vec<Vec<String>>>()
        .await?[1..].iter()
        .map(|x| State{ 
            name: x[0].clone(),
            pop: Some(Population {
                total: Some(x[1].as_str().parse::<u32>().unwrap()),
                median_age: Some(x[2].as_str().parse::<f32>().unwrap() as u32),
                hisp_pop: Some(x[3].as_str().parse::<u32>().unwrap()),
            }),
            econ: Some(Economic {
                median_household_income: Some(x[4].as_str().parse::<u32>().unwrap()),
                median_gross_rent: Some(x[5].as_str().parse::<u32>().unwrap()),
                median_home_value: Some(x[6].as_str().parse::<u32>().unwrap()),
                percentage_poor: Some(x[7].as_str().parse::<f32>().unwrap()/x[1].as_str().parse::<f32>().unwrap()),
            }),
        })
        .collect::<Vec<State>>();
    
    Ok(states)
}

pub fn get_state_id(state: &str) -> u8 {
    match state {
        "Alabama"        => 47,
        "Alaska"         => 41,
        "Arizona"        => 31,
        "Arkansas"       => 33,
        "California"     => 21,
        "Colorado"       => 32,
        "Connecticut"    => 17,
        "Delaware"       => 28,
        "Florida"        => 49,
        "Georgia"        => 48,
        "Hawaii"         => 44,
        "Idaho"          => 12,
        "Illinois"       => 23,
        "Indiana"        => 26,
        "Iowa"           => 13,
        "Kansas"         => 34,
        "Kentucky"       => 36,
        "Louisiana"      => 46,
        "Maine"          => 9,
        "Maryland"       => 27,
        "Massachusetts"  => 19,
        "Michigan"       => 6,
        "Minnesota"      => 5,
        "Mississippi"    => 40,
        "Missouri"       => 35,
        "Montana"        => 2,
        "Nebraska"       => 25,
        "Nevada"         => 22,
        "New Hampshire"  => 8,
        "New Jersey"     => 29,
        "New Mexico"     => 42,
        "New York"       => 0,
        "North Carolina" => 38,
        "North Dakota"   => 4,
        "Ohio"           => 16,
        "Oklahoma"       => 43,
        "Oregon"         => 11,
        "Pennsylvania"   => 10,
        "Rhode Island"   => 18,
        "South Carolina" => 37,
        "South Dakota"   => 15,
        "Tennessee"      => 30,
        "Texas"          => 45,
        "Utah"           => 24,
        "Vermont"        => 7,
        "Virginia"       => 39,
        "Washington"     => 1,
        "West Virginia"  => 20,
        "Wisconsin"      => 3,
        "Wyoming"        => 14,
        _ => 0,
    }
}

