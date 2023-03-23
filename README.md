# Rock, Paper, Scissors

[![Build Status](https://travis-ci.org/kevinmichaelchen/rps.svg?branch=master)](https://travis-ci.org/kevinmichaelchen/rps)

This is a simple web application that allows users to play Rock, Paper, Scissors against a computer opponent.
The application is built using the Rust programming language and the Actix-web framework.

## Requirements

To run this application, you must have Rust and Cargo installed on your system. You can download Rust and Cargo from the official website: <https://www.rust-lang.org/tools/install>

## Running the application

To run the application, open a terminal window and navigate to the directory containing the source code. Then, execute the following command:

Copy code
`cargo run`
This will start the web server on <http://127.0.0.1:8080/>.

To play the game, navigate to <http://127.0.0.1:8080/> in your web browser. The home page will display instructions on how to play the game.

To make a move, simply add your move to the end of the URL. For example, to play "Rock", navigate to <http://127.0.0.1:8080/rock>. The computer will choose a move at random, and the result of the game will be displayed.

## References

- [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
