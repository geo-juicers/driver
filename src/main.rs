mod api;

fn get_state(state: &str) -> u8 {
    match state {
        "Alabama"        => 1,
        "Alaska"         => 2,
        "Arizona"        => 3,
        "Arkansas"       => 4,
        "California"     => 5,
        "Colorado"       => 6,
        "Connecticut"    => 7,
        "Delaware"       => 8,
        "Florida"        => 9,
        "Georgia"        => 10,
        "Hawaii"         => 11,
        "Idaho"          => 12,
        "Illinois"       => 13,
        "Indiana"        => 14,
        "Iowa"           => 15,
        "Kansas"         => 16,
        "Kentucky"       => 17,
        "Louisiana"      => 18,
        "Maine"          => 19,
        "Maryland"       => 20,
        "Massachusetts"  => 21,
        "Michigan"       => 22,
        "Minnesota"      => 23,
        "Mississippi"    => 24,
        "Missouri"       => 25,
        "Montana"        => 26,
        "Nebraska"       => 27,
        "Nevada"         => 28,
        "New Hampshire"  => 29,
        "New Jersey"     => 30,
        "New Mexico"     => 31,
        "New York"       => 32,
        "North Carolina" => 33,
        "North Dakota"   => 34,
        "Ohio"           => 35,
        "Oklahoma"       => 36,
        "Oregon"         => 37,
        "Pennsylvania"   => 38,
        "Rhode Island"   => 39,
        "South Carolina" => 40,
        "South Dakota"   => 41,
        "Tennessee"      => 42,
        "Texas"          => 43,
        "Utah"           => 44,
        "Vermont"        => 45,
        "Virginia"       => 46,
        "Washington"     => 47,
        "West Virginia"  => 48,
        "Wisconsin"      => 49,
        "Wyoming"        => 50,
        _ => 0,
    }
}

fn main() {
    let ports = serialport::available_ports().expect("No ports found!");
    for p in ports {
        println!("{}", p.port_name);
    }
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
