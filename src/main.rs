extern crate serde;
extern crate serde_json;

use base64::engine::general_purpose;
use base64::{Engine as _, alphabet, engine::{self}};
use clap::{Arg, Command};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::PAD);

#[derive(Debug, Serialize, Deserialize)]
struct NacosJWT {
    sub: String,
    exp: u64,
}

fn main() {
    let app =
        Command::new("nacosjwt")
            .arg(Arg::new("key").short('k').long("key").default_value(
                "SecretKey01234567890123456789012345678901234567890123456789012345678",
            ))
            .arg(
                Arg::new("user")
                    .short('u')
                    .long("user")
                    .default_value("nacos"),
            );

    let m = app.get_matches();
    if let Some(user) = m.get_one::<String>("user") {
        if let Some(key) = m.get_one::<String>("key") {

            let mut decoded_key = Vec::<u8>::new();

            CUSTOM_ENGINE.decode_vec(
                key,
                &mut decoded_key,
            ).unwrap();

            let now = SystemTime::now();
            let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
            let my_jwt: NacosJWT = NacosJWT {
                sub: user.clone(),
                exp: timestamp + 3600*12,
            };
            let token = encode(
                &Header::default(),
                &my_jwt,
                &EncodingKey::from_secret(&decoded_key),
            );

            let token = match token {
                Ok(t) => t,
                Err(_) => panic!("Failed to generate JWT"),
            };
            println!("user is {}", user);
            println!("key is {}", key);
            println!("nacos jwt is \nAuthorization: Bearer {}", token);
        };
    };
}
