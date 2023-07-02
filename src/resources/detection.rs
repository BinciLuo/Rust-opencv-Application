use opencv::{
    core::{absdiff, Point, Mat, Vector, Scalar, Size, CV_8UC1, BORDER_CONSTANT, Rect},
    prelude::{MatTraitConst, CascadeClassifierTrait, HOGDescriptorTraitConst, HOGDescriptorTrait, QRCodeDetectorTrait},
    imgproc::{threshold, erode, get_structuring_element, MORPH_RECT, dilate, find_contours, bounding_rect, approx_poly_dp,LINE_8, THRESH_BINARY, rectangle, cvt_color, RETR_EXTERNAL, COLOR_RGB2GRAY, CHAIN_APPROX_SIMPLE},
    objdetect::{CascadeClassifier, HOGDescriptor, QRCodeDetector},
    types::VectorOfPoint,
};
use super::{FrameDetection, Frame};

impl FrameDetection for Frame{
    fn body_detection(&mut self) -> Result<(), opencv::Error>{
        let mut hog = HOGDescriptor::default()?;
        hog.set_svm_detector(&HOGDescriptor::get_default_people_detector()?)?;
        let mut found_locations = Vector::new();
        hog.detect_multi_scale(self, &mut found_locations, 0., Size::default(), Size::default(), 1.05, 2.0, false)?;
        for rect in found_locations{
            rectangle(self, rect, Scalar::new(255., 0., 0., 255.), 2, LINE_8, 0)?;
        }
        Ok(())
    }

    fn face_detection(&mut self) -> Result<(), opencv::Error>{
        let mut classifier = CascadeClassifier::default()?;
        classifier.load("haarcascade_frontalface_alt2.xml")?;
        let mut gray = Mat::new_rows_cols_with_default(self.rows(), self.cols(), CV_8UC1, Scalar::default())?;    
        cvt_color(self, &mut gray, COLOR_RGB2GRAY, 0)?;
        let mut faces = Vector::new();
        classifier.detect_multi_scale(&gray, &mut faces, 1.1, 5, 0, Size::new(3, 3), Size::default())?;
        
        for rec in faces{
            rectangle(self, rec, Scalar::new(0., 0., 0., 255.), 2, LINE_8, 0)?;
        }
        Ok(())
    }

    fn moving_object_detection(frame_prev:&mut Self, frame_next:&mut Self, mini: i32, max: i32) -> Result<Mat, opencv::Error>{
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

    fn qrcode_detection(&mut self) -> Result<String,opencv::Error> {
        let mut qr_detector = QRCodeDetector::default()?;

        // 识别二维码
        let mut points = opencv::types::VectorOfPoint::new();
        let data: Vec<u8> = qr_detector.detect_and_decode(self, &mut points, &mut Mat::default())?;
        let url = String::from_utf8(data).unwrap();

        let rect = Rect::from_points(points.get(0)? , points.get(2)?);
        rectangle(
            self,
            rect,
            Scalar::new(0.0, 255.0, 0.0, 0.0),
            2,
            8,
            0,
        )?;
        Ok(url)
    }
}
