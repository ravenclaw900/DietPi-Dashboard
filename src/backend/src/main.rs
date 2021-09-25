#![warn(clippy::pedantic)]
use simple_logger::SimpleLogger;
use warp::Filter;

mod sockets;
mod systemdata;
mod terminal;
mod types;

#[tokio::main]
async fn main() {
    const DIR: include_dir::Dir = include_dir::include_dir!("public");

    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .env()
        .init()
        .unwrap();

    let build_route = warp::path("build")
        .and(warp::path::param())
        .map(|path: String| {
            warp::reply::with_header(
                DIR.get_file(format!("build/{}", path))
                    .unwrap()
                    .contents_utf8()
                    .unwrap(),
                "content-type",
                format!("text/{}", path.rsplit(".").next().unwrap()),
            )
        });

    let favicon_route = warp::path("favicon.png").map(|| {
        warp::reply::with_header(
            DIR.get_file("favicon.png").unwrap().contents(),
            "content-type",
            "image/png",
        )
    });

    let assets_route = warp::path("assets")
        .and(warp::path::param())
        .map(|path: String| {
            warp::reply::with_header(
                DIR.get_file(format!("assets/{}", path)).unwrap().contents(),
                "content-type",
                format!("image/png"),
            )
        });

    let terminal_route = warp::path!("ws" / "term")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(|socket| terminal::terminal_handler(socket)));

    let socket_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(|socket| sockets::socket_handler(socket)));

    let main_route = warp::any()
        .map(|| warp::reply::html(DIR.get_file("index.html").unwrap().contents_utf8().unwrap()));

    let page_routes = build_route
        .or(favicon_route)
        .or(assets_route)
        .or(main_route)
        .with(warp::compression::gzip());

    let socket_routes = terminal_route.or(socket_route);

    let routes = socket_routes
        .or(page_routes)
        .with(warp::log::custom(|info| {
            log::info!("Request to {}", info.path());
            log::debug!(
                "by {}, using {} {:?}, with response of HTTP code {:?}",
                info.remote_addr().unwrap().ip(),
                info.user_agent().unwrap(),
                info.version(),
                info.status()
            );
        }));

    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
