extern crate alloc;
use alloc::sync::Arc;

use av_data::audiosample::*;
use av_data::audiosample::formats::*;
use av_data::frame::*;

#[test]
fn test_format_cmp() {
    let mut map = ChannelMap::new();
    map.add_channel(ChannelType::C);

    let sn = Arc::new(S16);
    let info1 = AudioInfo::new(42, 48000, map.clone(), sn, None);

    let sn = Arc::new(S16);
    let info2 = AudioInfo::new(4242, 48000, map.clone(), sn, None);

    assert!(info1 == info2);

    let mut map = ChannelMap::new();
    map.add_channel(ChannelType::C);
    let sn = Arc::new(S16);
    let info1 = AudioInfo::new(42, 48000, map.clone(), sn, None);

    let sn = Arc::new(S32);
    let info2 = AudioInfo::new(42, 48000, map.clone(), sn, None);

    assert!(!(info1 == info2));
}
