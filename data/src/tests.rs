#[test]
fn read_packet() {
    let data: Vec<u8> = (0..128).collect();
    let mut buf = Cursor::new(data.clone());

    match buf.get_packet(64) {
        Ok(pkt) => assert_eq!(pkt.data, &data[..64]),
        _ => unreachable!(),
    }
}

/*#[test]
fn test_new(){
    let pkt = Packet::new();
    assert_eq!(0, pkt.data.len());
}*/

#[test]
fn write_packet() {
    let size = 1024;
    let mut buf = Cursor::new(Vec::with_capacity(size));

    let mut pkt = Packet::with_capacity(size);

    for i in 0..size {
        pkt.data.push(i as u8);
    }

    buf.put_packet(pkt).unwrap();

    let vec = buf.into_inner();

    for (i, elem) in vec.iter().enumerate().take(size) {
        println!("{}", elem);
        assert!(*elem == i as u8);
    }
}

#[test]
fn fmt() {
    println!("{}", formats::S16);
    println!("{}", formats::U8);
    println!("{}", formats::F32);
}

#[test]
fn test_format_cmp() {
    let mut map = ChannelMap::new();
    map.add_channel(ChannelType::C);

    let sn = Arc::new(formats::S16);
    let info1 = AudioInfo::new(42, 48000, map.clone(), sn, None);

    let sn = Arc::new(formats::S16);
    let info2 = AudioInfo::new(4242, 48000, map.clone(), sn, None);

    assert!(info1 == info2);

    let mut map = ChannelMap::new();
    map.add_channel(ChannelType::C);
    let sn = Arc::new(formats::S16);
    let info1 = AudioInfo::new(42, 48000, map.clone(), sn, None);

    let sn = Arc::new(formats::S32);
    let info2 = AudioInfo::new(42, 48000, map.clone(), sn, None);

    assert!(!(info1 == info2));
}

use crate::pixel::formats::{RGB565, YUV420};

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
    println!("formaton yuv- {}", formats::YUV420);
    println!("formaton pal- {}", formats::PAL8);
    println!("formaton rgb565- {}", formats::RGB565);
    println!("formaton rgba- {}", formats::RGBA);
    println!("formaton rgb48- {}", formats::RGB48);
    println!("formaton rgba64- {}", formats::RGBA64);
}

#[test]
fn comparison() {
    use std::sync::Arc;
    let rcf = Arc::new(*formats::YUV420);
    let cf = &formats::YUV420.clone();

    if cf != formats::YUV420 {
        panic!("cf");
    }

    if *rcf != *formats::YUV420 {
        panic!("rcf");
    }
}

fn p<'a, T>(v: T)
where
    T: Into<Value<'a>> + Debug,
{
    println!("{:?}", v);
}

#[test]
fn value_str() {
    p("test");
}
