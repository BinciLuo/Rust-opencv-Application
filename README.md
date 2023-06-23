# Rust Web Camera
## Introduction
A Camera that can take pics and capture frames.
## Preview 
![](for_readme/2023-06-23%5B17%3A56%3A24%5D.jpeg)
![](for_readme/3201687514247_.pic.jpg)

## Requirements
- Install opencv 

`brew install opencv`
## Usage
- Clone this repository 

`git clone https://github.com/BinciLuo/RustWebCam.git` 

`cd RustWebCam`
- Build 

`cargo build`
- Run 

`cargo run` 

It wiil create folders `pics`,`pics/Camera`,`pic/Capture` if they don't exist.

## Classes and Methods Implemented
Most important parts are in `src/camera.rs`.
```Rust
// Camera Defination
pub struct Camera{
    cam:videoio::VideoCapture,
}

impl Camera{
    pub fn new()->Self{
    }

    pub fn camera(&mut self) -> Result<(),opencv::Error> {
    }

    pub fn capture_frame(&mut self)->Result<(),opencv::Error>{
    }
}

// If you want to get one frame
pub fn get_frame() -> Result<Mat,opencv::Error> {
}

pub fn show_frame(frame:&Mat)->Result<(),opencv::Error>{
}

// If you want to save a Mat to jepg (It will be stored in {file_path}/{"%Y-%m-%d[%H:%M:%S]"}.jpeg)
pub fn save_mat_as_image(mat: &Mat, file_path: &str) {
    }
```