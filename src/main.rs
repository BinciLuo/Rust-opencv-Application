use std::time::Duration;

use tokio::time;

mod camera;
mod folder;

fn main() {
    let folder_names=vec!["pics","pics/Camera","pics/Capture","pics/Person_detection","pics/Face_detection"];
    for folder_name in folder_names{
        folder::set_folder(folder_name);
    }
    
    let mut my_camera=camera::Camera::new(true,"328_1687706104.mp4");
    my_camera.moving_object_detection(4,30).unwrap();
    // let mut my_camera=camera::Camera::new(false,"328_1687706104.mp4");
    // my_camera.moving_object_detection(6,60);
    // my_camera.face_detection();
    // my_camera.body_detection();
    // my_camera.camera();
    // my_camera.capture_frame();
}