use rouille::*;
use serde::Serialize;
use crate::api::{get_states_acs, get_state_id, State};
use std::{thread, time};

#[derive(Serialize)]
struct Routes {
    routes: String,
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
                .with_unique_header("Access-Control-Allow-Method", "*")
                .with_unique_header("Access-Control-Allow-Headers", "Content-Type")
        },

        (GET) (/routes) => {
            // This route returns the list of notes. We perform the query and output it as JSON.
            Response::json(&Routes{
                routes: "show all routes".to_string(),
                pop: vec![
                    "total".to_string(),
                    "total_hist".to_string(),
                    "median_age".to_string(),
                    "hispanic".to_string()
                ],
                econ: vec![
                    "median_household_income".to_string(),
                    "median_gross_rent".to_string(),
                    "median_home_value".to_string(),
                    "poverty".to_string()
                ]
            })
        },
        (GET) (/all) => {
            let mut arduino = serialport::new(dev, 9600).open().expect("Failed to open port");
            thread::sleep(time::Duration::from_secs(1));
            let mut data: [u8; 51] = [0; 51];
            for n in 0..50 {
                data[n] = 53
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
        (GET) (/{category: String}/{animation: String}) => {
            let api_key:String = dotenv::var("API_KEY").unwrap();
            let mut arduino = serialport::new(dev, 9600).open().expect("Failed to open port");
            thread::sleep(time::Duration::from_secs(1));
            let mut states: Vec::<State> = get_states_acs(&api_key, "2021").unwrap();
            //remove DC and Puerto Rico
            states.retain(|state| state.name != "District of Columbia" && state.name != "Puerto Rico");
            let mut data: [u8; 51] = [0; 51];
            match category.as_str() {
                "pop" => {
                    match animation.as_str() {
                        "total" => {
                            println!("getting total!");
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
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation))
                        },
                        "total_hist" => {
                            let max = states.iter().fold(0.0f32, |max_val, state| (state.pop.as_ref().unwrap().total.unwrap() as f32).max(max_val));
                            for state in states.iter() {
                                let brightness = ((state.pop.as_ref().unwrap().total.unwrap() as f32/max) * 5.0) as u8;
                                assert!(brightness <= 5);
                                data[get_state_id(&state.name)] = to_char(brightness);
                            }
                            data[50] = '\n' as u8;
                            arduino.write(&data).expect("Write failed!");
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation))
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
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation))
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
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation))
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
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation))
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
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation))
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
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation))
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
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation))
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
                            Response::text(format!("Category: Population, Animation: {} => arduino", animation))
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
