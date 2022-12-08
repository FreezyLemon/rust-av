//! Packets definitions.

#![allow(dead_code)]

use crate::timeinfo::TimeInfo;
use alloc::{vec, vec::Vec};

#[cfg(feature = "std")]
use std::io::{Read, Result, Write};

/// Packet with compressed data.
#[derive(Default, Debug, Clone)]
pub struct Packet {
    /// Packet data.
    pub data: Vec<u8>,
    /// Packet position in the stream.
    ///
    /// If `None`, the packet is not associated to a stream.
    pub pos: Option<usize>,
    /// Type of stream the packet is associated to.
    pub stream_index: isize,
    /// Packet timestamp information.
    pub t: TimeInfo,

    /// Tells whether a packet contains a keyframe.
    pub is_key: bool,
    /// Tells whether a packet is corrupted.
    pub is_corrupted: bool,
}

impl Packet {
    /// Creates a new empty `Packet` of a determined capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Packet {
            data: Vec::with_capacity(capacity),
            t: TimeInfo::default(),
            pos: None,
            stream_index: -1,
            is_key: false,
            is_corrupted: false,
        }
    }

    /// Creates a zero-initalized `Packet` of a determined capacity.
    pub fn zeroed(size: usize) -> Self {
        Packet {
            data: vec![0; size],
            t: TimeInfo::default(),
            pos: None,
            stream_index: -1,
            is_key: false,
            is_corrupted: false,
        }
    }

    /// Creates a new empty `Packet`.
    pub fn new() -> Self {
        Self::with_capacity(0)
    }
}

/// Used to read a packet from a source.
#[cfg(feature = "std")]
pub trait ReadPacket: Read {
    /// Reads a packet from a source.
    fn get_packet(&mut self, len: usize) -> Result<Packet> {
        let mut pkt = Packet::zeroed(len);
        self.read_exact(pkt.data.as_mut_slice())?;
        Ok(pkt)
    }
}

/// Used to write a packet into a source.
#[cfg(feature = "std")]
pub trait WritePacket: Write {
    /// Writes a packet into a source.
    fn put_packet(&mut self, pkt: Packet) -> Result<()> {
        self.write_all(pkt.data.as_slice())
    }
}

#[cfg(feature = "std")]
impl<R: Read + ?Sized> ReadPacket for R {}

#[cfg(feature = "std")]
impl<W: Write + ?Sized> WritePacket for W {}

use alloc::sync::Arc;

/// A specialized type for a thread-safe reference-counting pointer `Packet`.
pub type ArcPacket = Arc<Packet>;
