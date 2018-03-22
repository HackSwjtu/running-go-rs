#![feature(try_trait)]
#![feature(nll)]
#![feature(slice_concat_ext)]
#![feature(crate_in_paths)]
#![feature(match_default_bindings)]
#![allow(non_snake_case)]

extern crate base64;
extern crate itertools;
#[macro_use]
extern crate json;
#[macro_use]
extern crate maplit;
extern crate md5;
// #[macro_use]
// extern crate p_macro;
extern crate rand;
extern crate reqwest;
extern crate retry;
extern crate term;
extern crate uuid;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_ini;

mod api;
mod error;
mod constant;
mod config;
mod utils;
mod entities;
mod print;

use uuid::Uuid;
use retry::Retry;
use clap::App;

use crate::config::Config;
use crate::entities::*;
use crate::utils::*;
use crate::error::Error;
use crate::api::Api;
use crate::print::Print;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    
    if let Some(matches) = matches.subcommand_matches("run") {
        let config_file = matches.value_of("config").unwrap();
        let time = matches.value_of("time").unwrap();
            println!("run config {}{}", config_file,time);
    } else if let Some(matches) = matches.subcommand_matches("new") {
        let output = matches.value_of("output").unwrap();
        let username = matches.value_of("username").unwrap();
        let password = matches.value_of("password").unwrap();
            println!("run config {}{}{}", output,username,password);
    } else {
        eprintln!("Invalide arguments. Try <running-go --help> command for detail.")
    }
    
    let mut print = Print::new();

    // match run(&mut print) {
    //     Ok(()) => print.info("Success!"),
    //     Err(err) => print.error(&format!("Error occured: {:?}", err)),
    // }
}

fn run(start_time: u64, config: &Config,  print: &mut Print) -> Result<(), Error> {
    let user = User {
        username: config.username,
        password: config.password,
        ..Default::default()
    };

    let start_pos = GeoPoint {
        lat: config.start_pos_lat,
        lon: config.start_pos_lon,
    };

    let start_pos = start_pos.offset(Vector::ORIGIN.fuzz(300.0));
    let flag = start_time - rand_near(30 * 60 * 1000, 5 * 60 * 1000) as u64;
    let uuid = Uuid::new_v4().hyphenated().to_string();

    let mut api = Api::new(device, user);

    print.info("Start hacking");
    print.process("Login");

    api.login()?;

    print.process("Fetch point");

    let five_points = api.fetch_points(start_pos, config.min_distance_meter)?;

    print.process("Plan route");

    let route_plan = api.plan_route(start_pos, &five_points, config.min_distance_meter, &API_KEY_BAIDU)?;

    let captcha_result = Retry::new(
        &mut || {
            print.process("Get captcha");
            let captcha = api.start_validate(&uuid)?;

            print.process("Hack captcha");
            let res = api.anti_test(&captcha, &API_KEY_CAPTCHA);

            if res.is_err() {
                print.error("Captcha wrong");
            }

            res
        },
        &mut |res| res.is_ok(),
    ).try(5)
        .wait(500)
        .execute()
        .map_err(|_| Error::Api("Captcha retried too many time".into()))??;

    print.process("Validate captcha");

    api.post_validate(&uuid, &captcha_result)?;

    let record = RunRecord::plan(
        flag,
        &uuid,
        config.min_distance_meter,
        &route_plan,
        &five_points,
        start_time,
    );

    print.process("Upload record");

    api.post_record(&record)?;

    print.process("Logout");

    api.logout()?;

    Ok(())
}
