use windres::Build;

fn main() {
    Build::new().compile("tray-play.rc").unwrap();
}
