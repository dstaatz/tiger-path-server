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

use std::f64::consts::PI;
const TWO_PI: f64 = 2.0*PI;

pub fn gen_circle() -> Result<()> {
    
    // Parameters and arguments
    let file_name = get_file_name()?;
    rosrust::ros_info!("Opening file: {}", file_name);
    rosrust::ros_info!("Generating circle");

    let center = (-0.2, -0.9);
    let num_points = 100.0;
    let delta_theta = (TWO_PI - 0.1) / num_points;
    let radius = 0.9;
    let mut p = PathSaver::new(file_name, "map")?;

    let mut theta = PI / 2.0;

    for i in 0..(num_points as usize) {
        theta = add_theta(theta, delta_theta);
        let x = radius*f64::cos(theta);
        let y = radius*f64::sin(theta);
        p.add_point_simple((x + center.0, y + center.1));
        if i > 10 && i-10 > (num_points as usize) / 2 && theta > PI / 2.0 {
            break;
        }
    }

    Ok(())
}

fn add_theta(t1: f64, t2: f64) -> f64 {
    let mut ans = t1 + t2;
    while ans > PI { ans = ans - TWO_PI; }
    while ans < -PI { ans = ans + TWO_PI; }
    ans
}

/// Calculate the euclidean distance between two arbitrary points
fn distance(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    f64::sqrt((p2.0 - p1.0).powi(2) + (p2.1 - p1.1).powi(2))
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

