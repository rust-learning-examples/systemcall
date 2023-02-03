use clap::{Parser, Subcommand};
use std::net::SocketAddr;

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


use axum::{
    Router,
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    match &cli.command {
        Commands::Start(args) => {
            let addr = format!("{}:{}", args.host, args.port);
            tracing::debug!("listening on {}", addr);

            let app = Router::new()
                // `GET /` goes to `root`
                .route("/", get(|| async { "Hello, World!" }))
                // `POST /users` goes to `create_user`
                // .route("/notifications", post(|| -> impl IntoResponse {
                //     use notify_rust::Notification;
                //     Notification::new()
                //         .summary("Firefox News")
                //         .body("This will almost look like a real firefox notification.")
                //         .icon("firefox")
                //         .show().unwrap();
                //     StatusCode::CREATED
                // }))
                ;

            let addr = addr.parse().expect("Unable to parse socket address");
            axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
        },
        _ => ()
    }
}

async fn root() -> &'static str {
    "Hello, World!"
}
