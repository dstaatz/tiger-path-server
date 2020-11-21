/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


use std::fs::File;

use serde_json;

use rosrust_msg::std_msgs::Header;
use rosrust_msg::geometry_msgs::{Point, PointStamped, Quaternion, Pose, PoseStamped};
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
        let mut p = PointStamped::default();
        p.header.frame_id = fixed_frame_id.to_string();
        s.add_point_stamped(p);

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

    pub fn add_point_simple(&mut self, p: (f64, f64)) {

        let mut h = Header::default();
        h.frame_id = self.path.header.frame_id.clone();

        // Store point in a Pose
        let p = PoseStamped {
            header: h,
            pose: Pose {
                position: Point {
                    x: p.0,
                    y: p.1,
                    z: 0.0,
                },
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
        rosrust::ros_info!("Saving to file: {}", self.file_name);
        self.save().unwrap()
    }
}

