use std::time::Instant;
use tl::Node::Tag;

#[derive(Debug)]
pub struct SearchResult {
    title: String,
    url: String,
    snippet: String,
}

#[async_std::main]
async fn main() {
    let now = Instant::now();
    let url = format!("https://www.bing.com/search?q={}", "async+std");
    let request = reqwest::get(url).await.unwrap().text().await.unwrap();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    let dom = tl::parse(&request, tl::ParserOptions::default()).unwrap();
    let parser = dom.parser();
    let mut results = Vec::new();
    for node in dom.nodes() {
        if let Tag(node) = node {
            if let Some(class) =  node.attributes().get("class").flatten() {
                if class.try_as_utf8_str().unwrap().contains("b_algo") {
                    let mut title = String::new();
                    let mut url = String::new();
                    let mut snippet = String::new();
                    if class.try_as_utf8_str().unwrap().contains("b_title") {
                        title = node.raw().try_as_utf8_str().unwrap().to_string();
                    }
                    for child in node.children().all(parser) {
                        if let Tag(child) = child {
                            if let Some(class) = child.attributes().get("class").flatten() {
                                if class.try_as_utf8_str().unwrap().contains("b_lineclamp") {
                                    snippet = child.inner_text(parser).to_string();
                                }
                            } else if child.name().try_as_utf8_str().unwrap() == "h2" {
                                title = child.inner_text(parser).to_string();
                                if let Some(url1) = child.children().all(parser)[0].as_tag().unwrap().attributes().get("href").flatten() {
                                    url = url1.try_as_utf8_str().unwrap().to_string();
                                }
                            } 
                        } 
                    }
                    results.push(SearchResult {
                        title,
                        url,
                        snippet,
                    });
                }
            }
        }
    }
    println!("{:#?}", results);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
