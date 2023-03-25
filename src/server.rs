use rouille::*;
use serde::Serialize;

#[derive(Serialize)]
struct Routes {
    routes: String,
    pop: Vec<String>,
    econ: Vec<String>
}

fn handler(request: &Request) -> Response {
    router!(request,
        (GET) (/) => {
            //default
            Response::text("Driver API for geo-juicers, query /routes for all routes")
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
                    "median_home_value".to_string(),
                    "poverty".to_string()
                ]
            })
        },
        (GET) (/{category: String}/{animation: String}) => {
            // This route returns the list of notes. We perform the query and output it as JSON.
            Response::text(format!("Category: {}, Animation: {} => arduino", category, animation))
            //let arduino = serialport::new("/dev/ttyUSB0", 9600).open().expect("Failed to open port")
        },
        (PUT) (/{category: String}/{animation: String}) => {
            // This route returns the list of notes. We perform the query and output it as JSON.
            Response::text(format!("Category: {}, Animation: {} => arduino", category, animation))
            //let arduino = serialport::new("/dev/ttyUSB0", 9600).open().expect("Failed to open port")
        },
        _ => Response::empty_404()
    )
}

pub fn serve(host: &str) {
    rouille::start_server(host, move |request| {
        let response = handler(&request);
        response
    });
}
