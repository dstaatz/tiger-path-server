# tiger-path-server

This package supports 3 different utilities for the emerald-tiger project.

## saver
This utility is an ROS node that lets you create manual paths based on clicking points in rviz. The path is saved to the given file on shutdown. Run with the following command:
```
cargo run --bin=saver -- path-to-path-file.json
```

## sever
This utility is an ROS node that serves a JSON path file on the /path topic and the /static_path service. The path is loaded from the given file on startup. Run with the following command:
```
cargo run --bin=server -- path-to-path-file.json
```

## gen
The utility is just a allows the user to create a perfect circular path. The parameters of the circle are hard coded but can be changed. The path is saved on the given file. Run with the following command:
```
cargo run --bin=gen -- path-to-path-file.json
```
