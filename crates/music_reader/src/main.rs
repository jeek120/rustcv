use opencv::{core::Mat, imgcodecs, imgproc};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    cvtool::download_and_unzip("https://www.dropbox.com/s/0oe92zziik5mwhf/opencv_bootcamp_assets_NB4.zip?dl=1", "/tmp/assets").await.unwrap();

    let img = imgcodecs::imread("/tmp/assets/opencv_bootcamp_assets_NB4/Piano_Sheet_Music.png", imgcodecs::IMREAD_COLOR).unwrap();
    cvtool::show_mat(&img, "原图", 1);

    let mut gray_img = Mat::default();
    imgproc::cvt_color(&img, &mut gray_img, imgproc::COLOR_BGR2GRAY, 0).unwrap();


    cvtool::show_mat(&gray_img, "灰度图", 1);

    let mut threshold = Mat::default();
    imgproc::threshold(&gray_img, &mut threshold, 50.0, 255.0, imgproc::THRESH_BINARY).unwrap();
    cvtool::show_mat(&threshold, "50阀值", 1);


    imgproc::threshold(&gray_img, &mut threshold, 130.0, 255.0, imgproc::THRESH_BINARY).unwrap();
    cvtool::show_mat(&threshold, "130阀值", 1);

    imgproc::adaptive_threshold(&gray_img, &mut threshold, 255.0, imgproc::ADAPTIVE_THRESH_GAUSSIAN_C, imgproc::THRESH_BINARY, 11, 7.0).unwrap();
    cvtool::show_mat(&threshold, "自适应阀值11-7", 1);
}

