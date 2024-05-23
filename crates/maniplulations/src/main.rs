use opencv::imgcodecs;

#[tokio::main]
async fn main() {
    cvtool::download_and_unzip("https://www.dropbox.com/s/rys6f1vprily2bg/opencv_bootcamp_assets_NB2.zip?dl=1", "assets.ig").await.unwrap_or_default();
    let img = imgcodecs::imread("assets.ig/opencv_bootcamp_assets_NB2/New_Zealand_Boat.jpg", imgcodecs::IMREAD_COLOR).unwrap();

    cvtool::show_mat(&img, "原图", 1);
    // 这一节的内容台简单了，不做了。后面先看一遍教程，太监了的就先不写代码了。
}
