/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


use std::fs::File;

use serde_json;

use rosrust_msg::geometry_msgs::{PointStamped, Quaternion, Pose, PoseStamped};
use rosrust_msg::nav_msgs::Path;

use crate::errors::*;
use crate::serde::PathSerializer;


#[derive(Debug)]
pub struct PathSaver {
    path: Path,
    file_name: String,
}


impl PathSaver {

    pub fn new(file_name: String, fixed_frame_id: &str) -> Result<Self> {

        let _ = File::create(&file_name)?;
        let mut s = Self {
            path: Path::default(),
            file_name,
        };

        s.path.header.frame_id = fixed_frame_id.to_string();

        // Add starting point of (0,0)
        s.add_point_stamped(PointStamped::default());

        Ok(s)
    }

    pub fn save(&self) -> Result<()> {
        let file = File::create(&self.file_name)?;
        serde_json::to_writer_pretty(file, &PathSerializer(&self.path))?;
        Ok(())
    }

    pub fn add_point_stamped(&mut self, p: PointStamped) {

        // Store point in a Pose
        let p = PoseStamped {
            header: p.header,
            pose: Pose {
                position: p.point,
                orientation: Quaternion::default(),
            },
        };

        self.path.poses.push(p)
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }
}


impl Drop for PathSaver {
    fn drop(&mut self) {
        // Add end point as 0,0
        self.add_point_stamped(PointStamped::default());

        rosrust::ros_info!("Saving to file: {}", self.file_name);
        self.save().unwrap()
    }
}

