extern crate iron;
extern crate rand;

use iron::mime;
use iron::prelude::*;
use iron::status;

pub fn bs(_: &mut Request) -> IronResult<Response> {
    println!("           bullhockey");

    let html_type: mime::Mime = "text/html".parse().unwrap();
    let content: String = html_with_two_paragraphs("this page", "just nonsense");
    let resp = Response::with((html_type, status::Ok, content));
    Ok(resp)
}

fn html_with_two_paragraphs(first_para: &str, second_para: &str) -> String {
    let (background, foreground) = background_and_foreground();
    format!(
        "<html><head><style>html * {{ background-color: {}; color: {}; padding: 8% 20%; }} span {{ background-color: {}; color: {}; }}</style></head><body><h1>{}</h1><h2>is <span>{}</span></h2></body></html>",
        background,
        foreground,
        foreground,
        background,
        first_para,
        second_para
    )
}

fn random_color() -> String {
    format!("{:x}{:x}", rand::random::<u8>() / 16, rand::random::<u8>() / 16)
}

fn oppositish(color: String) -> String {
    if color.as_str() >= "80" { String::from("00") } else { String::from("ff") }
}

fn background_and_foreground() -> (String, String) {
    let mut background = String::from("#");
    let mut foreground = String::from("#");
    for _i in 1..=3 {
        let color = random_color();
        background.push_str(&color);
        foreground.push_str(&oppositish(color));
    }
    (background, foreground)
}
