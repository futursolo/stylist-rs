use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;
use example_yew_ssr::{ServerApp, ServerAppProps};
use stylist::manager::{render_static, StyleManager};
use warp::Filter;

/// A basic example
#[derive(Parser, Debug)]
struct Opt {
    /// the "dist" created by trunk directory to be served for hydration.
    #[clap(short, long)]
    dir: PathBuf,
}

async fn render(index_html_s: Arc<str>) -> String {
    let (writer, reader) = render_static();

    let body_s = yew::ServerRenderer::<ServerApp>::with_props(move || {
        let manager = StyleManager::builder()
            .writer(writer)
            .build()
            .expect("failed to create style manager.");
        ServerAppProps { manager }
    })
    .render()
    .await;

    let data = reader.read_style_data();

    let mut style_s = String::new();
    data.write_static_markup(&mut style_s)
        .expect("failed to read styles from style manager");

    index_html_s
        .replace("<!--%BODY_PLACEHOLDER%-->", &body_s)
        .replace("<!--%HEAD_PLACEHOLDER%-->", &style_s)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let opts = Opt::parse();

    let index_html_s: Arc<str> = tokio::fs::read_to_string(opts.dir.join("index.html"))
        .await
        .expect("failed to read index.html")
        .into();

    let render_f = warp::get().then(move || {
        let index_html_s = index_html_s.clone();

        async move { warp::reply::html(render(index_html_s).await) }
    });

    let routes = warp::path::end()
        .and(render_f.clone())
        .or(warp::path("index.html").and(render_f.clone()))
        .or(warp::fs::dir(opts.dir.clone()));

    println!("You can view the website at: http://localhost:8080/");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}
