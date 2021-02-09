use std::env;
use std::str::FromStr;

pub fn env_iss() -> String {
    env::var("JWT_ISS").unwrap_or("".to_string())
}
pub fn env_aud() -> String {
    env::var("JWT_AUD").unwrap_or("".to_string())
}

pub fn env_exp_days() -> u64 {
    return match u64::from_str(
        env::var("JWT_EXP_DAYS")
            .unwrap_or("30".to_string())
            .as_ref(),
    ) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };
}

pub fn env_nbf_days() -> u64 {
    return match u64::from_str(env::var("JWT_NBF_DAYS").unwrap_or("0".to_string()).as_ref()) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };
}

pub fn env_token_secret() -> String {
    env::var("JWT_SECRET").expect("No token secret found!")
}

pub fn env_leeway() -> i64 {
    return match i64::from_str(
        env::var("JWT_LEEWAY_SEC")
            .unwrap_or("0".to_string())
            .as_ref(),
    ) {
        Ok(v) => v,
        Err(e) => panic!(e),
    };
}
