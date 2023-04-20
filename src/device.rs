// I am essentially trying to stick as close as possible to the dwipe source structure. For now.

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path, borrow::Cow,
};

use crate::context::RwipeContext;

impl<'a> RwipeContext<'a> {
    pub fn identify(&mut self) {
        let label_file = Path::new(&self.device_name).with_extension("label");
        let file = match File::open(&label_file) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error: Unable to open '{}': {}", label_file.display(), e);
                self.label = &Cow::Owned("Uninitialized Device");
                return;
            }
        };
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        if let Ok(len) = reader.read_line(&mut buffer) {
            if len > 0 {
                self.label = &Cow::Owned(buffer.trim_end());
            }
        }
    }
}
