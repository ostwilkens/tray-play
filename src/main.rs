use rodio::Source;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use trayicon::{Icon, MenuBuilder, TrayIconBuilder};
use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
};

#[derive(Clone, Eq, PartialEq, Debug)]
enum Events {
    Exit,
    TogglePause,
}

fn main() {
    let event_loop = EventLoop::<Events>::with_user_event();
    let proxy = event_loop.create_proxy();

    let icon_bytes = include_bytes!("../icon.ico");

    let icon = Icon::from_buffer(icon_bytes, None, None).unwrap();
    let icon_paused = Icon::from_buffer(include_bytes!("../icon-paused.ico"), None, None).unwrap();

    let mut tray_icon = TrayIconBuilder::new()
        .sender_winit(proxy)
        .icon_from_buffer(icon_bytes)
        .tooltip("Tray Play")
        .on_click(Events::TogglePause)
        .menu(
            MenuBuilder::new()
                .checkable("Pause", false, Events::TogglePause)
                .item("E&xit", Events::Exit),
        )
        .build()
        .unwrap();

    let file_path = Path::new(&dirs::home_dir().unwrap()).join("tray-play.ogg");
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let file = File::open(file_path).unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source.repeat_infinite());

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        let _ = tray_icon;

        match event {
            Event::UserEvent(e) => match e {
                Events::Exit => *control_flow = ControlFlow::Exit,
                Events::TogglePause => {
                    if let Some(was_paused) = tray_icon.get_menu_item_checkable(Events::TogglePause)
                    {
                        if was_paused {
                            sink.play();
                            tray_icon.set_icon(&icon).unwrap();
                            tray_icon
                                .set_menu_item_checkable(Events::TogglePause, false)
                                .unwrap();
                        } else {
                            sink.pause();
                            tray_icon.set_icon(&icon_paused).unwrap();
                            tray_icon
                                .set_menu_item_checkable(Events::TogglePause, true)
                                .unwrap();
                        }
                    }
                }
            },
            _ => (),
        }
    });
}
