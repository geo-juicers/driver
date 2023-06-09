use rouille::*;
use serde::Serialize;
use crate::api::{get_states_acs, get_state_id, State};
use std::{thread, time};

#[derive(Serialize)]
struct Routes {
    routes: String,
    grad: Vec<String>,
    none: Vec<String>,
    all: Vec<String>,
    pop: Vec<String>,
    econ: Vec<String>
}

fn to_char(n: u8) -> u8 {
    match n {
        0 => 48,
        1 => 49,
        2 => 50,
        3 => 51,
        4 => 52,
        5 => 53,
        _ => 48
    }
}

fn handler(request: &Request, dev: &str) -> Response {
    router!(request,
        (GET) (/) => {
            //default
            Response::text("Driver API for geo-juicers, query /routes for all routes")
        },
        (OPTIONS) (/) => {
            //default       
            Response::text("OPTIONS")
                .with_unique_header("Access-Control-Allow-Origin", "*")
        },

        (GET) (/routes) => {
            // This route returns the list of notes. We perform the query and output it as JSON.
            Response::json(&Routes{
                routes: "show all routes".to_string(),
                grad: vec!["Gradient on all LEDs".to_string()],
                none: vec!["All LEDs off".to_string()],
                all: vec!["All LEDs on".to_string()],
                pop: vec![
                    "total".to_string(),
                    "median_age".to_string(),
                    "hispanic".to_string(),
                    "hispanic_per_capita".to_string()
                ],
                econ: vec![
                    "median_household_income".to_string(),
                    "income_hist".to_string(),
                    "median_gross_rent".to_string(),
                    "median_home_value".to_string(),
                    "poverty".to_string(),
                    "joined_union".to_string()
                ]
            }).with_unique_header("Access-Control-Allow-Origin", "*")
        },
        (GET) (/all) => {
            let mut arduino = serialport::new(dev, 9600).open().expect("Failed to open port");
            thread::sleep(time::Duration::from_secs(1));
            let mut data: [u8; 51] = [0; 51];
            for n in 0..50 {
                data[n] = 52
            }
            data[50] = '\n' as u8;
            arduino.write(&data).expect("Write failed!");
            Response::text("Turned all LEDs to 5")
        },
        (GET) (/none) => {
            let mut arduino = serialport::new(dev, 9600).open().expect("Failed to open port");
            thread::sleep(time::Duration::from_secs(1));
            let mut data: [u8; 51] = [0; 51];
            for n in 0..50 {
                data[n] = 48
            }
            data[50] = '\n' as u8;
            arduino.write(&data).expect("Write failed!");
            Response::text("Turned all LEDs to 0")
        },
        (GET) (/grad) => {
            let mut arduino = serialport::new(dev, 9600).open().expect("Failed to open port");
            thread::sleep(time::Duration::from_secs(1));
            for l in 0..10 {
                for n in 0..6 {
                    let mut data: [u8; 51] = [to_char(n); 51];
                    data[50] = '\n' as u8;
                    arduino.write(&data).expect("Write failed!");
                    thread::sleep(time::Duration::from_millis(500));
                }
            }
            Response::text("Running Gradiant on LEDs")
        },
        (GET) (/{category: String}/{animation: String}) => {
            let api_key:String = dotenv::var("API_KEY").unwrap();
            let mut arduino = serialport::new(dev, 9600).open().expect("Failed to open port");
            thread::sleep(time::Duration::from_millis(500));
            let mut states: Vec::<State> = get_states_acs(&api_key, "2021").unwrap();
            //remove DC and Puerto Rico
            states.retain(|state| state.name != "District of Columbia" && state.name != "Puerto Rico");
            let mut data: [u8; 51] = [48; 51];
            match category.as_str() {
                "pop" => {
                    match animation.as_str() {
                        "total" => {
                            let max = states.iter().fold(0.0f32, |max_val, state| (state.pop.as_ref().unwrap().total.unwrap() as f32).max(max_val));
                            for state in states.iter() {
                                let mut brightness = ((state.pop.as_ref().unwrap().total.unwrap() as f32/max) * 5.0) as u8;
                                if brightness < 5 {
                                    brightness += 1
                                }
                                assert!(brightness <= 5);
                                data[get_state_id(&state.name)] = to_char(brightness);
                            }
                            data[50] = '\n' as u8;
                            arduino.write(&data).expect("Write failed!");
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation)).with_unique_header("Access-Control-Allow-Origin", "*")
                        },
                        "median_age" => {
                            let max = states.iter().fold(0.0f32, |max_val, state| (state.pop.as_ref().unwrap().median_age.unwrap() as f32).max(max_val));
                            for state in states.iter() {
                                let brightness = ((state.pop.as_ref().unwrap().median_age.unwrap() as f32/max) * 5.0) as u8;
                                assert!(brightness <= 5);
                                data[get_state_id(&state.name)] = to_char(brightness);
                            }
                            data[50] = '\n' as u8;
                            arduino.write(&data).expect("Write failed!");
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation)).with_unique_header("Access-Control-Allow-Origin", "*")
                        },
                        "hispanic" => {
                            let max = states.iter().fold(0.0f32, |max_val, state| (state.pop.as_ref().unwrap().hisp_pop.unwrap() as f32).max(max_val));
                            for state in states.iter() {
                                let brightness = ((state.pop.as_ref().unwrap().hisp_pop.unwrap() as f32/max) * 5.0) as u8;
                                assert!(brightness <= 5);
                                data[get_state_id(&state.name)] = to_char(brightness);
                            }
                            data[50] = '\n' as u8;
                            arduino.write(&data).expect("Write failed!");
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation)).with_unique_header("Access-Control-Allow-Origin", "*")
                        },
                        "hispanic_per_capita" => {
                            let max = states.iter().fold(0.0f32, 
                                                         |max_val, state|
                                                         (state.pop.as_ref().unwrap().hisp_pop.unwrap() as f32 / state.pop.as_ref().unwrap().total.unwrap() as f32).max(max_val));
                            for state in states.iter() {
                                let brightness = (((state.pop.as_ref().unwrap().hisp_pop.unwrap() as f32 / state.pop.as_ref().unwrap().total.unwrap() as f32) / max) * 5.0) as u8;
                                assert!(brightness <= 5);
                                data[get_state_id(&state.name)] = to_char(brightness);
                            }
                            data[50] = '\n' as u8;
                            arduino.write(&data).expect("Write failed!");
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation)).with_unique_header("Access-Control-Allow-Origin", "*")
                        },
                        _ => Response::empty_404()
                    }
                },
                "econ" => {
                    match animation.as_str() {
                        "median_household_income" => {
                            let max = states.iter().fold(0.0f32, |max_val, state| (state.econ.as_ref().unwrap().median_household_income.unwrap() as f32).max(max_val));
                            for state in states.iter() {
                                let brightness = ((state.econ.as_ref().unwrap().median_household_income.unwrap() as f32/max) * 5.0) as u8;
                                assert!(brightness <= 5);
                                data[get_state_id(&state.name)] = to_char(brightness);
                            }
                            data[50] = '\n' as u8;
                            arduino.write(&data).expect("Write failed!");
                            Response::text(format!("Category: Economic, Animation: {} => arduino", animation)).with_unique_header("Access-Control-Allow-Origin", "*")
                        },
                        "income_hist" => {
                            for year in vec!["2009", "2010", "2011", "2012", "2013", "2014", "2015", "2016", "2017", "2018", "2019", "2020", "2021"] {
                                let s = get_states_acs(&api_key, year).unwrap();
                                let max = s.iter().fold(0.0f32, |max_val, state| (state.econ.as_ref().unwrap().median_household_income.unwrap() as f32).max(max_val));
                                for state in s.iter() {
                                    let brightness = ((state.econ.as_ref().unwrap().median_household_income.unwrap() as f32/max) * 5.0) as u8;
                                    assert!(brightness <= 5);
                                    data[get_state_id(&state.name)] = to_char(brightness);
                                }
                                data[50] = '\n' as u8;
                                arduino.write(&data).expect("Write failed!");
                                thread::sleep(time::Duration::from_millis(500));
                            }
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation)).with_unique_header("Access-Control-Allow-Origin", "*")
                        },
                        "median_gross_rent" => {
                            let max = states.iter().fold(0.0f32, |max_val, state| (state.econ.as_ref().unwrap().median_gross_rent.unwrap() as f32).max(max_val));
                            for state in states.iter() {
                                let brightness = ((state.econ.as_ref().unwrap().median_gross_rent.unwrap() as f32/max) * 5.0) as u8;
                                assert!(brightness <= 5);
                                data[get_state_id(&state.name)] = to_char(brightness);
                            }
                            data[50] = '\n' as u8;
                            arduino.write(&data).expect("Write failed!");
                            Response::text(format!("Category: Economic, Animation: {} => arduino", animation)).with_unique_header("Access-Control-Allow-Origin", "*")
                        },
                        "median_home_value" => {
                            let max = states.iter().fold(0.0f32, |max_val, state| (state.econ.as_ref().unwrap().median_home_value.unwrap() as f32).max(max_val));
                            for state in states.iter() {
                                let brightness = ((state.econ.as_ref().unwrap().median_home_value.unwrap() as f32/max) * 5.0) as u8;
                                assert!(brightness <= 5);
                                data[get_state_id(&state.name)] = to_char(brightness);
                            }
                            data[50] = '\n' as u8;
                            arduino.write(&data).expect("Write failed!");
                            Response::text(format!("Category: Economic, Animation: {} => arduino", animation)).with_unique_header("Access-Control-Allow-Origin", "*")
                        },
                        "poverty" => {
                            let max = states.iter().fold(0.0f32, |max_val, state| (state.econ.as_ref().unwrap().percentage_poor.unwrap() as f32).max(max_val));
                            for state in states.iter() {
                                let brightness = ((state.econ.as_ref().unwrap().percentage_poor.unwrap() as f32/max) * 5.0) as u8;
                                assert!(brightness <= 5);
                                data[get_state_id(&state.name)] = to_char(brightness);
                            }
                            data[50] = '\n' as u8;
                            arduino.write(&data).expect("Write failed!");
                            Response::text(format!("Category: Economic, Animation: {} => arduino", animation)).with_unique_header("Access-Control-Allow-Origin", "*")
                        },
                        "joined_union" => {
                            let join_order: Vec<&str> = vec![
                                "Delaware",
                                "Pennsylvania",
                                "New Jersey",
                                "Georgia",
                                "Connecticut",
                                "Massachusetts",
                                "Maryland",
                                "South Carolina",
                                "New Hampshire",
                                "Virginia",
                                "New York",
                                "North Carolina",
                                "Rhode Island",
                                "Vermont",
                                "Kentucky",
                                "Tennessee",
                                "Ohio",
                                "Louisiana",
                                "Indiana",
                                "Mississippi",
                                "Illinois",
                                "Alabama",
                                "Maine",
                                "Missouri",
                                "Arkansas",
                                "Michigan",
                                "Florida",
                                "Texas",
                                "Iowa",
                                "Wisconsin",
                                "California",
                                "Minnesota",
                                "Oregon",
                                "Kansas",
                                "West Virginia",
                                "Nevada",
                                "Nebraska",
                                "Colorado",
                                "North Dakota",
                                "South Dakota",
                                "Montana",
                                "Washington",
                                "Idaho",
                                "Wyoming",
                                "Utah",
                                "Oklahoma",
                                "New Mexico",
                                "Arizona",
                                "Alaska",
                                "Hawaii"
                            ];

                            for state in join_order.iter() {
                                let brightness: u8 = 4;
                                assert!(brightness <= 5);
                                data[get_state_id(state)] = to_char(brightness);
                                data[50] = '\n' as u8;

                                arduino.write(&data).expect("Write failed!");
                                thread::sleep(time::Duration::from_millis(500));
                            }
                            Response::text(format!("Category: Economic, Animation: {} => arduino", animation)).with_unique_header("Access-Control-Allow-Origin", "*")
                        },
                        _ => Response::empty_404()
                    }
                },
                _ => Response::empty_404()
            }
        },
        _ => Response::empty_404()
    )
}

pub fn serve(host: &str, dev: String) {
    rouille::start_server(host, move |request| {
        let response = handler(&request, &dev);
        response
    });
}
