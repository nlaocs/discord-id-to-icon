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
    return input.trim().to_string();
}

fn get_token(id: &str ) -> String {
    return general_purpose::STANDARD.encode(id).replace("=", "")
}

fn get_icon_link(id: &str ,avatar_id: &str) -> Result<String, bool> {
    let gif_url = format!("https://cdn.discordapp.com/avatars/{}/{}.gif?size=4096", id, avatar_id);
    let png_url = format!("https://cdn.discordapp.com/avatars/{}/{}.png?size=4096", id, avatar_id);

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

fn get_banner_link(id: &str, banner_id: &str) -> Result<String, bool> {
    let gif_url = format!("https://cdn.discordapp.com/banners/{}/{}.gif?size=4096", id, banner_id);
    let png_url = format!("https://cdn.discordapp.com/banners/{}/{}.png?size=4096", id, banner_id);

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

fn get_info(){
    dotenv().ok();
    let token = std::env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
    let id = get_id();
    let url = format!("https://discordapp.com/api/users/{}", id);
    let resp = ureq::get(&url)
        .set("authorization", &format!("Bot {}", token))
        .set("content-type", "application/json")
        .call();
    if let Ok(response) = resp {
        let response_text = response.into_string().unwrap();
        let info: UserInfo = serde_json::from_str(&response_text).expect("エラーーー");
        let id = &info.id;
        let username = info.username;
        let avatar_link = match get_icon_link(&info.id, &info.avatar.unwrap_or_else(|| "".to_string())) {
            Ok(url) => url,
            Err(_) => "null".to_string(),
        };
        let discriminator = info.discriminator;
        let public_flags = info.public_flags;
        let premium_type = info.premium_type;
        let flags = info.flags;
        let bot = info.bot.unwrap_or(false);
        let banner_link = match get_banner_link(&info.id, &info.banner.unwrap_or_else(|| "".to_string())) {
            Ok(url) => url,
            Err(_) => "null".to_string(),
        };
        let accent_color = info.accent_color.map_or("null".to_string(), |color| color.to_string());
        let global_name = info.global_name.unwrap_or("null".to_string());
        let avatar_decoration_data = info.avatar_decoration_data.unwrap_or_else(|| serde_json::json!(null));
        let banner_color = info.banner_color.unwrap_or("null".to_string());
        let token = format!("{}.****.*********", get_token(&info.id));
        let created_account_utc = convert_timestamp(&info.id);
        let created_account_jst = created_account_utc + chrono::Duration::hours(9);
        println!("ID: {}", id);
        println!("Username: {}", username);
        println!("AvatarLink: {}", avatar_link);
        println!("Discriminator: {}", discriminator);
        println!("Public Flags: {}", public_flags);
        println!("Premium Type: {}", premium_type);
        println!("Flags: {}", flags);
        println!("Bot: {}", bot);
        println!("Banner Link: {}", banner_link);
        println!("Accent Color: {}", accent_color);
        println!("Global Name: {}", global_name);
        println!("Avatar Decoration Data: {}", avatar_decoration_data);
        println!("Banner Color: {}", banner_color);
        println!("Token: {}.****.*********", token);
        println!("Created Account(UTC): {}", created_account_utc);
        println!("Created Account(JST): {}", created_account_jst);
    } else {
        println!("IDが正しくありません");
        eprintln!("Error: {:?}", resp.unwrap_err());
    }
}

fn main() {
    get_info();
}
