/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


use serde::{Serialize, Deserialize};
use rosrust::Time;
use rosrust_msg::std_msgs::Header;
use rosrust_msg::geometry_msgs::{Point, Quaternion, Pose, PoseStamped};
use rosrust_msg::nav_msgs::Path;


#[derive(Serialize, Deserialize)]
#[serde(remote = "Header")]
struct HeaderDef {
    seq: u32,
    stamp: Time,
    frame_id: String,
}


#[derive(Serialize, Deserialize)]
#[serde(remote = "Point")]
struct PointDef {
    x: f64,
    y: f64,
    z: f64,
}


#[derive(Serialize, Deserialize)]
#[serde(remote = "Quaternion")]
struct QuaternionDef {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}


#[derive(Serialize, Deserialize)]
#[serde(remote = "Pose")]
struct PoseDef {
    #[serde(with = "PointDef")]
    position: Point,
    #[serde(with = "QuaternionDef")]
    orientation: Quaternion,
}


#[derive(Serialize, Deserialize)]
#[serde(remote = "PoseStamped")]
struct PoseStampedDef {
    #[serde(with = "HeaderDef")]
    header: Header,
    #[serde(with = "PoseDef")]
    pose: Pose,
}


mod posestamped_vec {

    use super::PoseStampedDef;
    use rosrust_msg::geometry_msgs::PoseStamped;
    use serde::{Serialize, Deserialize, Serializer, Deserializer};

    pub fn serialize<S>(value: &Vec<PoseStamped>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        struct Helper<'a>(#[serde(with = "PoseStampedDef")] &'a PoseStamped);

        let v: Vec<Helper> = value.into_iter().map(Helper).collect();
        v.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<PoseStamped>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Wrapper(#[serde(with = "PoseStampedDef")] PoseStamped);
    
        let v = Vec::deserialize(deserializer)?;
        Ok(v.into_iter().map(|Wrapper(a)| a).collect())
    }
}


#[derive(Serialize, Deserialize)]
#[serde(remote = "Path")]
struct PathDef {
    #[serde(with = "HeaderDef")]
    header: Header,
    #[serde(with = "posestamped_vec")]
    poses: Vec<PoseStamped>,
}


#[derive(Serialize)]
pub struct PathSerializer<'a>(#[serde(with = "PathDef")] pub &'a Path);


#[derive(Deserialize)]
pub struct PathDeserializer(#[serde(with = "PathDef")] pub Path);


#[cfg(test)]
mod tests {

    use std::fs::{self, File};

    use super::*;

    #[test]
    fn test_read_write() {

        let mut test_path = Path::default();

        test_path.poses = vec![
            PoseStamped::default(),
            PoseStamped::default(),
        ];

        let filename = "./temp1.json";
        let file = File::create(filename).unwrap();
        serde_json::to_writer_pretty(file, &PathSerializer(&test_path)).unwrap();

        let file = File::open(filename).unwrap();
        let test_path_data: Path = serde_json::from_reader(file).map(|PathDeserializer(path)| path).unwrap();

        let _file: Option<File> = None;

        fs::remove_file(filename).unwrap();
        assert_eq!(test_path, test_path_data);
    }
}

