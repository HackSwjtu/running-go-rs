#![feature(try_trait)]
#![feature(nll)]
#![feature(slice_concat_ext)]
#![feature(crate_in_paths)]
#![feature(match_default_bindings)]
#![allow(non_snake_case)]

extern crate base64;
extern crate chrono;
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
extern crate uuid;

mod api;
mod error;
mod config;
mod utils;
mod entities;

use uuid::Uuid;
use retry::retry;

use crate::entities::*;
use crate::utils::*;
use crate::error::Error;
use crate::api::Api;

fn main() {
    match run() {
        Ok(()) => println!("Wow! Successful!"),
        Err(err) => eprintln!("Error occured: {:?}", err),
    }
}

fn run() -> Result<(), Error> {
    let API_KEY_CAPTCHA = "API KEY".into();
    let API_KEY_BAIDU = "API KEY".into();

    let device = Device {
        imei: "".into(),
        model: "Xiaomi MI 4LTE".into(),
        mac: "58:44:98:21:59:7".into(),
        os_version: "6.0.1".into(),
        user_agent: "Dalvik/2.1.0 (Linux; U; Android 6.0.1; MI 4LTE Build/MMB29M)".into(),
        ..Default::default()
    };

    let user = User {
        username: "username".into(),
        password: "password".into(),
        ..Default::default()
    };

    let start_pos = GeoPoint {
        lat: 23.169042,
        lon: 113.044233,
    };

    let sel_distance = 2000;
    let start_time = 1521539825299;
    let flag = start_time - rand_near(30 * 60 * 1000, 5 * 60 * 1000) as u64;
    let uuid = Uuid::new_v4().hyphenated().to_string();

    let mut api = Api::new(device, user);

    api.login()?;

    let five_points = api.fetch_points(start_pos, sel_distance)?;

    let route_plan = api.plan_route(start_pos, &five_points, sel_distance, &API_KEY_BAIDU)?;

    let captcha = api.start_validate(&uuid)?;

    let captcha_result = retry(
        5,
        500,
        || api.anti_test(&captcha, &API_KEY_CAPTCHA),
        |res| res.is_ok(),
    ).map_err(|_| Error::Api("Captcha retried too many time".into()))??;

    api.post_validate(&uuid, &captcha_result)?;

    let record = RunRecord::plan(
        flag,
        &uuid,
        sel_distance,
        &route_plan,
        &five_points,
        start_time,
    );

    api.post_record(&record)?;

    api.logout()?;

    Ok(())
}
