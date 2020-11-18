/* Copyright (C) 2020 Dylan Staatz - All Rights Reserved. */


// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

mod errors;


////////////////////////////////////////////////////////////////////////////////


use errors::*;


pub fn run() -> Result<()> {
    unimplemented!();
}

