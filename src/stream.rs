mod detect;
mod tools;

use tools::save_mat_as_image;
use opencv::{
    prelude::*,
    videoio,
    highgui,
    core::Mat,
};

pub const VEDIO: i32 = 0;
pub const CAMERA: i32 = 1;


pub struct Stream{
    stream_frames: videoio::VideoCapture,
    stream_source: i32,
}

impl Stream{
    pub fn from_video(path:&str) -> Self{
        Self{
                stream_frames:videoio::VideoCapture::from_file(path,videoio::CAP_FFMPEG).unwrap(),
                stream_source: VEDIO,
            }
    }

    pub fn from_camera() -> Self{
        Self {
                stream_frames: videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap(),
                stream_source: CAMERA,
            }
    }

    pub fn camera(&mut self) -> Result<(),opencv::Error> {
        highgui::named_window("Rust Web Camera Tips:Press[(p, Take picture), (s, Save), (q, quit)]", highgui::WINDOW_FULLSCREEN)?;
        let mut frame = Mat::default();
        let mut saving=false;
        loop {
            if !saving{
                self.stream_frames.read(&mut frame)?;
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
        highgui::destroy_all_windows()?;
        Ok(())
    }

    pub fn capture_frame(&mut self)->Result<(),opencv::Error>{
        highgui::named_window("RustCapture Tips:Press[(s, Save), (q, quit)]", highgui::WINDOW_FULLSCREEN)?;
        let mut frame = Mat::default();
        let mut saved=false;
        loop{
            if !saved{
                self.stream_frames.read(&mut frame)?;
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

    pub fn body_detection(&mut self)->Result<(),opencv::Error>{
        highgui::named_window("Body Detection Tips:Press[(p, Take picture), (s, Save), (q, quit)]", highgui::WINDOW_FULLSCREEN)?;
        let mut frame: Mat = Mat::default();
        let mut saving=false;
        loop {
            if !saving{
                self.stream_frames.read(&mut frame)?;
                detect::hog_body_detector(&mut frame)?;
            }

            highgui::imshow("Body Detection Tips:Press[(p, Take picture), (s, Save), (q, quit)]", &frame)?;

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
        highgui::destroy_all_windows()?;
        Ok(())
    }

    pub fn face_detection(&mut self)->Result<(),opencv::Error>{
        highgui::named_window("Face Detection Tips:Press[(p, Take picture), (s, Save), (q, quit)]", highgui::WINDOW_FULLSCREEN)?;
        let mut frame: Mat = Mat::default();
        let mut saving=false;
        loop {
            if !saving{
                self.stream_frames.read(&mut frame)?;
                detect::haar_face_detector(&mut frame)?;
            }

            highgui::imshow("Face Detection Tips:Press[(p, Take picture), (s, Save), (q, quit)]", &frame)?;

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
        highgui::destroy_all_windows()?;
        Ok(())
    }

    pub fn moving_object_detection(&mut self,mini: i32,max: i32, fps: i32)->Result<(),opencv::Error>{
        //init
        let mut fps_adjuster = tools::FPSAdjuster::new(fps);
        highgui::named_window("Face Detection Tips:Press[(p, Take picture), (s, Save), (q, quit)]", highgui::WINDOW_FULLSCREEN)?;
        let mut frame_prev: Mat = Mat::default();
        let mut frame_next: Mat = Mat::default();
        let mut frame_show: Mat = Mat::default();
        let mut saving=false;
        self.stream_frames.read(&mut frame_next)?;
        //detect
        loop {
            if !saving{
                frame_prev.clone_from(&frame_next);
                self.stream_frames.read(&mut frame_next)?;
                fps_adjuster.start();
                frame_show=detect::moving_object_detector(&mut frame_prev,
                    &mut frame_next,
                    mini,
                    max)?;
                fps_adjuster.end();
            }

            highgui::imshow("Moving Object Detection Tips:Press[(p, Take picture), (s, Save), (q, quit)]", &frame_show)?;

            let mut key=highgui::wait_key(1)?;
            if saving{
                key = highgui::wait_key(500000)?;
            }

            if saving && key == 115{//s
                save_mat_as_image(&frame_show,"pics/Camera");
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
        highgui::destroy_all_windows()?;
        Ok(())
    }
}

impl Stream{
    pub fn get_frame(&mut self) -> Result<Mat,opencv::Error> {
        let mut frame = Mat::default();
        self.stream_frames.read(&mut frame)?;
        Ok(frame)
    }
    
    fn show_frame(frame:&Mat)->Result<(),opencv::Error>{
        highgui::named_window("show_frame", highgui::WINDOW_FULLSCREEN)?;
        highgui::imshow("show_frame", frame)?;
        let key = highgui::wait_key(50000)?;
        if key == 113 {//q
            return Ok(());
        }
        Ok(())
    }
}



