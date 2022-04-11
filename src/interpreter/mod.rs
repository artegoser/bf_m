use std::io;

pub struct Interpreter {
  input: String,
  mem: Vec<u8>,
  mem_addr: usize,
  pos: usize,
}

impl Interpreter {
  pub fn new(input: String) -> Interpreter {
    Interpreter {
      input,
      mem: Vec::from([0]),
      pos: 0,
      mem_addr: 0,
    }
  }

  pub fn run(&mut self) -> io::Result<()> {
    let chars: Vec<char> = self.input.chars().collect();
    loop {
      if self.pos >= chars.len() {
        break;
      }
      let mut char = chars[self.pos];

      if char == '+' {
        if self.mem[self.mem_addr] == 255 {
          self.mem[self.mem_addr] = 0;
        } else {
          self.mem[self.mem_addr] += 1;
        }
      } else if char == '-' {
        if self.mem[self.mem_addr] == 0 {
          self.mem[self.mem_addr] = 255;
        } else {
          self.mem[self.mem_addr] -= 1;
        }
      } else if char == '.' {
        print!("{}", self.mem[self.mem_addr] as char)
      } else if char == ',' {
        let mut buffer: String = "".to_string();
        io::stdin().read_line(&mut buffer)?;
        let total: Vec<char> = buffer.chars().collect();
        self.mem[self.mem_addr] = total[0] as u8;
      } else if char == '>' {
        if self.mem.len() - 1 <= self.mem_addr {
          self.mem.push(0);
        }
        self.mem_addr += 1;
      } else if char == '<' {
        self.mem_addr -= 1;
      } else if char == '[' {
        if self.mem[self.mem_addr] == 0 {
          let mut brackets: usize = 0;
          loop {
            self.pos += 1;
            char = chars[self.pos];
            if char == '[' {
              brackets += 1;
            } else if char == ']' {
              if brackets == 0 {
                break;
              }
              brackets -= 1;
            }
          }
        }
      } else if char == ']' {
        if self.mem[self.mem_addr] != 0 {
          let mut brackets: usize = 0;
          loop {
            self.pos -= 1;
            char = chars[self.pos];
            if char == ']' {
              brackets += 1;
            } else if char == '[' {
              if brackets == 0 {
                break;
              }
              brackets -= 1;
            }
          }
        }
      }

      self.pos += 1;
    }
    println!();
    Ok(())
  }
}
