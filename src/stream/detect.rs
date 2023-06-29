use opencv::{
    prelude::*, 
    core::{CV_8UC1, Scalar, Size, Vector, absdiff, Point, BORDER_CONSTANT}, 
    imgproc::{COLOR_RGB2GRAY, cvt_color, rectangle, LINE_8, THRESH_BINARY, get_structuring_element, MORPH_RECT, erode, threshold, dilate, find_contours, CHAIN_APPROX_SIMPLE, RETR_EXTERNAL, approx_poly_dp, bounding_rect},
    types::VectorOfPoint,
};

pub fn moving_object_detector(frame_prev:&mut Mat, frame_next:&mut Mat, mini: i32, max: i32) -> Result<Mat, opencv::Error>{
    let mut gray_prev = Mat::new_rows_cols_with_default(frame_prev.rows(), frame_prev.cols(), CV_8UC1, Scalar::default())?;
    let mut gray_next = Mat::new_rows_cols_with_default(frame_next.rows(), frame_next.cols(), CV_8UC1, Scalar::default())?;
    let mut diff = Mat::new_rows_cols_with_default(frame_next.rows(), frame_next.cols(), CV_8UC1, Scalar::default())?;
    let mut diff2 = Mat::new_rows_cols_with_default(frame_next.rows(), frame_next.cols(), CV_8UC1, Scalar::default())?;
    let mut frame_result = Mat::default();
    frame_result.clone_from(frame_next);
    cvt_color(frame_prev, &mut gray_prev, COLOR_RGB2GRAY, 0)?;
    cvt_color(frame_next, &mut gray_next, COLOR_RGB2GRAY, 0)?;

    absdiff(&gray_prev, &gray_next, &mut diff)?;
    absdiff(&gray_prev, &gray_next, &mut diff2)?;
    threshold(&diff2, &mut diff, 25.0, 255.0, THRESH_BINARY)?;

    let element = get_structuring_element(MORPH_RECT, Size::new(mini,mini), Point::default())?;
    erode(&diff, &mut diff2, &element, Point::new(-1,-1), 1, BORDER_CONSTANT, Scalar::default())?;

    let element=get_structuring_element(MORPH_RECT, Size::new(max,max), Point::default())?;
    dilate(&diff2, &mut diff, &element, Point::new(-1,-1), 1, BORDER_CONSTANT, Scalar::default())?;

    let mut contours:Vector<Vector<Point>> = Vector::new();
    find_contours(&diff, &mut contours, RETR_EXTERNAL, CHAIN_APPROX_SIMPLE, Point::new(0, 0))?;
    
    for contour in contours.iter() {
        let mut contour_poly = VectorOfPoint::new();
        approx_poly_dp(&contour, &mut contour_poly, 3.0, true)?;
        let bound_rect = bounding_rect(&contour_poly)?;
        rectangle(&mut frame_result, bound_rect, Scalar::new(0.0, 255.0, 0.0, 0.0), 2, LINE_8, 0)?;
    }
    Ok(frame_result)
}
