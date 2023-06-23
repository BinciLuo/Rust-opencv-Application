mod camera;
mod folder;

fn main() {
    let mut folder_name = "pics";
    folder::set_folder(folder_name);
    folder_name = "pics/Camera";
    folder::set_folder(folder_name);
    folder_name = "pics/Capture";
    folder::set_folder(folder_name);


    let mut my_camera=camera::Camera::new();
    my_camera.camera().unwrap();
    my_camera.capture_frame().unwrap();
}



