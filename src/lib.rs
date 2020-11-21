/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors;
mod serde;
mod saver;
mod server;


////////////////////////////////////////////////////////////////////////////////


use std::env;
use std::sync::{Arc, Mutex};

use rosrust_msg::geometry_msgs::PointStamped;
use rosrust_msg::nav_msgs::{GetPlan, GetPlanRes};

use errors::*;
use saver::PathSaver;
use server::PathServer;


pub fn run_saver() -> Result<()> {

    // Parameters and arguments
    let file_name = get_file_name()?;
    let rate = rosrust::param("~rate").unwrap().get().unwrap_or(6.0);
    rosrust::ros_info!("File to save to: {}", file_name);

    // Setup
    let saver0 = PathSaver::new(file_name, "map")?;
    let saver0 = Arc::new(Mutex::new(saver0));
    let saver1 = saver0.clone();

    let path_pub = rosrust::publish("/path", 100)?;

    let _s = rosrust::subscribe(
        "/clicked_point",
        100,
        move |p: PointStamped| {
            rosrust::ros_info!("New point: ({}, {})", p.point.x, p.point.y);
            saver0.lock().unwrap().add_point_stamped(p)
        }
    );

    rosrust::ros_info!("Listening for points on /clicked_point");
    rosrust::ros_info!("Publishing path on /path");

    // Loop
    let rate = rosrust::rate(rate);
    while rosrust::is_ok() {
        let saver = saver1.lock().unwrap();
        let path = saver.get_path();
        path_pub.send(path.clone())?;
        rate.sleep();
    }

    Ok(())
}


pub fn run_server() -> Result<()> {

    // Parameters and arguments
    let file_name = get_file_name()?;
    let rate = rosrust::param("~rate").unwrap().get().unwrap_or(6.0);
    rosrust::ros_info!("Opening file: {}", file_name);

    // Setup
    let server0 = PathServer::load(file_name)?;
    let server0 = Arc::new(Mutex::new(server0));
    let server1 = server0.clone();
    let path_pub = rosrust::publish("/path", 100)?;

    rosrust::ros_info!("Hosting /static_path service");

    let _service = rosrust::service::<GetPlan, _>(
        "static_path",
        move |_| {
            rosrust::ros_info!("Returning path on static_path service");
            let path = server0.lock().unwrap().get_path().clone();
            Ok(GetPlanRes { plan: path })
        }
    )?;

    rosrust::ros_info!("Publishing path on /path");

    // Loop
    let rate = rosrust::rate(rate);
    while rosrust::is_ok() {
        let server = server1.lock().unwrap();
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

