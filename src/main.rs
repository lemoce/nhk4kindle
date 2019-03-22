extern crate reqwest;
extern crate regex;
extern crate serde_json;


use std::u8;
use std::u16;

use regex::{Regex, Captures};

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
struct News {
    news_priority_number: String,
    news_prearranged_time: String,
    news_id: String,
    title: String,
    title_with_ruby: String,
    news_file_ver: bool,
    news_creation_time: String,
    news_preview_time: String,
    news_publication_time: String,
    news_publication_status: bool,
    has_news_web_image: bool,
    has_news_web_movie: bool,
    has_news_easy_image: bool,
    has_news_easy_movie: bool,
    has_news_easy_voice: bool,
    news_web_image_uri: String,
    news_web_movie_uri: String,
    news_easy_image_uri: String,
    news_easy_movie_uri: String,
    news_easy_voice_uri: String,
    news_display_flag: bool,
    news_web_url: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DiaryNews {
    date: String,
    news: Vec<News>,
}


fn html_unicode_converter<'a>(my_str: &'a str) -> String {
    let re = Regex::new(r"(\\u(..)(..))").unwrap();
    let result_content = re.replace_all(my_str, |caps: &Captures| {
        let mut utf16_ch = [0_u16; 1];
        utf16_ch[0] = (u8::from_str_radix(&caps[2], 16).unwrap() as u16) << 8;
        utf16_ch[0] += u8::from_str_radix(&caps[3], 16).unwrap() as u16;
        format!("{}", String::from_utf16(&utf16_ch).unwrap())
    });

    String::from(result_content)
}

fn get_response_body() -> Result<String, reqwest::Error> {
    let content: String = reqwest::get("https://www3.nhk.or.jp/news/easy/news-list.json")?.text()?;
    Ok(html_unicode_converter(&content))
}

fn main() {
    //let news_list: Vec<DiaryNews> = serde_json::from_str(&get_response_body().unwrap()).unwrap();
    //println!("{:?}", news_list);
    println!("{}", get_response_body().unwrap());
    
    let teste = String::from("japanese: 日本語のメサージ");
    let teste_u16: Vec<u16> = teste.encode_utf16().collect();
    println!("{:x?}", teste_u16);
    println!("{:x?}", String::from_utf16(teste_u16.as_slice()).unwrap().as_bytes());

    
}
