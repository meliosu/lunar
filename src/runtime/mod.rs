#![allow(unused)]

use std::{thread, time::Duration};

use api::Block;
use imp::RUNTIME;

mod api;
mod id;
mod imp;
mod storage;
mod threadpool;

pub fn launch(main: String, libs: Vec<String>) -> anyhow::Result<()> {
    let mut libraries = Vec::new();

    for lib in libs {
        let library = unsafe { libloading::Library::new(format!("lib{lib}.so"))? };
        libraries.push(library);
    }

    let main = unsafe { libloading::Library::new(main)? };

    let entry = unsafe { main.get::<Block>(b"entry").unwrap() };

    RUNTIME.spawn(unsafe { *entry }, 0);

    thread::sleep(Duration::from_secs(1) * 5);

    Ok(())
}
