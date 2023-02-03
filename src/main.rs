mod api;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Start { port: Option<u32> },
    Start(StartArgs),
    Stop,
    Status,
}


#[derive(Parser, Debug)]
struct StartArgs {
    #[arg(name = "hostname", long, default_value = "127.0.0.1")]
    host: String,
    #[arg(short, long, default_value_t = 9999)]
    port: u16,
}


use axum::{Router, routing, response};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Start(args) => {
            let addr = format!("{}:{}", args.host, args.port);
            tracing::debug!("listening on {}", addr);

            let api_router = create_api_route();

            let addr = addr.parse().expect("Unable to parse socket address");
            axum::Server::bind(&addr).serve(api_router.into_make_service()).await.unwrap();
        },
        _ => ()
    }
}

fn create_api_route() -> Router {
    Router::new()
        .route("/", routing::get(|| async { response::Html("Hello, World!") }))
        .route("/notifications", routing::post(api::notifications::create))
}
