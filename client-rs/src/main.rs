extern crate cairo;
extern crate minifb;

use minifb::{ Key, Window, WindowOptions };

const WIDTH: usize = 640;
const HEIGHT: usize = 360;


struct TableTennisGame {
    your_pos:     i32,
    opponent_pos: i32,
}

fn draw_next (buffer: &mut Vec<u32>) {
        for i in buffer.iter_mut () {
            let red: u32 = 0x28;
            let green: u32 = 0x28;
            let blue: u32 = 0x28;
            *i = (red << 16) | (green << 8) | blue;
        }
}

fn game_loop (buffer: &mut Vec<u32>, window: &mut Window) {
     while window.is_open () && !window.is_key_down (Key::Escape) {

        draw_next (buffer);

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap ();
    }
}

fn main () {
    let mut game: TableTennisGame = TableTennisGame {
        your_pos: 0,
        opponent_pos: 0,
    };
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    
    let mut window = Window::new (
        "Test - EXC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default (),
    ).unwrap_or_else (|e| {
        panic! ("{}", e);
    });

    window.limit_update_rate (Some (std::time::Duration::from_micros (16600)));
    game_loop (&mut buffer, &mut window, &mut game);

}
