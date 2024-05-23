use opencv::{core::{Mat, MatTraitConst, Size, Vector}, highgui, imgcodecs, imgproc};

#[tokio::main]
async fn main() {
    download().await;
    let cb_img = imgcodecs::imread("./assets.ig/opencv_bootcamp_assets_NB1/checkerboard_color.png", imgcodecs::IMREAD_COLOR).unwrap();
    cvtool::show_mat(&cb_img, "通道-原图" , 1);
    
    let gray_img = imgcodecs::imread("./assets.ig/opencv_bootcamp_assets_NB1/checkerboard_color.png", imgcodecs::IMREAD_GRAYSCALE).unwrap();
    cvtool::show_mat(&gray_img, "通道-单通道灰色", 1);

    
    let color_img = imgcodecs::imread("./assets.ig/opencv_bootcamp_assets_NB1/coca-cola-logo.png", imgcodecs::IMREAD_COLOR).unwrap();
    // 我在ubuntu22下,opencv的显示效果和操作系统显示的一样.
    cvtool::show_mat(&color_img, "颜色-原图", 1);


    let img_NZ_bgr = imgcodecs::imread("./assets.ig/opencv_bootcamp_assets_NB1/New_Zealand_Lake.jpg", imgcodecs::IMREAD_COLOR).unwrap();
    cvtool::show_mat(&img_NZ_bgr, "图像处理-原图", 1);

    let mut channels = Vector::new();
    opencv::core::split(&img_NZ_bgr, &mut channels);

    cvtool::show_mat(&channels.get(0).unwrap(), "RGB单通道-蓝通道", 1);
    cvtool::show_mat(&channels.get(1).unwrap(), "RGB单通道-绿通道", 1);
    cvtool::show_mat(&channels.get(2).unwrap(), "RGB单通道-红通道", 1);


    let img_hsv = imgproc::cvt_color(&img_NZ_bgr, &mut channels, imgproc::COLOR_BGR2HSV, 0);

    cvtool::show_mat(&channels.get(0).unwrap(), "HSV单通道-色彩通道", 1);
    cvtool::show_mat(&channels.get(1).unwrap(), "HSV单通道-深浅通道", 1);
    cvtool::show_mat(&channels.get(2).unwrap(), "HSV单通道-明亮通道", 1);


}

async fn download() {
    let url = "https://www.dropbox.com/s/qhhlqcica1nvtaw/opencv_bootcamp_assets_NB1.zip?dl=1";
    let save_path = "./assets.ig";
    match cvtool::download_and_unzip(url, save_path).await {
        Ok(_) => println!("File download and unzipped successfully."),
        Err(e) => eprintln!("Failed to download and unzip file:{}", e),
    };
}