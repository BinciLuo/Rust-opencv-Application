use opencv::{
    prelude::*,
    videoio,
    highgui,
};

use opencv::core::Mat;
use opencv::core::Vector;
use opencv::imgcodecs::{self, ImwriteFlags};
use chrono::Local;

pub struct Camera{
    cam:videoio::VideoCapture,
}

impl Camera{
    pub fn new()->Self{
        Self { cam: videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap()}

    }

    pub fn camera(&mut self) -> Result<(),opencv::Error> {
        highgui::named_window("Rust Web Camera Tips:Press[(p, Take picture), (s, Save), (q, quit)]", highgui::WINDOW_FULLSCREEN)?;
        let mut frame = Mat::default();
        let mut saving=false;
        loop {
            if !saving{
                self.cam.read(&mut frame)?;
            }
            highgui::imshow("Rust Web Camera Tips:Press[(p, Take picture), (s, Save), (q, quit)]", &frame)?;

            let mut key=highgui::wait_key(1)?;
            if saving{
                key = highgui::wait_key(500000)?;
            }

            if saving && key == 115{//s
                save_mat_as_image(&frame,"pics/Camera");
                saving=false;
                continue;
            }
            saving=false;
            if key == 113 {//q
                break;
            }else if key == 112 {//p
                //save_mat_as_image(&frame,"pics/Camera");
                saving=true;
                continue;
            }
        }
        highgui::destroy_all_windows();
        Ok(())
    }

    pub fn capture_frame(&mut self)->Result<(),opencv::Error>{
        highgui::named_window("RustCapture Tips:Press[(s, Save), (q, quit)]", highgui::WINDOW_FULLSCREEN)?;
        let mut frame = Mat::default();
        let mut saved=false;
        loop{
            if !saved{
                self.cam.read(&mut frame)?;
            }
            saved=false;
            
            highgui::imshow("Capture    Press [(q, Quit), (s, Save), (other keys, Continue)]", &frame)?;
            let key = highgui::wait_key(50000)?;
            if key == 113 {//q
                return Ok(());
            }else if key == 114 {//r
                continue;
            }else if key == 115 {//s
                save_mat_as_image(&frame,"pics/Capture");
                saved=true;
                continue;
            }
        }
    }
}

pub fn get_frame() -> Result<Mat,opencv::Error> {
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY)?;
    let mut frame = Mat::default();
    cam.read(&mut frame)?;

    Ok(frame)
}

pub fn show_frame(frame:&Mat)->Result<(),opencv::Error>{
    highgui::named_window("window", highgui::WINDOW_FULLSCREEN)?;
    highgui::imshow("window", frame)?;
    let key = highgui::wait_key(50000)?;
    if key == 113 {//q
        return Ok(());
    }
    Ok(())
}

pub fn save_mat_as_image(mat: &Mat, file_path: &str) {
    let current_time = Local::now();
    let time_string = current_time.format("%Y-%m-%d[%H:%M:%S]").to_string();
    let mut params=Vector::<i32>::new();
    params.push(ImwriteFlags::IMWRITE_JPEG_QUALITY as i32);
    params.push(100);
    imgcodecs::imwrite(
        format!("{}/{}.jpeg",file_path,time_string).as_str(),
        mat,
        &params
    )
    .expect("Failed to save image");
}
