use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body("Hello AWS Lambda HTTP request".into())
        .map_err(Box::new)?;
    Ok(resp)
}

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use rand::Rng;
use std::cmp::Ordering;
use std::cmp::{Ord, PartialOrd};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Eq for Choice {}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Choice {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Choice::Rock, Choice::Scissors) => Ordering::Greater,
            (Choice::Scissors, Choice::Paper) => Ordering::Greater,
            (Choice::Paper, Choice::Rock) => Ordering::Greater,
            (Choice::Scissors, Choice::Rock) => Ordering::Less,
            (Choice::Paper, Choice::Scissors) => Ordering::Less,
            (Choice::Rock, Choice::Paper) => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

fn determine_winner(player: Choice, computer: Choice) -> String {
    match player.cmp(&computer) {
        Ordering::Equal => "It's a tie!".to_string(),
        Ordering::Greater => "You win!".to_string(),
        Ordering::Less => "Computer wins!".to_string(),
    }
}
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Welcome to Rock, Paper, Scissors!\nPlease input your choice after the slash in the URL.(e.g. /rock, /paper, /scissors)")
}
impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rock" => Ok(Choice::Rock),
            "paper" => Ok(Choice::Paper),
            "scissors" => Ok(Choice::Scissors),
            _ => Err(()),
        }
    }
}

async fn play_game(choice: web::Path<String>) -> impl Responder {
    match Choice::from_str(&choice) {
        Ok(player_choice) => {
            let computer_choice = rand::thread_rng().gen_range(0..3);
            let computer_choice = match computer_choice {
                0 => Choice::Rock,
                1 => Choice::Paper,
                _ => Choice::Scissors,
            };

            let result = determine_winner(player_choice, computer_choice);
            HttpResponse::Ok().body(result)
        }
        Err(_) => {
            HttpResponse::BadRequest().body("Invalid choice. Choose rock, paper, or scissors.")
        }
    }
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(web::resource("/{choice}").to(play_game))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
