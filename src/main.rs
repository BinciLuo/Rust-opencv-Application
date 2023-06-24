use std::time::Duration;

use tokio::time;

mod camera;
mod folder;

fn main() {
    let folder_names=vec!["pics","pics/Camera","pics/Capture","pics/Person_detection","pics/Face_detection"];
    for folder_name in folder_names{
        folder::set_folder(folder_name);
    }


    let mut my_camera=camera::Camera::new();
    my_camera.face_detection();
    my_camera.body_detection();
    my_camera.camera();
    my_camera.capture_frame();
}



