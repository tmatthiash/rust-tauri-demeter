#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{ Manager };

#[derive(Clone, serde::Serialize)]
struct Payload {
    data: Vec<u8>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // listen();
    println!("UP");
    tauri::Builder::default()
        .setup(|app| {
            listen(app.get_window("main").unwrap());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        // .invoke_handler(tauri::generate_handler![send_udp])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// #[tauri::command]
// fn send_udp(buffer: Vec<u8>) {
//     println!("trying to emitTTTTT");
//     tauri::Builder::default()
//         .setup(|app| {
//             println!("trying to emit");
//             app.emit_all("got-udp", Payload{data: buffer}).unwrap();
//             Ok(())
//     });
// }

fn listen(window: tauri::Window) {
    let socket = std::net::UdpSocket::bind("0.0.0.0:60084").expect("couldn't bind to address");

    // use std::sync::{Arc, Mutex};
    // let addr: Arc<Mutex<Option<std::net::SocketAddr>>> = Arc::from(Mutex::new(None));

    // let addr_clone = addr.clone();
    let socket_clone = socket.try_clone().unwrap();

    std::thread::spawn(move || loop {
        let mut buffer = [0;1442];
        let (amt, src_addr) = socket_clone.recv_from(&mut buffer).unwrap();
        // *addr_clone.lock().unwrap() = Some(src_addr);
        println!("amt: {}", amt);
        println!("src: {}", src_addr);                
        let first_slice = &buffer[2..182];
        // let leading_zeros = format!("{:0>8b}", first_slice);
        // println!("{:?}", first_slice); 
           
        // send_udp(first_slice.to_vec());
        window.emit("got-udp", Payload{data: first_slice.to_vec()});
        // print!("{}", std::str::from_utf8(&buffer).unwrap());
    });




    // use std::thread;
    // use std::net::UdpSocket;

    // let socket = match UdpSocket::bind("0.0.0.0:60084") {
    //     Ok(s) => s,
    //     Err(e) => panic!("couldn't bind socket: {}", e)
    // };

    // let mut buf = [0; 1442];
    // loop {
    //     match socket.recv_from(&mut buf) {
    //         Ok((amt, src)) => {
    //             thread::spawn(move || {
    //                 println!("amt: {}", amt);
    //                 println!("src: {}", src);
    //                 // println!("{:?}", buf);
    //                 let first_slice = &buf[2..182];
    //                 for (index, binary) in first_slice.iter().enumerate() {
    //                     if index % 4 == 0 {
    //                         print!("Row {:0>2} : ", index / 4);
    //                     }
    //                     let leading_zeros = format!("{:0>8b}", binary);
    //                     print!("{}", leading_zeros);
    //                     if index % 4 == 3 {
    //                         println!("");
    //                     }
    //                 }
    //             });
    //         },
    //         Err(e) => {
    //             println!("couldn't recieve a datagram: {}", e);
    //         }
    //     }
    // }
}