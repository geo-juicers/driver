mod api;
mod server;

fn main() {
    let _ports = serialport::available_ports().expect("No ports found!");
    //println!("{:#?}", ports);
    println!("Serving API @ localhost:8000");
    server::serve("localhost:8000", "/dev/tty.usbmodem14301".to_string());
}
