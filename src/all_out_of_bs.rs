extern crate iron;

use iron::mime;
use iron::prelude::*;
use iron::status;

pub fn bs(_: &mut Request) -> IronResult<Response> {
    println!("           bullhockey");

    let html_type: mime::Mime = "text/html".parse().unwrap();
    let content: String = htmlize("hello,", "with bs");
    let resp = Response::with((html_type, status::Ok, content));
    Ok(resp)
}

fn htmlize(first_para: &str, second_para: &str) -> String {
    format!(
        "<html><head><style>p {{ color: #cc33cc; }} span {{ color: red; }}</style></head><body><p>{}</p><p><span>{}</span></p></body></html>",
        first_para,
        second_para
    )
}
