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

    let mut qr_pic = resources::Frame::get_from_img("8c0aa0e220417fb1011dd4ae28388345.jpg")?;
    let urls = qr_pic.qrcode_detection()?;
    println!("urls : {:#?}",urls);
    qr_pic.show("window_name")?;
    

    // Use Video as Stream
    let mut video_stream = resources::Stream::from_video("328_1687706104.mp4");
    video_stream.moving_object_detection(3, 20, 30, false, "")?;

    // Use Camera as Stream
    let mut my_camera = resources::Stream::from_camera();
    my_camera.moving_object_detection(6, 60, 60, true, "")?;
    my_camera.face_detection(true, "")?;
    my_camera.body_detection(true,"")?;
    my_camera.camera()?;
    my_camera.capture_frame()?;

    Ok(())
}