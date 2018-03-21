#[derive(Debug)]
pub struct Captcha {
    pub challenge: String,
    pub gt: String,
}

#[derive(Debug)]
pub struct CaptchaResult {
    pub challenge: String,
    pub validate: String,
}
