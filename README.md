# Rust Web Camera
## Introduction
A Camera that can take pics and capture frames.
## Preview 
![](for_readme/2023-06-23%5B17%3A56%3A24%5D.jpeg)
![](for_readme/3201687514247_.pic.jpg)

<span id="Requirements"></span>
## Requirements
- Install opencv 

Macos: 
`brew install opencv` 

Linux: 
1. Download opencv-4.x.x source code in https://opencv.org/releases/
2. unzip it to anywhere you want
3. `cd opencv-4.x.x`
4. `mkdir build && cd build`
5. `cmake -D WITH_TBB=ON -D WITH_EIGEN=ON -D OPENCV_GENERATE_PKGCONFIG=ON  -D BUILD_DOCS=ON -D BUILD_TESTS=OFF -D BUILD_PERF_TESTS=OFF -D BUILD_EXAMPLES=OFF  -D WITH_OPENCL=OFF -D WITH_CUDA=OFF -D BUILD_opencv_gpu=OFF -D BUILD_opencv_gpuarithm=OFF -D BUILD_opencv_gpubgsegm=O -D CMAKE_BUILD_TYPE=RELEASE -D CMAKE_INSTALL_PREFIX=/usr/local ..`
6. `make -j8`
7. `make install`
8. `sudo -i vim /etc/ld.so.conf.d/opencv.conf` and then add `/usr/local/lib` in it
9. `echo 'PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/usr/local/lib/pkgconfig' >> /etc/bash.bashrc && echo 'export PKG_CONFIG_PATH' >> /etc/bash.bashrc`
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

## TroubleShooting
### Macos
Temporarily None
### Linux
#### `openssl`
Message: `failed to run custom build command for openssl-sys v0.9.60` 
Solution: `sudo apt install libssl-dev`
#### `llvm`
Message: `Could not execute 'llvm-config' one or more times` 
Solution: `apt install llvm clang libclang-dev`
#### `videoio compile error`
Message: `function or associated item not found in 'VideoCapture'` 
Solution: OpenCV version too old. first exec `sudo apt-get purge *libopencv* && sudo apt-get autoremove && sudo apt-get autoclean` to uninstall old opencv and then install it again follow tips in [Requirements](#Requirements).
#### `gtk`
Message: `Failed to load module acnberra-gtk-module` 
Solution: `sudo  apt install libcanberra-gtk-moudle`