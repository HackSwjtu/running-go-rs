use std::time::{SystemTime, UNIX_EPOCH};
use std::str::FromStr;
// use reqwest;
use reqwest::Client;
use reqwest::header::Headers;
use base64;
use json::{self, JsonValue};
use md5;

use crate::entities::*;
use crate::utils::*;
use crate::constant::*;
use crate::error::Error;

#[derive(Debug)]
pub struct Api {
    device: Device,
    user: User,
    client: Client,
}

impl Api {
    pub fn new(mut device: Device, user: User) -> Self {
        device.build();
        Api {
            device,
            user,
            client: Client::new(),
            // client: Client::builder()
            //     .proxy(reqwest::Proxy::https("http://127.0.0.1:8888").unwrap())
            //     .proxy(reqwest::Proxy::http("http://127.0.0.1:8888").unwrap())
            //     .build()
            //     .unwrap(),
        }
    }

    fn headers_user_agent(&mut self) -> Headers {
        let mut headers = Headers::new();
        headers.set_raw("User-Agent", self.device.user_agent.clone());
        headers
    }

    fn headers(&mut self) -> Headers {
        let mut headers = Headers::new();
        headers.set_raw("Accept", "application/json");
        headers.set_raw("User-Agent", self.device.user_agent.clone());
        headers.set_raw("Content-Type", "application/json;charset=UTF-8");
        headers.set_raw("appVersion", APP_VERSION);
        headers.set_raw("CustomDeviceId", self.device.custom_id.clone());
        headers.set_raw("DeviceId", self.device.id.clone());
        headers.set_raw("deviceName", self.device.model.clone());
        headers.set_raw("osType", OS_TYPE);
        headers.set_raw("osVersion", self.device.os_version.clone());

        if self.user.token != "" {
            let now = SystemTime::now();
            let since_the_epoch = now.duration_since(UNIX_EPOCH).unwrap();
            let time_stamp = since_the_epoch.as_secs() * 1000
                + since_the_epoch.subsec_nanos() as u64 / 1_000_000;

            let sign_param = btreemap! {
                "uid".into() => self.user.uid.to_string(),
                "token".into() => self.user.token.clone(),
                "timeStamp".into() => time_stamp.to_string(),
            };
            let sign = compute_sign(&sign_param, MD5_SIGN_SALT);

            headers.set_raw("uid", self.user.uid.to_string());
            headers.set_raw("token", self.user.token.clone());
            headers.set_raw("timeStamp", time_stamp.to_string());
            headers.set_raw("tokenSign", sign);
        }

        headers
    }

    pub fn login(&mut self) -> Result<(), Error> {
        let auth_key = base64::encode(&format!("{}:{}", self.user.username, self.user.password));
        let auth_str = format!("Basic {}", auth_key);

        let mut headers = self.headers();
        headers.set_raw("Authorization", auth_str);

        let json = object!{
            "app_version" => APP_VERSION,
            "channel" => "",
            "device_id" => self.device.id.clone(),
            "device_model" => self.device.model.clone(),
            "imei" => self.device.imei.clone(),
            "loginType" => 1,
            "mac_address" => self.device.mac.clone(),
            "os_type" => 0,
            "os_version" => self.device.os_version.clone(),
        };

        let res = self.client
            .post("https://gxapp.iydsj.com/api/v23/login")
            .headers(headers)
            .body(json.to_string())
            .send()?
            .text()?;

        let res = json::parse(&res)?;

        if res["error"] != 10000 {
            return Err(Error::Api(res["message"].as_str()?.to_string()));
        }

        let data = &res["data"];

        self.user.token = data["token"].as_str()?.to_string();
        self.user.uid = data["uid"].as_u64()?;
        self.user.unid = data["unid"].as_u64()?;
        self.user.campus_name = data["campusName"].as_str()?.to_string();

        Ok(())
    }

    pub fn fetch_points(
        &mut self,
        start_pos: GeoPoint,
        distance: u64,
    ) -> Result<Vec<FivePoint>, Error> {
        let sign_str = format!("http://gxapp.iydsj.com/api/v18/get/1/distance/{}", distance);
        let sign = format!("{:X}", md5::compute(sign_str + MD5_KEY));

        let json = object!{
            "latitude" => start_pos.lat,
            "longitude" => start_pos.lon,
            "selectedUnid" => self.user.unid,
            "sign" => sign,
        };

        let res = self.client
            .post("https://gxapp.iydsj.com/api/v18/get/1/distance/2000")
            .headers(self.headers())
            .body(json.to_string())
            .send()?
            .text()?;

        let res = validate(&res)?;

        let data = &res["data"]["pointsResModels"];

        data.members()
            .enumerate()
            .map(|(i, obj)| {
                Ok(FivePoint {
                    id: i as u64,
                    name: obj["pointName"].as_str()?.to_string(),
                    fixed: obj["isFixed"].as_u64()?,
                    pos: GeoPoint {
                        lon: obj["lon"].as_f64()?,
                        lat: obj["lat"].as_f64()?,
                    },
                })
            })
            .collect()
    }

