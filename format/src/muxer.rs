use crate::common::*;
use crate::data::packet::Packet;
use crate::data::value::*;
use std::any::Any;
use std::io::{ErrorKind, Seek, SeekFrom, Write};
use std::sync::Arc;

use crate::error::*;

pub trait WriteSeek: Write + Seek {}

impl<T: Write + Seek> WriteSeek for T {}

/// Runtime wrapper around either a [`Write`] or a [`WriteSeek`] trait object which supports querying
/// for seek support.
pub enum Writer<W: Write, WS: WriteSeek> {
    NonSeekable(W, u64),
    Seekable(WS),
}

impl<W: Write, WS: WriteSeek> Writer<W, WS> {
    /// Creates a [`Writer`] from an object that implements both [`Write`] and [`Seek`] traits.
    pub fn from_seekable(inner: WS) -> Self {
        Self::Seekable(inner)
    }

    /// Creates a [`Writer`] from an object that implements the [`Write`] trait.
    pub fn from_nonseekable(inner: W) -> Self {
        Self::NonSeekable(inner, 0)
    }

    /// Returns whether the [`Writer`] can seek within the source.
    pub fn can_seek(&self) -> bool {
        matches!(self, Self::Seekable(_))
    }
}

impl<W: Write, WS: WriteSeek> Write for Writer<W, WS> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        match self {
            Self::NonSeekable(inner, ref mut index) => {
                let result = inner.write(bytes);

                if let Ok(written) = result {
                    *index += written as u64;
                }

                result
            }
            Self::Seekable(inner) => inner.write(bytes),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            Self::NonSeekable(inner, ..) => inner.flush(),
            Self::Seekable(inner) => inner.flush(),
        }
    }
}

impl<W: Write, WS: WriteSeek> Seek for Writer<W, WS> {
    fn seek(&mut self, seek: SeekFrom) -> std::io::Result<u64> {
        match self {
            Self::NonSeekable(_, index) => {
                if let SeekFrom::Current(0) = seek {
                    Ok(*index)
                } else {
                    Err(std::io::Error::new(
                        ErrorKind::Other,
                        "Seeking not supported",
                    ))
                }
            }
            Self::Seekable(inner) => inner.seek(seek),
        }
    }
}

/// Used to implement muxing operations.
pub trait Muxer: Send {
    /// Configures a muxer.
    fn configure(&mut self) -> Result<()>;
    /// Writes a stream header into a data structure implementing
    /// the `Write` trait.
    fn write_header<W: Write, WS: WriteSeek>(&mut self, out: &mut Writer<W, WS>) -> Result<()>;
    /// Writes a stream packet into a data structure implementing
    /// the `Write` trait.
    fn write_packet<W: Write, WS: WriteSeek>(
        &mut self,
        out: &mut Writer<W, WS>,
        pkt: Arc<Packet>,
    ) -> Result<()>;
    /// Writes a stream trailer into a data structure implementing
    /// the `Write` trait.
    fn write_trailer<W: Write, WS: WriteSeek>(&mut self, out: &mut Writer<W, WS>) -> Result<()>;

    /// Sets global media file information for a muxer.
    fn set_global_info(&mut self, info: GlobalInfo) -> Result<()>;
    /// Sets a muxer option.
    ///
    /// This method should be called as many times as the number of options
    /// present in a muxer.
    fn set_option<'a>(&mut self, key: &str, val: Value<'a>) -> Result<()>;
}

/// Auxiliary structure to encapsulate a muxer object and
/// its additional data.
pub struct Context<M: Muxer + Send, W: Write, WS: WriteSeek> {
    muxer: M,
    writer: Writer<W, WS>,
    /// User private data.
    ///
    /// This data cannot be cloned.
    pub user_private: Option<Box<dyn Any + Send + Sync>>,
}

impl<M: Muxer + Send, W: Write, WS: WriteSeek> Context<M, W, WS> {
    /// Creates a new `Context` instance.
    pub fn new(muxer: M, writer: Writer<W, WS>) -> Self {
        Context {
            muxer,
            writer,
            user_private: None,
        }
    }

    /// Configures a muxer.
    pub fn configure(&mut self) -> Result<()> {
        self.muxer.configure()
    }

    /// Writes a stream header to an internal buffer and returns how many
    /// bytes were written or an error.
    pub fn write_header(&mut self) -> Result<()> {
        self.muxer.write_header(&mut self.writer)
    }

    /// Writes a stream packet to an internal buffer and returns how many
    /// bytes were written or an error.
    pub fn write_packet(&mut self, pkt: Arc<Packet>) -> Result<()> {
        self.muxer.write_packet(&mut self.writer, pkt)
    }

    /// Writes a stream trailer to an internal buffer and returns how many
    /// bytes were written or an error.
    pub fn write_trailer(&mut self) -> Result<()> {
        self.muxer.write_trailer(&mut self.writer)?;
        self.writer.flush()?;

        Ok(())
    }

    /// Sets global media file information for a muxer.
    pub fn set_global_info(&mut self, info: GlobalInfo) -> Result<()> {
        self.muxer.set_global_info(info)
    }

    /// Sets a muxer option.
    ///
    /// This method should be called as many times as the number of options
    /// present in a muxer.
    pub fn set_option<'a, V>(&mut self, key: &str, val: V) -> Result<()>
    where
        V: Into<Value<'a>>,
    {
        self.muxer.set_option(key, val.into())
    }
}

/// Format descriptor.
///
/// Contains information on a format and its own muxer.
#[derive(Clone, Debug, PartialEq)]
pub struct Descr {
    /// Format name.
    pub name: &'static str,
    /// Muxer name.
    pub demuxer: &'static str,
    /// Format description.
    pub description: &'static str,
    /// Format media file extensions.
    pub extensions: &'static [&'static str],
    /// Format MIME.
    pub mime: &'static [&'static str],
}

/// Used to get a format descriptor and create a new muxer.
pub trait Descriptor {
    type OutputMuxer: Muxer + Send;

    /// Creates a new muxer for the requested format.
    fn create(&self) -> Self::OutputMuxer;
    /// Returns the descriptor of a format.
    fn describe(&self) -> &Descr;
}

/// Used to look for a specific format.
pub trait Lookup<T: Descriptor + ?Sized> {
    /// Retrieves a specific format by name.
    fn by_name(&self, name: &str) -> Option<&'static T>;
}

impl<T: Descriptor + ?Sized> Lookup<T> for [&'static T] {
    fn by_name(&self, name: &str) -> Option<&'static T> {
        self.iter().find(|&&d| d.describe().name == name).copied()
    }
}
