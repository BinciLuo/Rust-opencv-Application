# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run

```bash
# Build the project
cargo build

# Run the application
cargo run
```

## Project Architecture

This is a Rust computer vision application powered by OpenCV that detects bodies, faces, moving objects, and QR codes from video streams, camera feeds, or static images.

### Module Structure

- **src/main.rs**: Entry point demonstrating all features (frame processing from images, video streams, and camera)
- **src/folder.rs**: Folder management utilities (`set_folder` creates directories if they don't exist)
- **src/resources/**: Core computer vision functionality
  - **consts.rs**: Constants for stream sources (VIDEO=0, CAMERA=1)
  - **tools.rs**: Frame utility methods (`save_as_img`, `show`, `get_from_img`) and `FPSAdjuster`
  - **detection.rs**: Detection implementations for body, face, moving object, and QR code
  - **resources.rs**: Re-`pub use`s the resources module and defines:
    - `Stream` struct: Wraps `VideoCapture` for continuous video/camera input
    - `Frame` type: Alias for `opencv::core::Mat`
    - `FrameDetection` trait: Detection methods (`body_detection`, `face_detection`, `moving_object_detection`, `qrcode_detection`)
    - `FrameTools` trait: Frame utility methods

### Key Design Patterns

- **Stream API**: `Stream::from_video()` or `Stream::from_camera()` creates a source, then call detection methods like `body_detection()`, `face_detection()`, `moving_object_detection()`, or interactive methods like `camera()` and `capture_frame()`
- **Frame API**: `Frame::get_from_img()` for static images, then call detection methods directly on the frame
- **Traits**: `FrameDetection` and `FrameTools` provide method-style APIs for `Frame` (which is `opencv::Mat`)
- **Interactive UI**: GUI windows support keyboard shortcuts: `p` (take picture), `s` save), `q` (quit)

### Dependencies

- `opencv` 0.83.0: Core computer vision library
- `reqwest`: HTTP client
- `tokio`: Async runtime
- `chrono`: Time formatting for image filenames

### External Resources

The project requires `haarcascade_frontalface_alt2.xml` in the root directory for face detection.
