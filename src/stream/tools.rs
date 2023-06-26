use opencv::{
    core::{Mat,Vector},
    imgcodecs::{self, ImwriteFlags}
};
use chrono::{Local, Duration};
use std::{
    time::Instant,
    thread,
};

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
