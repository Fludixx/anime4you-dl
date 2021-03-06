use crate::downloader::Downloader;
use anyhow::{anyhow, Error};
use regex::Regex;

/*
 * https://github.com/Fludixx/serienstream-dl/blob/master/src/downloader/vidoza.rs
 * fludixx
 */

pub fn new(url: &str) -> Result<Downloader, Error> {
    let mut request = reqwest::get(url).expect("Failed to reach vidoza.net");
    let site_source = request.text().unwrap();
    let url_regex = Regex::new(r#"(?s)sourcesCode:\s\[\{\ssrc:\s"(.+)", type"#).unwrap();
    let name_regex = Regex::new(r#"(?s)var\scurFileName\s=\s"(.*?)";"#).unwrap();
    let url_capture = url_regex.captures(&site_source);
    let name_capture = name_regex.captures(&site_source);
    if url_capture.is_none() || name_capture.is_none() {
        Err(anyhow!("Failed to retrieve sources."))?
    }
    let video_url = String::from(url_capture.unwrap().get(1).unwrap().as_str());
    let file_name = String::from(name_capture.unwrap().get(1).unwrap().as_str());
    Ok(Downloader {
        url: String::from(url),
        video_url,
        file_name,
    })
}
