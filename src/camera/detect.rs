//use anyhow::{Result, Ok};
use opencv::{
    prelude::*, objdetect::{CascadeClassifier, HOGDescriptor}, 
    core::{CV_8UC1, Scalar, Size, Vector,absdiff,Point,BORDER_CONSTANT,Rect}, 
    imgproc::{COLOR_RGB2GRAY, cvt_color, rectangle, LINE_8,THRESH_BINARY,get_structuring_element,MORPH_RECT,erode,threshold,dilate,find_contours,CHAIN_APPROX_SIMPLE,RETR_EXTERNAL,approx_poly_dp,bounding_rect}
};
use opencv::types::{VectorOfPoint, VectorOfRect};
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

pub fn moving_object_detector(frame_prev:&mut Mat,frame_next:&mut Mat)->Result<(),opencv::Error>{
    let mut gray_prev=Mat::new_rows_cols_with_default(frame_prev.rows(), frame_prev.cols(), CV_8UC1, Scalar::default())?;
    let mut gray_next=Mat::new_rows_cols_with_default(frame_next.rows(), frame_next.cols(), CV_8UC1, Scalar::default())?;
    let mut diff=Mat::new_rows_cols_with_default(frame_next.rows(), frame_next.cols(), CV_8UC1, Scalar::default())?;
    let mut diff2=Mat::new_rows_cols_with_default(frame_next.rows(), frame_next.cols(), CV_8UC1, Scalar::default())?;
    cvt_color(frame_prev, &mut gray_prev, COLOR_RGB2GRAY, 0)?;
    cvt_color(frame_next, &mut gray_next, COLOR_RGB2GRAY, 0)?;

    absdiff(&gray_prev,&gray_next,&mut diff)?;
    absdiff(&gray_prev,&gray_next,&mut diff2)?;
    threshold(&diff2,&mut diff,25.0,255.0,THRESH_BINARY)?;

    let mut element=get_structuring_element(MORPH_RECT,Size::new(4,4),Point::default())?;
    erode(&diff,&mut diff2,&element,Point::new(-1,-1), 1,BORDER_CONSTANT,Scalar::default())?;

    let mut element2=get_structuring_element(MORPH_RECT,Size::new(30,30),Point::default())?;
    dilate(&diff2,&mut diff,&element2,Point::new(-1,-1), 1,BORDER_CONSTANT,Scalar::default())?;

    let mut contours:Vector<Vector<Point>>=Vector::new();
    find_contours(&diff, &mut contours,RETR_EXTERNAL,CHAIN_APPROX_SIMPLE, Point::new(0, 0))?;
    
    for (i, contour) in contours.iter().enumerate() {
        let mut contour_poly = VectorOfPoint::new();
        approx_poly_dp(&contour, &mut contour_poly, 3.0, true)?;
        let bound_rect = bounding_rect(&contour_poly)?;
        //let (x, y, w, h) = (bound_rect.x, bound_rect.y, bound_rect.width, bound_rect.height);
        rectangle(frame_next, bound_rect, Scalar::new(0.0, 255.0, 0.0, 0.0), 2, LINE_8, 0)?;
    }
    Ok(())    
}