//use anyhow::{Result, Ok};
use opencv::{prelude::*, objdetect::{CascadeClassifier, HOGDescriptor}, core::{CV_8UC1, Scalar, Size, Vector}, imgproc::{COLOR_RGB2GRAY, cvt_color, rectangle, LINE_8}};

pub fn hog_body_detector(frame:&mut Mat)->Result<(), opencv::Error>{
    let mut hog = HOGDescriptor::default()?;
    hog.set_svm_detector(&HOGDescriptor::get_default_people_detector()?)?;
    let mut found_locations = Vector::new();
    hog.detect_multi_scale(frame, &mut found_locations, 0., Size::default(), Size::default(), 1.05, 2.0, false)?;
    println!("hog 检测到{}个人体", found_locations.len());
    for rect in found_locations{
        rectangle(frame, rect, Scalar::new(255., 0., 0., 255.), 2, LINE_8, 0)?;
    }
    Ok(())
}

pub fn haar_face_detector(frame:&mut Mat) -> Result<(), opencv::Error>{
    let mut classifier = CascadeClassifier::default()?;
    classifier.load("haarcascade_frontalface_alt2.xml")?;
    let mut gray = Mat::new_rows_cols_with_default(frame.rows(), frame.cols(), CV_8UC1, Scalar::default())?;    
    cvt_color(frame, &mut gray, COLOR_RGB2GRAY, 0)?;
    let mut faces = Vector::new();
    classifier.detect_multi_scale(&gray, &mut faces, 1.1, 5, 0, Size::new(3, 3), Size::default())?;
    
    println!("haar 检测到{}个人脸", faces.len());
    for rec in faces{
        rectangle(frame, rec, Scalar::new(0., 0., 0., 255.), 2, LINE_8, 0)?;
    }
    Ok(())
}
