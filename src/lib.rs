/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors;
mod serde;
mod path;


////////////////////////////////////////////////////////////////////////////////


use std::env;
use std::sync::{Arc, Mutex};

use rosrust_msg::geometry_msgs::PointStamped;

use errors::*;
use path::{PathSaver, PathServer};


pub fn run_saver() -> Result<()> {

    let file_name = get_file_name()?;
    
    let saver = PathSaver::new(file_name, "map")?;
    let saver = Arc::new(Mutex::new(saver));

    let _s = rosrust::subscribe(
        "/clicked_point",
        100,
        move |p: PointStamped| {
            rosrust::ros_info!("New point: ({}, {})", p.point.x, p.point.y);
            saver.lock().unwrap().add_point_stamped(p)
        }
    );

    rosrust::spin();

    Ok(())
}


pub fn run_server() -> Result<()> {

    // Setup
    let file_name = get_file_name()?;
    let rate = rosrust::param("~rate").unwrap().get().unwrap_or(6.0);
    let server = PathServer::load(file_name)?;
    let path_pub = rosrust::publish("/path", 100)?;

    // Loop
    let rate = rosrust::rate(rate);
    while rosrust::is_ok() {
        let path = server.get_path();
        path_pub.send(path.clone())?;
        rate.sleep();
    }

    Ok(())
}


fn get_file_name() -> Result<String> {

    let args: Vec<String> = env::args().collect();

    if args.len() == 0 {
        return Err(ErrorKind::InvalidArguments(0).into())
    } else if args.len() != 2 {
        return Err(ErrorKind::InvalidArguments(args.len()-1).into())
    } else {
        Ok(args[1].to_string())
    }
}

