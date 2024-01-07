// Inspired by https://github.com/xou816/spot/blob/aa1c57842c3f4e424eb62c69365438f904b2dc63/src/app/loader.rs
// Without the caching.
//
use bytes::Bytes;
use gdk_pixbuf::traits::PixbufLoaderExt;
use gdk_pixbuf::{Pixbuf, PixbufLoader};
use std::io::{Error, ErrorKind, Write};

use crate::api::*;

// A wrapper to be able to implement the Write trait on a PixbufLoader
struct LocalPixbufLoader<'a>(&'a PixbufLoader);

impl<'a> Write for LocalPixbufLoader<'a> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.0
            .write(buf)
            .map_err(|e| Error::new(ErrorKind::Other, format!("glib error: {e}")))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.0
            .close()
            .map_err(|e| Error::new(ErrorKind::Other, format!("glib error: {e}")))?;
        Ok(())
    }
}

pub struct ImageLoader {}

impl ImageLoader {
    pub fn new() -> Self {
        Self {}
    }

    fn get_image(url: &str) -> Option<Bytes> {
        let client = base::get_curse_forge_client().unwrap();
        let response = client.get(url).send().ok();
        response.unwrap().bytes().ok()
    }

    pub fn load_remote(&self, url: &str, width: i32, height: i32) -> Option<Pixbuf> {
        let pixbuf_loader = PixbufLoader::new();
        pixbuf_loader.set_size(width, height);
        let mut loader = LocalPixbufLoader(&pixbuf_loader);

        if let Some(mut resp) = Self::get_image(url) {
            loader.write_all(&resp).ok()?;
        }

        pixbuf_loader.close().ok()?;
        pixbuf_loader.pixbuf()
    }
}
