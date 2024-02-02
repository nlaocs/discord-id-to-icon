use std::io;
use std::io::Write;
use ureq;
use dotenv::dotenv;
use serde::Deserialize;
use serde_json::Value;
use base64::{engine::general_purpose, Engine};
use chrono::{Utc, NaiveDateTime, TimeZone};

#[derive(Deserialize)]
struct UserInfo {
    id: String,
    username: String,
    avatar: Option<String>,
    discriminator: String,
    public_flags: u32,
    premium_type: u32,
    flags: u32,
    bot: Option<bool>,
    banner: Option<String>,
    accent_color: Option<i32>,
    global_name: Option<String>,
    avatar_decoration_data: Option<Value>,
    banner_color: Option<String>,
}

fn convert_timestamp(id_str: &str) -> NaiveDateTime {
    let id: i64 = id_str.parse().unwrap();
    let epoch: i64 = 1420070400000;
    let timestamp = ((id >> 22) + epoch) / 1000;
    let datetime = Utc.timestamp_opt(timestamp, 0).unwrap().naive_utc();
    datetime
}

fn get_id() -> String {
    print!("IDを入力: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!();
    input.trim().to_string()
}

fn get_token(id: &str ) -> String {
    general_purpose::STANDARD.encode(id).replace("=", "")
}

fn get_link(id: &str, image_id: &str, image_type: &str) -> Result<String, bool> {
    let gif_url = format!("https://cdn.discordapp.com/{}/{}/{}.gif?size=4096", image_type, id, image_id);
    let png_url = format!("https://cdn.discordapp.com/{}/{}/{}.png?size=4096", image_type, id, image_id);

    let gif_resp = ureq::get(&gif_url).call();
    if gif_resp.is_ok() {
        return Ok(gif_url);
    } else {
        let png_resp = ureq::get(&png_url).call();
        if png_resp.is_ok() {
            return Ok(png_url);
        } else {
            return Err(false);
        }
    }
}

fn nitro_type(nitrotype: u32) -> String {
    match nitrotype {
        0 => "false".to_string(),
        1 => "Nitro Classic".to_string(),
        2 => "Nitro".to_string(),
        3 => "Nitro Basic".to_string(),
        _ => "null".to_string(),
    }
}

fn check_flags(user_flags: &u32) -> Vec<String> {
    let flags = [
        ("Discord_Employee", 1),
        ("Partnered_Server_Owner", 2),
        ("HypeSquad_Events", 4),
        ("Bug_Hunter_Level_1", 8),
        ("HypeSquad_Bravery", 64),
        ("HypeSquad_Brilliance", 128),
        ("HypeSquad_Balance", 256),
        ("PremiumEarlySupporter", 512),
        ("TeamPseudoUser", 1024),
        ("BugHunterLevel2", 16384),
        ("VerifiedBot", 65536),
        ("VerifiedDeveloper", 131072),
        ("DiscriminatorZero", 1048576),
        ("BotHTTPInteractions", 524288),
        ("ActiveDeveloper", 4194304),
    ];

    let mut user_badges = Vec::new();

    for &(flag_name, flag_value) in flags.iter() {
        if user_flags & flag_value == flag_value {
            user_badges.push(flag_name.to_string());
        }
    }

    user_badges
}

fn old_name(username: &str, number: &str) -> String {
    if number == "0" {
        "null".to_string()
    } else {
        format!("{}#{}", username, number)
    }
}

fn check_token(token: &str) -> bool {
    let url = "https://discordapp.com/api/v9/users/@me";
    let resp = ureq::get(url)
        .set("authorization", &format!("Bot {}", token))
        .call();
    resp.is_ok()
}

fn get_info(token: &str) {
    let id = get_id();
    let url = format!("https://discord.com/api/v10/users/{}", id);
    let resp = ureq::get(&url)
        .set("authorization", &format!("Bot {}", token))
        .set("content-type", "application/json")
        .call();
    if let Ok(response) = resp {
        let response_text = response.into_string().unwrap();
        let info: UserInfo = serde_json::from_str(&response_text).expect("エラーーー");
        let id = &info.id;
        let username = &info.username;
        let global_name = info.global_name.unwrap_or("null".to_string());
        let old_name = old_name(&info.username, &info.discriminator);
        let avatar_link = match get_link(&info.id, &info.avatar.unwrap_or_else(|| "".to_string()), "avatars") {
            Ok(url) => url,
            Err(_) => "null".to_string(),
        };
        let discriminator = info.discriminator;
        let public_flags = info.public_flags;
        let premium_type = nitro_type(info.premium_type);
        let flags = info.flags;
        let badges = check_flags(&flags);
        let bot = info.bot.unwrap_or(false);
        let banner_link = match get_link(&info.id, &info.banner.unwrap_or_else(|| "".to_string()), "banners") {
            Ok(url) => url,
            Err(_) => "null".to_string(),
        };
        let accent_color = info.accent_color.map_or("null".to_string(), |color| color.to_string());
        let avatar_decoration_data = info.avatar_decoration_data.unwrap_or_else(|| serde_json::json!(null));
        let banner_color = info.banner_color.unwrap_or("null".to_string());
        let token = format!("{}.****.*********", get_token(&info.id));
        let created_account_utc = convert_timestamp(&info.id);
        let created_account_jst = created_account_utc + chrono::Duration::hours(9);
        println!("ID: {}", id);
        println!("Username: {}", username);
        println!("Global Name: {}", global_name);
        println!("Old Name: {}", old_name);
        println!("AvatarLink: {}", avatar_link);
        println!("Discriminator: {}", discriminator);
        println!("Public Flags: {}", public_flags);
        println!("Nitro Type: {}", premium_type);
        println!("Badge Flags: {}", flags);
        for badges in badges.iter() {
            println!("Badge: {}", badges);
        }
        println!("Bot: {}", bot);
        println!("Banner Link: {}", banner_link);
        println!("Accent Color: {}", accent_color);
        println!("Avatar Decoration Data: {}", avatar_decoration_data);
        println!("Banner Color: {}", banner_color);
        println!("Token: {}", token);
        println!("Created Account(UTC): {}", created_account_utc);
        println!("Created Account(JST): {}", created_account_jst);
    } else {
        println!("IDが正しくありません");
        eprintln!("Error: {:?}", resp.unwrap_err());
    }
}

fn main() {
    dotenv().ok();
    let token = std::env::var("DISCORD_BOT_TOKEN").expect("DISCORD_BOT_TOKENが設定されていません");
    if !check_token(&token) {
        eprintln!("トークンが正しくありません");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        std::process::exit(1);
    }
    loop {
        get_info(&token);
        println!();
    }
}