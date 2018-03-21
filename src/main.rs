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
extern crate term;
extern crate uuid;

mod api;
mod error;
mod config;
mod utils;
mod entities;
mod print;

use uuid::Uuid;
use retry::Retry;

use crate::entities::*;
use crate::utils::*;
use crate::error::Error;
use crate::api::Api;
use crate::print::Print;

fn main() {
    let mut print = Print::new();

    match run(&mut print) {
        Ok(()) => print.info("Successful!"),
        Err(err) => print.error(&format!("Error occured: {:?}", err)),
    }
}

fn run(print: &mut Print) -> Result<(), Error> {
    let API_KEY_CAPTCHA = "API KEY".into();
    let API_KEY_BAIDU = "API KEY".into();

    let device = Device {
        imei: "".into(),
        model: "Meizu Pro6 Plus".into(),
        mac: "38:44:94:21:29:7".into(),
        os_version: "6.0.0".into(),
        user_agent: "Dalvik/2.1.0 (Linux; U; Android 6.0.0)".into(),
        ..Default::default()
    };

    let user = User {
        username: "username".into(),
        password: "password".into(),
        ..Default::default()
    };

    let start_pos = GeoPoint {
        lat: 23.04767,
        lon: 113.380725,
    };

    let sel_distance = 2000;
    let start_time = 1521636213000;

    let start_pos = start_pos.offset(Vector::ORIGIN.fuzz(300.0));
    let flag = start_time - rand_near(30 * 60 * 1000, 5 * 60 * 1000) as u64;
    let uuid = Uuid::new_v4().hyphenated().to_string();

    let mut api = Api::new(device, user);

    print.info("Start hacking");
    print.process("Login");

    api.login()?;

    print.process("Fetch point");

    let five_points = api.fetch_points(start_pos, sel_distance)?;

    print.process("Plan route");

    let route_plan = api.plan_route(start_pos, &five_points, sel_distance, &API_KEY_BAIDU)?;

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
        sel_distance,
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
