mod api;

fn get_state(state: &str) -> u8 {
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

fn main() {
    let api_key:String = dotenv::var("API_KEY").unwrap();
    let states: Vec::<api::State> = api::get_states_acs(&api_key, "2021").unwrap();
    println!("{:#?}", states);
    //for year in vec!["2009", "2010", "2011", "2012", "2013", "2014", "2015", "2016", "2017", "2018", "2019", "2020", "2021"].iter() {
    //    //let states: Vec::<api::State> = api::get_states_acs(&api_key, year).unwrap();
    //    api::get_states_acs(&api_key, year).unwrap();
    //    println!("{}", year)
    //}
    //for state in states {
    //    println!("{}: {}", state.name, state.pop.unwrap().total.unwrap());
    //}
}
