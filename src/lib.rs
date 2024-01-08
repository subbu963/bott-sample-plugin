use std::str::from_utf8;
use extism_pdk::{plugin_fn, HttpRequest, FnResult, http};

#[plugin_fn]
pub fn get_prompt(default_prompt: String) -> FnResult<String> {
    let req = HttpRequest::new("https://wttr.in/?format=3").with_method("GET");

    let res = http::request::<String>(&req, None).unwrap();
    let body = res.body();
    let body = from_utf8(&body).unwrap();

    Ok(body.to_string())
}