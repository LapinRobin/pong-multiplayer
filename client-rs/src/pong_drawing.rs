pub fn fill_background (buffer: &mut Vec<u32>, color: u32) {
    for i in buffer.iter_mut () {
        *i = color;
    }
}

pub fn draw_rectangle (buffer: &mut Vec<u32>, x: usize, y: usize, width: usize, height: usize, color: u32, max_width: usize, max_height: usize) {
    if x < width / 2 || x + width / 2 >= max_width || y < height / 2 || y + height / 2 >= max_height {
        return;
    }
    for i in 0..=(width/2) {
        for j in 0..=(height/2) {
            buffer[(y-j)*max_width+(x-i)] = color;
            buffer[(y-j)*max_width+(x+i)] = color;
            buffer[(y+j)*max_width+(x-i)] = color;
            buffer[(y+j)*max_width+(x+i)] = color;
        }
    }
}

pub fn draw_circle (buffer: &mut Vec<u32>, x: usize, y: usize, r: usize, color: u32, max_width: usize, max_height: usize) {  
    if x < r || x + r >= max_width || y < r || y + r >= max_height {
        return; 
    }
    for i in 0..=r {
        for j in 0..=r {
            if i*i + j*j <= r*r {
                buffer[(y-j)*max_width+(x-i)] = color;
                buffer[(y-j)*max_width+(x+i)] = color;
                buffer[(y+j)*max_width+(x-i)] = color;
                buffer[(y+j)*max_width+(x+i)] = color;
            }
        }
    }
}