    pub fn plan_route(
        &mut self,
        start_pos: GeoPoint,
        five_points: &Vec<FivePoint>,
        sel_distance: u64,
        apikey: &String,
    ) -> Result<RoutePlan, Error> {
        let mut route_points = vec![start_pos];

        let north_east = start_pos.offset(Vector {
            x: 10000.0,
            y: 10000.0,
        });

        let mut orig;
        let mut dest = start_pos;

        for p in five_points.iter().map(|p| p.pos) {
            orig = dest;
            dest = p;

            route_points.extend(self.baidu_get_path(orig, dest, apikey)?.iter());
        }

        let min_points = route_points.len() as u64;

        route_points.extend(self.baidu_get_path(dest, north_east, apikey)?.iter());

        Ok(RoutePlan {
            min_distance: sel_distance + rand_near(150, 50),
            min_points,
            route_points,
        })
    }

    fn baidu_get_path(
        &mut self,
        orig: GeoPoint,
        dest: GeoPoint,
        apikey: &String,
    ) -> Result<Vec<GeoPoint>, Error> {
        let res = self.client
            .get("http://api.map.baidu.com/direction/v2/riding")
            .query(&[
                ("origin", format!("{:.6},{:.6}", orig.lat, orig.lon)),
                ("destination", format!("{:.6},{:.6}", dest.lat, dest.lon)),
                ("ak", apikey.clone()),
            ])
            .send()?
            .text()?;

        let res = json::parse(&res)?;

        if res["status"] != 0 {
            return Err(Error::Api(res["message"].as_str()?.to_string()));
        }

        let mut route_points = Vec::new();

        for step in res["result"]["routes"][0]["steps"].members() {
            let path = step["path"].as_str()?;
            let points = path.split(";");

            for point in points {
                let mut lat_lon = point.split(",");
                let lon = f64::from_str(lat_lon.next()?)?;
                let lat = f64::from_str(lat_lon.next()?)?;
                route_points.push(GeoPoint { lon, lat });
            }
        }

        Ok(route_points)
    }

    pub fn start_validate(&mut self, uuid: &String) -> Result<Captcha, Error> {
        let res = self.client
            .get("https://gxapp.iydsj.com/api/v20/security/geepreprocess")
            .headers(self.headers_user_agent())
            .query(&[
                ("osType", OS_TYPE),
                ("uid", &self.user.uid.to_string()),
                ("uuid", &uuid),
            ])
            .send()?
            .text()?;

        let res = validate(&res)?;

        let data = &res["data"];

        Ok(Captcha {
            challenge: data["challenge"].as_str()?.to_string(),
            gt: data["gt"].as_str()?.to_string(),
        })
    }

    pub fn anti_test(
        &mut self,
        captcha: &Captcha,
        apikey: &String,
    ) -> Result<CaptchaResult, Error> {
        let res = self.client
            .get("http://jiyan.25531.com/api/create")
            .query(&[
                ("appkey", apikey),
                ("gt", &captcha.gt.clone()),
                ("challenge", &captcha.challenge.clone()),
                ("referer", &"".into()),
                ("model", &3.to_string()),
            ])
            .send()?
            .text()?;

        let res = json::parse(&res)?;

        if res["code"] != 10000 {
            return Err(Error::Api(res["data"].as_str()?.to_string()));
        }

        let data = &res["data"];

        Ok(CaptchaResult {
            challenge: data["challenge"].as_str()?.to_string(),
            validate: data["validate"].as_str()?.to_string(),
        })
    }

    pub fn post_validate(&mut self, uuid: &String, captcha: &CaptchaResult) -> Result<(), Error> {
        let params = hashmap! {
            "uid" => self.user.uid.to_string(),
            "osType" => OS_TYPE.to_string(),
            "uuid" => uuid.clone(),
            "geetest_challenge" => captcha.challenge.clone(),
            "geetest_seccode" => captcha.validate.clone(),
            "geetest_validate" => captcha.validate.clone(),
        };

        let res = self.client
            .post("https://gxapp.iydsj.com/api/v20/security/geevalidate")
            .headers(self.headers_user_agent())
            .form(&params)
            .send()?
            .text()?;

        validate(&res)?;

        Ok(())
    }

    pub fn post_record(&mut self, record: &RunRecord) -> Result<(), Error> {
        let data = record.to_json(self.user.uid, self.user.unid);

        let res = self.client
            .post("https://gxapp.iydsj.com/api/v22/runnings/save/record")
            .headers(self.headers())
            .body(data.to_string())
            .send()?
            .text()?;

        validate(&res)?;

        Ok(())
    }

    pub fn logout(&mut self) -> Result<(), Error> {
        let res = self.client
            .post("https://gxapp.iydsj.com/api/v6/user/logout")
            .headers(self.headers())
            .send()?
            .text()?;

        validate(&res)?;

        Ok(())
    }
}

fn validate(text: &str) -> Result<JsonValue, Error> {
    let json = json::parse(&text)?;
    if json["error"] != 10000 {
        return Err(Error::Api(json["message"].as_str()?.to_string()));
    }
    Ok(json)
}
