use opencv::{highgui, imgcodecs};

fn main() {
    let img = imgcodecs::imread("assets/01.png", imgcodecs::IMREAD_ANYCOLOR).unwrap();
    highgui::imshow("Test", &img).unwrap();
    highgui::wait_key(0).unwrap();
}
