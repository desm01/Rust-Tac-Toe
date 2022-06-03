use std::io::{Stdout, Write};

pub struct Board {
    board: Vec<Vec<char>>,
    current_position: (usize, usize)
}

impl Board {
    pub fn new() -> Board {
        let mut vec = Vec::new();
        for _i in 0..3 {
            vec.push(vec!['-','-','-']);
        }

        Board { board: vec, current_position: (0,0) }
    }

    pub fn move_position(&mut self, (x,y): (usize, usize)) {
        self.current_position = (x, y)
    }

    pub fn draw_board(&self, stdout: &mut Stdout) {
        std::process::Command::new("clear").status().unwrap();
        for line in self.board.iter() {
            println!("\r{}{}{}", line[0], line[1], line[2]);
        }
        stdout.flush().unwrap();
    }
}