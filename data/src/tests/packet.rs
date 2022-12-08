use std::io::Cursor;

use av_data::packet::*;

#[test]
fn read_packet() {
    let data: Vec<u8> = (0..128).collect();
    let mut buf = Cursor::new(data.clone());

    match buf.get_packet(64) {
        Ok(pkt) => assert_eq!(pkt.data, &data[..64]),
        _ => unreachable!(),
    }
}

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
