/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


use std::fs::File;

use serde_json;

use rosrust_msg::nav_msgs::Path;

use crate::errors::*;
use crate::serde::PathDeserializer;


#[derive(Debug)]
pub struct PathServer {
    path: Path,
}


impl PathServer {

    pub fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let path: Path = serde_json::from_reader(file).map(|PathDeserializer(path)| path)?;
        Ok(Self { path })
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

