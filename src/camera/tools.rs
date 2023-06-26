use opencv::{
    core::{Mat,Vector},
    imgcodecs::{self, ImwriteFlags}
};
use chrono::Local;


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