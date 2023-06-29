use opencv::{
    core::{Mat, Vector, Scalar, Size, CV_8UC1},
    imgcodecs::{self, ImwriteFlags,},
    prelude::{MatTraitConst, CascadeClassifierTrait, HOGDescriptorTraitConst, HOGDescriptorTrait},
    imgproc::{LINE_8, rectangle, cvt_color, COLOR_RGB2GRAY},
    objdetect::{CascadeClassifier, HOGDescriptor},
};

use chrono::Local;
use std::{
    time::Instant,
    thread,
};

use super::{FrameProcess, Frame};

pub struct FPSAdjuster{
    fram_per_seconds:i32,
    start: Instant,
    end: Instant,
}

impl FPSAdjuster{
    pub fn new(fps: i32) -> Self{
        Self { fram_per_seconds: fps, start: Instant::now(), end: Instant::now() }
    }

    pub fn start(&mut self) {
        self.start = Instant::now()
    }

    pub fn end(&mut self) {
        if self.fram_per_seconds==0{
            return;
        }
        self.end = Instant::now();
        let exec_duration = self.end - self.start;
        let time_per_frame = std::time::Duration::from_secs(1) / self.fram_per_seconds as u32;
        if  exec_duration < time_per_frame {
            thread::sleep(time_per_frame - exec_duration)
        }
    }
}

impl FrameProcess for Frame{
    fn body_detection(&mut self) -> Result<(), opencv::Error>{
        let mut hog = HOGDescriptor::default()?;
        hog.set_svm_detector(&HOGDescriptor::get_default_people_detector()?)?;
        let mut found_locations = Vector::new();
        hog.detect_multi_scale(self, &mut found_locations, 0., Size::default(), Size::default(), 1.05, 2.0, false)?;
        println!("hog 检测到{}个人体", found_locations.len());
        for rect in found_locations{
            rectangle(self, rect, Scalar::new(255., 0., 0., 255.), 2, LINE_8, 0)?;
        }
        Ok(())
    }

    fn face_detection(&mut self) -> Result<(), opencv::Error>{
        let mut classifier = CascadeClassifier::default()?;
        classifier.load("haarcascade_frontalface_alt2.xml")?;
        let mut gray = Mat::new_rows_cols_with_default(self.rows(), self.cols(), CV_8UC1, Scalar::default())?;    
        cvt_color(self, &mut gray, COLOR_RGB2GRAY, 0)?;
        let mut faces = Vector::new();
        classifier.detect_multi_scale(&gray, &mut faces, 1.1, 5, 0, Size::new(3, 3), Size::default())?;
        
        println!("haar 检测到{}个人脸", faces.len());
        for rec in faces{
            rectangle(self, rec, Scalar::new(0., 0., 0., 255.), 2, LINE_8, 0)?;
        }
        Ok(())
    }

    fn save_as_image(&self, file_path: &str) -> Result<(),opencv::Error>{
        let current_time = Local::now();
        let time_string = current_time.format("%Y-%m-%d[%H:%M:%S]").to_string();
        let mut params=Vector::<i32>::new();
        params.push(ImwriteFlags::IMWRITE_JPEG_QUALITY as i32);
        params.push(100);
        let _ = imgcodecs::imwrite(
            format!("{}/{}.jpeg",file_path,time_string).as_str(),
            self,
            &params
        );
        Ok(())
    }

    // TODO: fn read_from_img(file_path: &str) -> Result<Frame, opencv::Error> {}

    // TODO: fn moving_object_detection(frame_prev: &Self, frame_next: &Self, mini: i32, max: i32) -> Result<Frame, opencv::Error> {}
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
