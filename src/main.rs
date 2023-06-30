mod resources;
mod folder;
use resources::{FrameTools, FrameDetection};



fn main() ->Result<(),opencv::Error>{
    // Create folders to save pics
    let folder_names=vec!["pics","pics/Camera","pics/Capture","pics/Person_detection","pics/Face_detection"];
    for folder_name in folder_names{
        folder::set_folder(folder_name);
    }

    // Use Pics as Frame
    let mut baoguo_pic = resources::Frame::get_from_img("TeacherBaoguo.jpeg")?;
    baoguo_pic.face_detection()?;
    baoguo_pic.show("Baoguo")?;
    baoguo_pic.save_as_img("for_readme").unwrap();

    // Use Video as Stream
    let mut video_stream = resources::Stream::from_video("328_1687706104.mp4");
    video_stream.moving_object_detection(3, 20, 30, false, "moving_video.mp4")?;

    // // Use Camera as Stream
    // let mut my_camera = resources::Stream::from_camera();
    // my_camera.moving_object_detection(6, 60, 60,true, "moving_cam.mp4")?;
    // my_camera.face_detection(true, "face_cam.mp4")?;
    // my_camera.body_detection(true,"body_cam.mp4")?;
    // my_camera.camera()?;
    // my_camera.capture_frame()?;

    Ok(())
}