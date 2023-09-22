extern crate minifb;
extern crate tungstenite;
extern crate url;
extern crate serde_json;

use tungstenite::{ connect, Message };
use minifb::{ Key, Window, WindowOptions };
use url::Url;

const HEIGHT: usize = 600;
const WIDTH: usize = 800;

const BLACK: u32 = 0x282828;
const WHITE: u32 = 0xfbf1c7;
const PADDLE_HEIGHT: usize = 100;
const PADDLE_WIDTH: usize = 10;

struct MPState {
    left_paddle_y: i32,
    right_paddle_y: i32,
    ball_x: i32,
    ball_y: i32,
}

fn fill_background (buffer: &mut Vec<u32>) {
    for i in buffer.iter_mut () {
        *i = BLACK;
    }
}

fn draw_circle (buffer: &mut Vec<u32>, x: usize, y: usize, r: usize, color: u32) {  
    if x < r || x + r >= WIDTH || y < r || y + r >= HEIGHT {
        return; 
    }
    for i in 0..=r {
        for j in 0..=r {
            if i*i + j*j <= r*r {
                buffer[(y-j)*WIDTH+(x-i)] = color;
                buffer[(y-j)*WIDTH+(x+i)] = color;
                buffer[(y+j)*WIDTH+(x-i)] = color;
                buffer[(y+j)*WIDTH+(x+i)] = color;
            }
        }
    }
}

fn draw_rectangle (buffer: &mut Vec<u32>, x: usize, y: usize, width: usize, hight: usize, color: u32) {
    for i in 0..=(width/2) {
        for j in 0..=(hight/2) {
            buffer[(y-j)*WIDTH+(x-i)] = color;
            buffer[(y-j)*WIDTH+(x+i)] = color;
            buffer[(y+j)*WIDTH+(x-i)] = color;
            buffer[(y+j)*WIDTH+(x+i)] = color;
        }
    }
}

fn draw_next (buffer: &mut Vec<u32>, current_state: &MPState, next_state: &MPState) {
    draw_circle (buffer, current_state.ball_x as usize, current_state.ball_y as usize, 10, BLACK);

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
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    fill_background (&mut buffer);

    let mut window = Window::new (
        "Test - EXC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default (),
        ).unwrap_or_else (|e| {
        panic! ("{}", e);
    });

    window.limit_update_rate (Some (std::time::Duration::from_micros (16600)));

    let mut current_state = MPState {
        left_paddle_y: 300,
        right_paddle_y: 300,
        ball_x: 300,
        ball_y: 300,
    };

    let mut next_state = MPState {
        left_paddle_y: 300,
        right_paddle_y: 300,
        ball_x: 300,
        ball_y: 300,
    };

    let (mut socket, response) =
        connect (Url::parse ("ws://localhost:8080").unwrap ())
        .expect("Can't connect");

    while window.is_open () && !window.is_key_down (Key::Escape) {

        if window.is_key_down (Key::Up) {
            socket.send (
                Message::Text ("{ action: 'move', direction: 'up' }".into ()))
                .unwrap ();
        } else if window.is_key_down (Key::Down) {
            socket.send (
                Message::Text ("{ action: 'move', direction: 'down' }".into ()))
                .unwrap ();
        }

        let msg = socket.read();
        let msg = match msg {
            Ok(Message::Text(s)) => s,
            _ => { panic!() },
        };

        let parsed: serde_json::Value = serde_json::from_str(&msg).expect("Can't parse to JSON");

        write_state (&parsed, &mut next_state);
        draw_next (&mut buffer, &current_state, &next_state);

        window
            .update_with_buffer (&buffer, WIDTH, HEIGHT)
            .unwrap ();

        current_state.ball_y = next_state.ball_y;
        current_state.ball_x = next_state.ball_x;
        current_state.right_paddle_y = next_state.right_paddle_y;
        current_state.left_paddle_y = next_state.left_paddle_y;
    }
}
