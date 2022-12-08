use av_data::audiosample::formats::*;
use av_data::value::*;

use core::fmt::Debug;

#[test]
fn audiosample_fmt() {
    println!("s16: {S16}");
    println!("u8: {U8}");
    println!("f32: {F32}");
}

fn p<'a, T>(v: T)
where
    T: Into<Value<'a>> + Debug,
{
    println!("{v:?}");
}

#[test]
fn value_str() {
    p("test");
}
