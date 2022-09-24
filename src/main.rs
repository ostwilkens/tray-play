#![windows_subsystem = "windows"]

use rodio::source::Source;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use {std::sync::mpsc, tray_item::TrayItem};

enum Message {
    Quit,
    TogglePause,
}

fn main() {
    let mut tray = TrayItem::new("Tray Play", "tray-icon").unwrap();
    let mut playing = true;
    let (tx, rx) = mpsc::sync_channel(1);

    let tx_pause = tx.clone();
    tray.add_menu_item("Toggle Pause", move || {
        tx_pause.send(Message::TogglePause).unwrap();
    })
    .unwrap();

    let tx_quit = tx.clone();
    tray.add_menu_item("Quit", move || {
        tx_quit.send(Message::Quit).unwrap();
    })
    .unwrap();

    let file_path = Path::new(&dirs::home_dir().unwrap()).join("tray-play.ogg");

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let file = File::open(file_path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source.repeat_infinite());

    loop {
        match rx.recv() {
            Ok(Message::Quit) => break,
            Ok(Message::TogglePause) => {
                if playing {
                    playing = false;
                    sink.pause();
                    tray.set_icon("tray-icon-paused").unwrap();
                } else {
                    playing = true;
                    sink.play();
                    tray.set_icon("tray-icon").unwrap();
                }
            }
            _ => {
                sink.stop();
                sink.sleep_until_end();
            }
        }
    }
}
