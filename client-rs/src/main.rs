extern crate minifb;
extern crate tungstenite;
extern crate url;
extern crate serde_json;

mod pong_drawing;

use pong_drawing::{ fill_background, draw_rectangle, draw_circle};
use tungstenite::{ connect, Message };
use tungstenite::stream::MaybeTlsStream;

use minifb::{ Key, Window, WindowOptions };
use url::Url;

const HEIGHT: usize = 600;
const WIDTH: usize = 800;
const BLACK: u32 = 0x282828;
const WHITE: u32 = 0xfbf1c7;


const PADDLE_HEIGHT: usize = 100;
const PADDLE_WIDTH: usize = 10;

#[derive(Clone)]
struct MPState {
    left_paddle_y: i32,
    right_paddle_y: i32,
    ball_x: i32,
    ball_y: i32,
}

struct PlayerSession {
    state: MPState,
    buffer: Vec<u32>,
    window: Window,
    socket: tungstenite::protocol::WebSocket<MaybeTlsStream<std::net::TcpStream>>
}

impl PlayerSession {
    fn new(title: &str) -> Self {
        let buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
        let window = Window::new(
            title,
            WIDTH,
            HEIGHT,
            WindowOptions::default(),
        ).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        let socket = connect(Url::parse("ws://localhost:8080").unwrap())
            .expect("Can't connect")
            .0;

        PlayerSession {
            state: MPState {
                left_paddle_y: 300,
                right_paddle_y: 300,
                ball_x: 300,
                ball_y: 300,
            },
            buffer,
            window,
            socket,
        }
    }

    fn run(&mut self) {
        fill_background(&mut self.buffer, BLACK);

        self.window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        let mut current_state = self.state.clone();
        let mut next_state = self.state.clone();

        while self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            if self.window.is_key_down(Key::Up) {
                self.socket.send(
                    Message::Text("{ action: 'move', direction: 'up' }".into())
                ).unwrap();
            } else if self.window.is_key_down(Key::Down) {
                self.socket.send(
                    Message::Text("{ action: 'move', direction: 'down' }".into())
                ).unwrap();
            }

            let msg = self.socket.read();
            match msg {
                Ok(Message::Text(parsed_msg)) => {
                    let parsed: serde_json::Value = serde_json::from_str(&parsed_msg).expect("Can't parse to JSON");
                    write_state(&parsed, &mut next_state);
                    draw_next(&mut self.buffer, &current_state, &next_state);
                },
                Ok(other) => {
                    println!("Received non-text message: {:?}", other);
                    continue;
                },
                Err(e) => {
                    println!("Error reading from socket: {:?}", e);
                    break;
                },
            }

            self.window.update_with_buffer(&self.buffer, WIDTH, HEIGHT).unwrap();

            current_state = next_state.clone();
        }
    }
}



fn draw_next (buffer: &mut Vec<u32>, current_state: &MPState, next_state: &MPState) {
    
    draw_circle (
        buffer,
        current_state.ball_x as usize,
        current_state.ball_y as usize,
        10,
        BLACK,
        WIDTH,
        HEIGHT,
    );

    draw_circle (
        buffer,
        next_state.ball_x as usize,
        next_state.ball_y as usize,
        10,
        WHITE,
        WIDTH,
        HEIGHT,
    );

    // left paddle erase
    draw_rectangle(
        buffer,
        PADDLE_WIDTH / 2,
        current_state.left_paddle_y as usize,
        PADDLE_WIDTH,
        PADDLE_HEIGHT,
        BLACK,
        WIDTH,
        HEIGHT
    );

    // right paddle erase
    draw_rectangle(
        buffer,
        WIDTH - PADDLE_WIDTH / 2 - 1,
        current_state.right_paddle_y as usize,
        PADDLE_WIDTH,
        PADDLE_HEIGHT,
        BLACK,
        WIDTH,
        HEIGHT,
    );

    // left paddle draw

    draw_rectangle(
        buffer,
        PADDLE_WIDTH / 2,
        next_state.left_paddle_y as usize,
        PADDLE_WIDTH,
        PADDLE_HEIGHT,
        WHITE,
        WIDTH,
        HEIGHT,
    );

    // right paddle draw

    draw_rectangle(
        buffer,
        WIDTH - PADDLE_WIDTH / 2 - 1,
        next_state.right_paddle_y as usize,
        PADDLE_WIDTH,
        PADDLE_HEIGHT,
        WHITE,
        WIDTH,
        HEIGHT,
    );




}

fn write_state (parsed: &serde_json::Value, state: &mut MPState) {
    match parsed["leftPaddleY"].as_i64() {
        Some(p) => {state.left_paddle_y = p as i32},
        None => println!("field is empty"),
    }
    match parsed["rightPaddleY"].as_i64() {
        Some(p) => {state.right_paddle_y = p as i32},
        None => println!("field is empty"),
    }
    match parsed["ballX"].as_i64() {
        Some(p) => {state.ball_x = p as i32},
        None => println!("field is empty"),
    }
    match parsed["ballY"].as_i64() {
        Some(p) => {state.ball_y = p as i32},
        None => println!("field is empty"),
    }
}

fn main () {
    println!("player1 created");
    let mut player1 = PlayerSession::new("Player 1");
    println!("player2 created");
    let mut player2 = PlayerSession::new("Player 2");

    loop {
        println!("player1.run()");
        player1.run();
        println!("player2.run()");
        player2.run();
    
        if (!player1.window.is_open() || player1.window.is_key_down(Key::Escape))
            && (!player2.window.is_open() || player2.window.is_key_down(Key::Escape)) {
                break;
        }
    }

}
