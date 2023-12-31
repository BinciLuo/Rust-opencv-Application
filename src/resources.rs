mod tools;
mod detection;
mod consts;

use opencv::{
    prelude::*,
    videoio::{self, VideoWriter},
    highgui,
    core::{Mat, Size},
};
use crate::folder::set_folder;

/*
    Stream

    Here is defination and impl for Stream
*/
pub struct Stream{
    stream_frames: videoio::VideoCapture,
    stream_source: i32,
}

impl Stream{
    /// # from Video
    /// ## Get a stream from video
    /// ## Arguements
    /// ### path
    /// - type: &str
    /// - description: get a stream from video
    /// ## Example usage
    /// `let mut video_stream = resources::Stream::from_video("example.mp4");`
    pub fn from_video(path:&str) -> Self{
        Self{
                stream_frames:videoio::VideoCapture::from_file(path,videoio::CAP_FFMPEG).unwrap(),
                stream_source: consts::VIDEO,
            }
    }

    /// # from Camera
    /// ## Get a stream from camera
    /// ## Example usage
    /// `let mut camera_stream = resources::Stream::from_camera();`
    pub fn from_camera() -> Self{
        Self {
                stream_frames: videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap(),
                stream_source: consts::CAMERA,
            }
    }

    pub fn camera(&mut self) -> Result<(),opencv::Error> {
        highgui::named_window("Rust Web Camera Tips:Press[(p, Take picture), (s, Save), (q, quit)]", highgui::WINDOW_FULLSCREEN)?;
        let mut frame = Frame::default();
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
                frame.save_as_img("pics/Camera")?;
                saving=false;
                continue;
            }
            saving=false;
            if key == 113 {//q
                break;
            }else if key == 112 {//p
                saving=true;
                continue;
            }
        }
        highgui::destroy_all_windows()?;
        Ok(())
    }

    pub fn capture_frame(&mut self)->Result<(),opencv::Error>{
        highgui::named_window("RustCapture Tips:Press[(s, Save), (q, quit)]", highgui::WINDOW_FULLSCREEN)?;
        let mut frame = Frame::default();
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
                frame.save_as_img("pics/Capture")?;
                saved=true;
                continue;
            }
        }
    }

    /// # Body Detection
    /// ## Detect Bodys
    /// ## Arguements 
    /// ### show
    /// - type: bool
    /// - description: whether to create a GUI window
    /// ### output_path
    /// - type: &str
    /// - description: save path, if you don't want to save, set it to ""
    /// ## Example usage
    /// `stream.body_detection(true, "")?;`
    pub fn body_detection(&mut self, show: bool, mut output_path: &str)->Result<(),opencv::Error>{
        // Get Stream Info
        let stream_width = self.stream_frames.get(opencv::videoio::CAP_PROP_FRAME_WIDTH)?;
        let stream_height = self.stream_frames.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)?;
        let fps = self.stream_frames.get(opencv::videoio::CAP_PROP_FPS)?;
        let frame_size = Size::new(stream_width as i32, stream_height as i32);
        let mut save_video = true;
        

        // Some Init
        if output_path == ""{
            set_folder("temp");
            save_video = false;
            output_path = "temp/temp_video.mp4";
        }
        let fourcc = VideoWriter::fourcc('m', 'p', '4', 'v').unwrap();
        let mut video_writer = VideoWriter::new(output_path, fourcc, fps, frame_size, true).unwrap();

        // Process
        highgui::named_window("Body Detection Tips:Press[(p, Take picture), (s, Save), (q, quit)]", highgui::WINDOW_FULLSCREEN)?;
        let mut frame: Frame = Frame::default();
        let mut saving=false;
        loop {
            if !saving {
                self.stream_frames.read(&mut frame)?;
                if frame.empty(){
                    break;
                }
                frame.body_detection()?;
                if save_video{
                    video_writer.write(&frame).unwrap();
                }
            }

            if show{
                highgui::imshow("Body Detection Tips:Press[(p, Take picture), (s, Save), (q, quit)]", &frame)?;
                let mut key=highgui::wait_key(1)?;
                if saving{
                    key = highgui::wait_key(500000)?;
                }
                if saving && key == 115{//s
                    frame.save_as_img("pics/Camera")?;
                    saving=false;
                    continue;
                }
                saving=false;
                if key == 113 {//q
                    break;
                }else if key == 112 {//p
                    saving=true;
                    continue;
                }
            }
            
        }
        highgui::destroy_all_windows()?;
        Ok(())
    }

    /// # Face Detection
    /// ## Detect faces
    /// ## Arguements 
    /// ### show
    /// - type: bool
    /// - description: whether to create a GUI window
    /// ### output_path
    /// - type: &str
    /// - description: save path, if you don't want to save, set it to ""
    /// ## Example usage
    /// `stream.face_detection(true, "")?;`
    pub fn face_detection(&mut self, show: bool, mut output_path: &str)->Result<(),opencv::Error>{
        // Get Stream Info
        let stream_width = self.stream_frames.get(opencv::videoio::CAP_PROP_FRAME_WIDTH)?;
        let stream_height = self.stream_frames.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)?;
        let fps = self.stream_frames.get(opencv::videoio::CAP_PROP_FPS)?;
        let frame_size = Size::new(stream_width as i32, stream_height as i32);
        let mut save_video = true;
        // Some Init
        if output_path == ""{
            set_folder("temp");
            save_video = false;
            output_path = "temp/temp_video.mp4";
        }
        let fourcc = VideoWriter::fourcc('m', 'p', '4', 'v').unwrap();
        let mut video_writer = VideoWriter::new(output_path, fourcc, fps, frame_size, true).unwrap();

        highgui::named_window("Face Detection Tips:Press[(p, Take picture), (s, Save), (q, quit)]", highgui::WINDOW_FULLSCREEN)?;
        let mut frame: Frame = Frame::default();
        let mut saving=false;
        loop {
            if !saving{
                self.stream_frames.read(&mut frame)?;
                if frame.empty(){
                    break;
                }
                frame.face_detection()?;
                if save_video{
                    video_writer.write(&frame).unwrap();
                }    
            }
            if show{
                highgui::imshow("Face Detection Tips:Press[(p, Take picture), (s, Save), (q, quit)]", &frame)?;

                let mut key=highgui::wait_key(1)?;
                if saving{
                    key = highgui::wait_key(500000)?;
                }

                if saving && key == 115{//s
                    frame.save_as_img("pics/Camera")?;
                    saving=false;
                    continue;
                }
                saving=false;
                if key == 113 {//q
                    break;
                }else if key == 112 {//p
                    saving=true;
                    continue;
                }
            }
            
        }
        highgui::destroy_all_windows()?;
        Ok(())
    }

    /// # Moving Object Detection
    /// ## Detect moving object from stream
    /// ## Arguements 
    /// ### mini
    /// - type: i32
    /// - description: for erode
    /// ### max
    /// - type: i32
    /// - description: for dilate
    /// ### show_fps
    /// - type: i32
    /// - description: how many frames per seconds(set 0 to ignore)
    /// 
    /// ### show
    /// - type: bool
    /// - description: whether to create a GUI window
    /// ### output_path
    /// - type: &str
    /// - description: save path, if you don't want to save, set it to ""
    /// ## Example usage
    /// `stream.moving_object_detection(6, 60, 60, true, "")?;`
    pub fn moving_object_detection(&mut self, mini: i32, max: i32, show_fps: i32, show: bool, mut output_path: &str)->Result<(),opencv::Error>{
        // Get Stream Info
        let stream_width = self.stream_frames.get(opencv::videoio::CAP_PROP_FRAME_WIDTH)?;
        let stream_height = self.stream_frames.get(opencv::videoio::CAP_PROP_FRAME_HEIGHT)?;
        let fps = self.stream_frames.get(opencv::videoio::CAP_PROP_FPS)?;
        let frame_size = Size::new(stream_width as i32, stream_height as i32);
        let mut save_video = true;

        // Some Init
        if output_path == ""{
            set_folder("temp");
            save_video = false;
            output_path = "temp/temp_video.mp4";
        }
        let fourcc = VideoWriter::fourcc('m', 'p', '4', 'v').unwrap();
        let mut video_writer = VideoWriter::new(output_path, fourcc, fps, frame_size, true).unwrap();

        //init
        let mut fps_adjuster = tools::FPSAdjuster::new(show_fps);
        highgui::named_window("Face Detection Tips:Press[(p, Take picture), (s, Save), (q, quit)]", highgui::WINDOW_FULLSCREEN)?;
        let mut frame_prev: Frame = Frame::default();
        let mut frame_next: Frame = Frame::default();
        let mut frame_show: Frame = Frame::default();
        let mut saving=false;
        self.stream_frames.read(&mut frame_next)?;
        //detect
        loop {
            fps_adjuster.start();
            if !saving{
                frame_prev.clone_from(&frame_next);
                self.stream_frames.read(&mut frame_next)?;
                if frame_next.empty() {
                    break;
                }
                frame_show=Frame::moving_object_detection(&mut frame_prev,&mut frame_next, mini, max)?;
                if save_video{
                    video_writer.write(&frame_show).unwrap();
                }
            }

            if show{
                fps_adjuster.end();
                highgui::imshow("Moving Object Detection Tips:Press[(p, Take picture), (s, Save), (q, quit)]", &frame_show)?;

                let mut key=highgui::wait_key(1)?;
                if saving{
                    key = highgui::wait_key(500000)?;
                }

                if saving && key == 115{//s
                    frame_show.save_as_img("pics/Camera")?;
                    saving=false;
                    continue;
                }
                saving=false;
                if key == 113 {//q
                    break;
                }else if key == 112 {//p
                    saving=true;
                    continue;
                }
            }

            
        }
        highgui::destroy_all_windows()?;
        Ok(())
    }
}

