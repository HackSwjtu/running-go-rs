#![feature(try_trait)]
#![feature(nll)]
#![feature(slice_concat_ext)]
#![feature(crate_in_paths)]
#![feature(match_default_bindings)]
#![allow(non_snake_case)]

extern crate base64;
#[macro_use]
extern crate clap;
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
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_ini;
extern crate term;
extern crate time;
extern crate uuid;

mod api;
mod error;
mod constant;
mod config;
mod utils;
mod entities;
mod print;

use std::str::FromStr;
use uuid::Uuid;
use retry::Retry;
use clap::App;

use crate::config::Config;
use crate::entities::*;
use crate::utils::*;
use crate::constant::*;
use crate::error::Error;
use crate::api::Api;
use crate::print::Print;

fn main() {
    let mut print = Print::new();

    match parse_argument(&mut print) {
        Err(err) => print.error(&format!("Error occured: {:?}", err)),
        _ => (),
    }

    print.done_prev_process();
}

fn parse_argument(print: &mut Print) -> Result<(), Error> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand() {
        ("generate", Some(matches)) => {
            print.process("Parse argument");

            let username = matches.value_of("username")?.to_string();
            let password = matches.value_of("password")?.to_string();
            let start_pos_lat = f64::from_str(matches.value_of("lat")?)?;
            let start_pos_lon = f64::from_str(matches.value_of("lon")?)?;
            let output = matches.value_of("output")?;

            let mut config = Config {
                username,
                password,
                start_pos_lat,
                start_pos_lon,
                ..Default::default()
            };

            print.process("Generate user config");

            config.build();
            config.output(output)?;
        }
        ("run", Some(matches)) => {
            print.process("Parse argument");

            let time_template = "%Y/%m/%d %H:%M:%S";
            let start_time = time::strptime(matches.value_of("time")?, time_template)?;
            let start_timestamp = start_time.to_timespec().sec as u64 * 1000;

            let duration = time::now() - start_time;
            if duration.num_days() > 3 {
                return Err(Error::Api(
                    "Date cannot be earlier than three days ago".into(),
                ));
            }

            if duration.num_days() < 0 {
                return Err(Error::Api("Date cannot be later than now".into()));
            }

            let distance = u64::from_str(matches.value_of("distance")?)?;
            if distance < SEL_DISTANCE {
                return Err(Error::Api(format!(
                    "Distance {} may not be less than {}",
                    distance, SEL_DISTANCE
                )));
            }

            print.process("Load user config");

            let config = Config::from_path(matches.value_of("config")?)?;

            run(start_timestamp, distance, config, print)?;
        }
        _ => {
            App::from_yaml(yaml).get_matches_from(&["", "-h"]);
            // eprintln!("Invalid argument.\nTry \"running-go --help\".")},
        }
    }

    Ok(())
}

fn run(start_time: u64, distance: u64, config: Config, print: &mut Print) -> Result<(), Error> {
    let user = User {
        username: config.username,
        password: config.password,
        ..Default::default()
    };

    let device = Device {
        imei: config.device_imei,
        model: config.device_model,
        mac: config.device_mac,
        os_version: config.device_os_version,
        user_agent: config.device_user_agent,
        ..Default::default()
    };

    let start_pos = GeoPoint {
        lat: config.start_pos_lat,
        lon: config.start_pos_lon,
    };

    let start_pos = start_pos.offset(Vector::ORIGIN.fuzz(300.0));
    let flag = start_time - rand_near(30 * 60 * 1000, 5 * 60 * 1000);
    let uuid = Uuid::new_v4().hyphenated().to_string();

    let mut api = Api::new(device, user);

    print.process("Login");

    api.login()?;

    print.process("Fetch point");

    let five_points = api.fetch_points(start_pos)?;

    print.process("Plan route");

    let route_plan = api.plan_route(start_pos, distance, &five_points, API_KEY_BAIDU)?;

    let captcha_result = Retry::new(
        &mut || {
            print.process("Get captcha");
            let captcha = api.start_validate(&uuid)?;

            print.process("Hack captcha");
            let res = api.anti_test(&captcha, API_KEY_CAPTCHA);

            if res.is_err() {
                print.error("Captcha wrong");
            }

            res
        },
        &mut |res| res.is_ok(),
    ).try(5)
        .wait(500)
        .execute()
        .map_err(|_| Error::Api("Retried captcha too many times".into()))??;

    print.process("Validate captcha");

    api.post_validate(&uuid, &captcha_result)?;

    let record = RunRecord::plan(flag, &uuid, &route_plan, &five_points, start_time);

    print.process("Upload record");

    api.post_record(&record)?;

    print.process("Logout");

    api.logout()?;

    Ok(())
}
