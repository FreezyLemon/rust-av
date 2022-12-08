extern crate alloc;
use alloc::sync::Arc;

use av_data::pixel::*;
use av_data::pixel::formats::*;
use av_data::frame::*;

#[test]
fn comparison() {
    let rcf = Arc::new(*YUV420);
    let cf = &YUV420.clone();

    if cf != YUV420 {
        panic!("cf");
    }

    if *rcf != *YUV420 {
        panic!("rcf");
    }
}

#[test]
fn test_video_format_cmp() {
    let yuv420: Formaton = *YUV420;
    let fm = Arc::new(yuv420);
    let info1 = VideoInfo::new(42, 42, false, FrameType::I, fm);

    let yuv420: Formaton = *YUV420;
    let fm = Arc::new(yuv420);
    let info2 = VideoInfo::new(42, 42, false, FrameType::P, fm);

    assert!(info1 == info2);

    let yuv420: Formaton = *YUV420;
    let fm = Arc::new(yuv420);
    let info1 = VideoInfo::new(42, 42, false, FrameType::I, fm);

    let rgb565: Formaton = *RGB565;
    let fm = Arc::new(rgb565);
    let info2 = VideoInfo::new(42, 42, false, FrameType::I, fm);

    assert!(!(info1 == info2));
}

#[test]
#[should_panic]
fn test_frame_copy_from_slice() {
    let yuv420: Formaton = *YUV420;
    let fm = Arc::new(yuv420);
    let video_info = VideoInfo::new(42, 42, false, FrameType::I, fm);

    let mut frame = Frame::new_default_frame(MediaKind::Video(video_info), None);

    frame.copy_from_slice(
        vec![vec![0].as_slice(); 2].into_iter(),
        vec![40; 2].into_iter(),
    );
}

#[test]
fn fmt() {
    println!("formaton yuv- {}", YUV420);
    println!("formaton pal- {}", PAL8);
    println!("formaton rgb565- {}", RGB565);
    println!("formaton rgba- {}", RGBA);
    println!("formaton rgb48- {}", RGB48);
    println!("formaton rgba64- {}", RGBA64);
}
