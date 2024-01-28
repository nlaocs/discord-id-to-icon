use std::io;
use std::io::Write;
use ureq;
use dotenv::dotenv;
use serde::Deserialize;
use serde_json::Value;
use base64::{engine::general_purpose, Engine};

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
        //print!("{}", response_text);
        let info: UserInfo = serde_json::from_str(&response_text).expect("エラーーー");
        println!("ID: {}", info.id);
        println!("Username: {}", info.username);
        match get_icon_link(&info.id, &info.avatar.unwrap_or_else(|| "".to_string())) {
            Ok(url) => println!("AvatarLink: {}", url),
            Err(_) => println!("AvatarLink: null"),
        }
        println!("Discriminator: {}", info.discriminator);
        println!("Public Flags: {}", info.public_flags);
        println!("Premium Type: {}", info.premium_type);
        println!("Flags: {}", info.flags);
        println!("Bot: {}", info.bot.unwrap_or(false));
        match get_banner_link(&info.id, &info.banner.unwrap_or_else(|| "".to_string())) {
            Ok(url) => println!("BannerLink: {}", url),
            Err(_) => println!("BannerLink: null"),
        }
        println!("Accent Color: {}", info.accent_color.map_or("null".to_string(), |color| color.to_string()));
        println!("Global Name: {}", info.global_name.unwrap_or("null".to_string()));
        println!("Avatar Decoration Data: {:?}", info.avatar_decoration_data);
        println!("Banner Color: {}", info.banner_color.unwrap_or("null".to_string()));
        println!("Token: {}.****.*********", get_token(&info.id));
    } else {
        eprintln!("Error: {:?}", resp.unwrap_err());
    }
}

fn main() {
    get_info();
}
