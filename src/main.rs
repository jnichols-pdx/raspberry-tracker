mod common;
mod ui;

//use std::env;
use crate::common::*;
use std::io::{self, stderr, Write, Error};
use tokio::sync::{mpsc};
//use tokio::time::{self, Duration};



//EGUI offers both native and web assembly compilation targets, I don't intend to use WASM.
#[cfg(not(target_arch = "wasm32"))] 
#[tokio::main]
async fn main() {//-> Result<(),io::Error> {

    let (tx_to_ui, rx_from_main) = mpsc::channel::<Action>(32);
    //let (tx_to_main, rx_from_websocket) = mpsc::channel(32);   

    let character_list = CharacterList::new();

    tokio::spawn(async move { //to host websocket connection
    });

    let app = ui::TrackerApp{
        from_main: rx_from_main,
        in_character_ui: true,
        char_list: character_list,
    };

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(app), native_options);
}
