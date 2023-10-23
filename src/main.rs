use std::{
    net::*,
    thread,
    sync::mpsc::channel,
};
use rppal::gpio::Gpio;

mod math_structs;

const MOTOR_LEFT: u8 = 15; //TODO make accurate
const MOTOR_MIDDLE: u8 = 15; //TODO make accurate
const MOTOR_RIGHT: u8 = 15; //TODO make accurate

const MOTOR_SYRINGE: u8 = 15; //TODO make accurate
const POTENTIOMETER: [u8; 4] = 
    [15, 15, 15, 15]; // TODO make accurate

struct ControlInfo {

}

struct CommunicationInfo {

}

fn main() {
    let mut stream: TcpStream;

    let (tx_control, rx_control) = channel();
    let (tx_communication, rx_communication) = channel();
    //communication with computer
    thread::spawn(move || {
        let info = rx_communication.try_recv();
        //TODO          
        tx_control.send(ControlInfo {
        
        }).ok();
    });

    //device control
    thread::spawn(move || {
        let info = rx_control.try_recv();
        //TODO
        tx_communication.send( CommunicationInfo {
            
        }).ok();
    });
}
