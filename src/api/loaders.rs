use gdk_pixbuf::traits::PixbufLoaderExt;
use gdk_pixbuf::{Pixbuf, PixbufLoader};
use std::io::{Error, ErrorKind, Write};
use std::thread;
use tokio::sync::oneshot;

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

    pub async fn from_url(&self, url: String) -> Option<Pixbuf> {
        let (sender, receiver) = oneshot::channel();

        let pixbuf_loader = PixbufLoader::new();
        let mut loader = LocalPixbufLoader(&pixbuf_loader);

        thread::spawn(move || {
            let client = base::get_curse_forge_client().unwrap();
            let response = client.get(url).send();
            let bytes = response.unwrap().bytes().ok().unwrap();
            sender.send(bytes).unwrap();
        });

        let bytes = receiver.await.ok()?;
        loader.write_all(&bytes).ok()?;

        pixbuf_loader.close().ok()?;
        pixbuf_loader.pixbuf()
    }
}
