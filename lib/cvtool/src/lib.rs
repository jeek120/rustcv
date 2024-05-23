use reqwest::{header::{HeaderMap, HeaderValue}, Client, Proxy};
use tokio::{fs::File, io::AsyncWriteExt, task};
use zip::ZipArchive;
use std::{env, error::Error, fs, io::Read, path::Path, time::Duration};
use opencv::{core::{self, Mat, MatTraitConst, Size}, highgui, imgproc};

pub async fn download_and_unzip(url:&str, save_path: &str) -> Result<(), Box<dyn Error>> {
    // 确保保存路径存在
    let save_path = Path::new(save_path);
    let file_name = url.rsplit("/").next().ok_or("Invalid URL")?.split("?").next().ok_or("Invalid URL?")?;
    let path = Path::new(save_path).join(file_name);
    if path.exists() {
        println!("{:?} file exist!", path);
        return Ok(());
    } else {
        println!("download {:?}", path);
    }
    if !save_path.exists() {
        fs::create_dir_all(save_path)?;
    }

    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/125.0.0.0 Safari/537.36"));

    let mut client = Client::builder()
        .timeout(Duration::from_secs(10))
        .default_headers(headers)
        .redirect(reqwest::redirect::Policy::default());

    if let Some(proxy) = get_system_proxy() {
        println!("Using proxy: {:?}", proxy);
        client = client.proxy(proxy);
    }
    let client = client.build()?;

    // 下载文件
    let resp = client.get(url).send().await.map_err(|e| {
        println!("Failed to send request: {:?}", e);
        e
    }).unwrap();
    if !resp.status().is_success() {
        return Err("Failed to download file".into());
    }
    println!("download success: status:{}", resp.status());

    let content = resp.bytes().await?;

    // 保存文件
    let mut file = File::create(path).await?;
    file.write_all(&content).await?;

    // 解压准备工作
    let buffer = content.clone();
    let mut archive = task::spawn_blocking(move || ZipArchive::new(std::io::Cursor::new(buffer))).await??;
    let output_dir = Path::new(save_path);
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }

    // 创建解压路径
    let unzip_folder = save_path.join(file_name.strip_suffix(".zip").unwrap_or(file_name));
    if !unzip_folder.exists() {
        fs::create_dir_all(&unzip_folder)?;
    }

    // 解压
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let out_path = match file.enclosed_name() {
            Some(path) => unzip_folder.join(path),
            None => continue
        };

        if file.is_dir() {
            fs::create_dir_all(&out_path)?;
        } else {
            if let Some(parent) = out_path.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)?;
                }
            }

            let mut outfile = File::create(&out_path).await?;
            let mut file_content = Vec::new();
            file.read_to_end(&mut file_content)?;
            outfile.write_all(&file_content).await?;
        }

        // 设置文件权限
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&out_path, fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}

fn get_system_proxy() -> Option<Proxy> {
    env::set_var("HTTPS_PROXY", "127.0.0.1:7890");
    if let Ok(http_proxy) = env::var("HTTP_PROXY") {
        return Some(Proxy::http(&http_proxy).unwrap());
    }

    if let Ok(https_proxy) = env::var("HTTPS_PROXY") {
        return Some(Proxy::https(&https_proxy).unwrap());
    }

    None
}

pub fn print_mat(mat: &core::Mat) -> Result<(), Box<dyn Error>> {
    let rows = mat.rows();
    let cols = mat.cols();
    let mat_type = mat.typ();
    let channels = mat.channels();

    println!("Matrix size: {}x{}, Channels: {}, Type: {:?}", rows, cols, channels, mat_type);

    match mat_type {
        core::CV_8UC1 => {
            for i in 0..rows {
                for j in 0..cols {
                    let pixel = *mat.at_2d::<u8>(i, j)?;
                    print!("{:3} ", pixel);
                }
                println!();
            }
        },
        core::CV_8UC3 => {
            for i in 0..rows {
                for j in 0..cols {
                    let pixel = mat.at_2d::<core::Vec3b>(i, j)?;
                    print!("[{:3}, {:3}, {:3}] ", pixel[0], pixel[1], pixel[2]);
                }
                println!();
            }
        },
        _ => {
            println!("Unsupported mat type or channels.");
        }
    }

    Ok(())
}

pub fn show_mat(img:&Mat,name:&str,show_scale: i32) {
    println!("目前展示{}", name);
    print_mat(&img).unwrap();
    let img_width = img.cols();
    let img_height = img.rows();
    let (new_img, new_width, new_height) = if show_scale > 1 {
        let new_width = img_width * show_scale;
        let new_height = img_height * show_scale;
        let mut resized_img = Mat::default();
        imgproc::resize(&img, &mut resized_img, Size::new(new_width, new_height), 0.0, 0.0, imgproc::INTER_NEAREST).unwrap();
        (resized_img, new_width, new_height)
    } else {
        (img.clone(), img_width, img_height)
    };

    highgui::imshow(&name, &new_img).unwrap();
    
    let k = highgui::wait_key(8000).unwrap();
    highgui::destroy_all_windows().unwrap();
}
