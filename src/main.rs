use comrak::{markdown_to_html, ComrakOptions};
use std::fs;
use warp::Filter;

#[tokio::main]
async fn main() {
    // println!("{}", parse_markdown("./markdown/stock.markdown"));

    // let blog_index = warp::path("blog").map(|| parse_markdown("./markdown/stock.markdown"));
    let blog_index =
        warp::path("blog").map(|| warp::reply::html(parse_markdown("./markdown/stock.markdown")));

    warp::serve(blog_index).run(([127, 0, 0, 1], 3030)).await;
}

fn parse_markdown(filename: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.autolink = true;
    options.extension.table = true;
    options.extension.description_lists = true;
    options.extension.superscript = true;
    options.extension.strikethrough = true;
    options.extension.footnotes = true;
    options.render.unsafe_ = true;

    markdown_to_html(&fs::read_to_string(filename).unwrap(), &options)
}
