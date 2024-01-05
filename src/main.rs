use std::{
    net::*,
    thread,
    sync::mpsc::channel,
};
use rppal::gpio::Gpio;
use physics::*;

mod physics;

const MOTOR_LEFT: u8 = 15; //TODO make accurate
const MOTOR_MIDDLE: u8 = 15; //TODO make accurate
const MOTOR_RIGHT: u8 = 15; //TODO make accurate

const MOTOR_SYRINGE: u8 = 15; //TODO make accurate
const POTENTIOMETER: [u8; 4] = 
    [15, 15, 15, 15]; // TODO make accurate

#[derive(Debug)]
struct ControlInfo {
    target_velocity: Vector3,
    target_rotation: f32,
    target_syringe: f32,
    disconnect: bool,
}

#[derive(Debug)]
struct CommunicationInfo {
    current_velocity: Vector3,
    current_rotation: f32,
    current_syringe: f32,
}

const CONTROL_PORT: u16 = 8080;
const SITE_PORT: u16 = 80;

fn main() -> std::io::Result<()> {
    open_website_server()?;

    Ok(())
}

fn open_website_server() -> std::io::Result<()> {
    let mut listener = TcpListener::bind(format!("127.0.0.1:{}", SITE_PORT))?;
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_website_client(stream);
                open_control_server()?;
            }
            Err(e) => { println!("Error: {}", e); }
        }
    }

    Ok(())
}

fn handle_website_client(mut stream: TcpStream) {
    
}

fn open_control_server() -> std::io::Result<()> {
    let mut listener = TcpListener::bind(format!("127.0.0.1:{}", CONTROL_PORT))?;
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_control_client(stream);
            }
            Err(e) => { println!("Error: {}", e); }
        }
    }
    Ok(())
}
fn handle_control_client(mut stream: TcpStream) {
    let (tx_dc, rx_dc) = channel();
    let (tx_control, rx_control) = channel();
    let (tx_communication, rx_communication) = channel();
    //communication with computer
    thread::spawn(move || {
        loop {
            //not try_recv so that the threads are synchronized
            let info = rx_communication.recv();

            stream.write();

            let target_velocity = Vector3::new(0.0, 0.0, 0.0); //TODO fix all
            let target_rotation = 0.0;
            let target_syringe = 0.0;

            let disconnect = false;

            tx_control.send(ControlInfo {
                target_velocity,
                target_rotation,
                target_syringe,
                disconnect,
            }).ok();
            if disconnect { 
                tx_dc.send(()).ok();
                break; 
            }
        }
    });

    //device control
    thread::spawn(move || {
        let mut gpio = Gpio::new().unwrap();

        let mut motor_left = gpio.get(MOTOR_LEFT).unwrap().into_output();
        let mut motor_middle = gpio.get(MOTOR_MIDDLE).unwrap().into_output();
        let mut motor_right = gpio.get(MOTOR_RIGHT).unwrap().into_output();

        let mut motor_syringe = gpio.get(MOTOR_SYRINGE).unwrap().into_output();


        loop {
            let info = rx_control.recv().ok().unwrap();

            if info.disconnect { 
                //TODO stop everything
                break; 
            }
            
            let current_velocity = Vector3::new(0.0, 0.0, 0.0); //TODO fix all
            let current_rotation = 0.0;
            let current_syringe = 0.0;

            tx_communication.send( CommunicationInfo {
                current_velocity,
                current_rotation,
                current_syringe,
            }).ok();
        }
    });
    rx_dc.recv().ok();
}
