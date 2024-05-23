use opencv::{core::{Mat, MatTraitConst, MatTraitConstManual, Point2i}, highgui, imgproc, types::{VectorOfPoint, VectorOfPoint2f}, videoio::{VideoCapture, VideoCaptureTrait, VideoCaptureTraitConst, CAP_ANY}};


const PREVIEW: i32 = 0;
const BLUR: i32 = 1;
const FEATURES: i32 = 2;
const CANNY: i32 = 3;
fn main() {
    let args:Vec<String> = std::env::args().collect();
    let file = if args.len() > 1 {
        &args[1]
    } else {
        "0"
    };

    let mut cap = match file {
        "0" => {
            print!("打开摄像头");
            VideoCapture::new(0, CAP_ANY).unwrap()
        },
        _ => VideoCapture::from_file(file, CAP_ANY).unwrap()
    };
    if !cap.is_opened().unwrap() {
        eprintln!("Cannot open video source");
        return;
    }

    let mut image_filter = PREVIEW;
    let mut alive = true;
    let mut edges = Mat::default();


    while alive {
        let mut frame = Mat::default();
        cap.read(&mut frame).unwrap();

        // 进行水平翻转
        opencv::core::flip(&frame.clone(), &mut frame, 1).unwrap();

        let result = match image_filter {
            PREVIEW => frame,
            CANNY => {
                imgproc::canny(&frame,&mut edges, 80.0, 150.0, 3, false).unwrap();
                edges.clone()
            },
            BLUR => {
                imgproc::blur(&frame, &mut edges, opencv::core::Size::new(13, 13), opencv::core::Point::new(-1, -1), opencv::core::BORDER_DEFAULT).unwrap();
                edges.clone()
            },
            FEATURES => {
                let mut gray = Mat::default();
                imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();
                let mut corners = VectorOfPoint2f::new();
                imgproc::good_features_to_track(&gray, &mut corners, 500, 0.2, 15.0, &opencv::core::no_array(), 9, false, 0.04).unwrap();

                for corner in corners.iter() {
                    println!("检测到了边角");
                    let corner_int = Point2i::new(corner.x as i32, corner.y as i32);
                    imgproc::circle(&mut frame, corner_int, 10, opencv::core::Scalar::new(0.0, 255.0, 0.0, 0.0), 1, 8, 0).unwrap();
                }
                frame
            },
            _ => {
                eprintln!("default");
                frame
            },
        };

        highgui::imshow("边缘检测", &result).unwrap();
        let key = highgui::wait_key(1).unwrap();
        match key {
            81 | 113 | 27 => alive = false, // Q or q or ESC
            67 | 99 => image_filter = CANNY, // C or c
            66 | 98 => image_filter = BLUR,  // B or b
            70 | 102 => image_filter = FEATURES, // F or f
            80 | 112 => image_filter = PREVIEW,  // P or p
            _ => {}
        }
    }


    cap.release().unwrap();
    highgui::destroy_all_windows().unwrap();
}
