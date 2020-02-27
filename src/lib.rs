//! Proof of concept Arc with IO trait delegation
//!
//! # Examples
//!
//! ```no_run
//! use std::net::TcpStream;
//! use io_arc::IoArc;
//! use std::io::{self, prelude::*};
//!
//! fn main() -> io::Result<()> {
//!     let stream = TcpStream::connect("localhost:8080")?;
//!     let stream = IoArc::new(stream);
//!
//!     let mut stream1 = stream.clone();
//!     let mut _stream2 = stream.clone();
//!
//!     stream1.write(b"hello world")?; // Write is implemented for Arc<TcpStream> directly
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, missing_doc_code_examples, unreachable_pub)]

use futures_io::{AsyncRead, AsyncWrite};
use std::io::{self, prelude::*};
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use std::borrow::Borrow;

/// A variant of `Arc` that delegates IO traits if available on `&T`.
#[derive(Debug)]
pub struct IoArc<T>(Arc<T>);

impl<T> IoArc<T> {
    /// Create a new instance of IoArc.
    pub fn new(data: T) -> Self {
        Self(Arc::new(data))
    }
}

impl<T> Read for IoArc<T>
where
    for<'a> &'a T: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (&mut &*self.0).read(buf)
    }
}

// impl<T> BufRead for IoArc<T>
// where
//     for<'a> &'a T: BufRead,
// {
//     fn consume(&mut self, amt: usize) {
//         (&mut &*self.0).consume(amt)
//     }

//     fn fill_buf(&mut self) -> io::Result<&[u8]> {
//         (&*self.0).fill_buf()
//     }
// }

impl<T> Write for IoArc<T>
where
    for<'a> &'a T: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        (&mut &*self.0).write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        (&mut &*self.0).flush()
    }
}

impl<T> AsyncRead for IoArc<T>
where
    for<'a> &'a T: AsyncRead,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self.0).poll_read(cx, buf)
    }
}

impl<T> AsyncWrite for IoArc<T>
where
    for<'a> &'a T: AsyncWrite,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut &*self.0).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self.0).poll_flush(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut &*self.0).poll_close(cx)
    }
}

// impl<T> futures_io::AsyncBufRead for IoArc<T>
// where
//     for<'a> &'a T: AsyncBufRead,
// {
//     fn poll_fill_buf(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<&[u8]>> {
//         // Pin::new(&mut &*self.0).poll_fill_buf(cx)
//         todo!();
//     }
//     fn consume(self: Pin<&mut Self>, amt: usize) {
//         AsyncBufRead::consume(Pin::new(&mut &*self.0), amt)
//     }
// }

impl<T: Default> Default for IoArc<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T> From<T> for IoArc<T> {
    fn from(t: T) -> Self {
        Self::new(t)
    }
}

impl<T> Borrow<T> for IoArc<T> {
    fn borrow(&self) -> &T {
        self.0.borrow()
    }
}

impl<T> AsRef<T> for IoArc<T> {
    fn as_ref(&self) -> &T {
        self.0.as_ref()
    }
}

impl<T> Unpin for IoArc<T> {}

impl<T> Clone for IoArc<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