impl Stream{
    pub fn get_frame(&mut self) -> Result<Frame,opencv::Error> {
        let mut frame = Frame::default();
        self.stream_frames.read(&mut frame)?;
        Ok(frame)
    }
}



/*
    Frame

    Here is defination and impl for Frame
*/
pub(crate) type Frame=Mat;

pub(crate) trait FrameDetection {
    fn body_detection(&mut self) -> Result<(),opencv::Error>;
    fn face_detection(&mut self) -> Result<(),opencv::Error>;
    fn moving_object_detection(frame_prev: &mut Self, frame_next: &mut Self, mini: i32, max: i32) -> Result<Frame, opencv::Error>;
    fn qrcode_detection(&mut self) -> Result<String,opencv::Error>;
}

pub(crate) trait FrameTools {
    /// # Save as IMG
    /// ## Save a frame as image
    /// ## Arguements
    /// ### file_path
    /// - type: &str
    /// - description: path to save the image
    /// ## Example usage
    /// `frame.save_as_img("example.png")`
    fn save_as_img(&self, file_path: &str) -> Result<(), opencv::Error>;

    /// # Show
    /// ## Show a frame
    /// ## Arguements
    /// ### window_name
    /// - type: &str
    /// - description: name of window
    /// ## Example usage
    /// ### 
    fn show(&self,window_name: &str) -> Result<(),opencv::Error>;

    /// # Get from IMG
    /// ## Get a frame from image
    /// ## Arguements
    /// ### file_path
    /// - type: &str
    /// - description: path to read image
    /// ## Example usage
    /// `let mut pic_frame = resources::Frame::get_from_img("example.jpeg")?;`
    fn get_from_img(file_path: &str) -> Result<Frame, opencv::Error>;
}