mod api;
mod server;

fn main() {
    let _ports = serialport::available_ports().expect("No ports found!");
    //println!("{:#?}", ports);
    let api_key:String = dotenv::var("API_KEY").unwrap();
    let _states: Vec::<api::State> = api::get_states_acs(&api_key, "2021").unwrap();
    //println!("{:#?}", states);
    println!("Serving API @ localhost:8000");
    server::serve("localhost:8000");
    //for year in vec!["2009", "2010", "2011", "2012", "2013", "2014", "2015", "2016", "2017", "2018", "2019", "2020", "2021"].iter() {
    //    //let states: Vec::<api::State> = api::get_states_acs(&api_key, year).unwrap();
    //    api::get_states_acs(&api_key, year).unwrap();
    //    println!("{}", year)
    //}
    //for state in states {
    //    println!("{}: {}", state.name, state.pop.unwrap().total.unwrap());
    //}
}
