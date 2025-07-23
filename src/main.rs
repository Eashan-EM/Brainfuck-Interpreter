use std::env;
use std::fs;
use std::io::{Read, stdin};

#[derive(Debug)]
enum InstUnits {
  InstIncrement,
  InstDecrement,
  InstNextCell,
  InstPrevCell,
  InstPrint,
  InstInput,
  InstLoopStart(usize),
  InstLoopEnd(usize),
  InstRepeat(usize)
}

fn validate_code(file_contents: String) -> Result<Vec<InstUnits>, String> {
  let mut instructions: Vec<InstUnits> = vec![];
  let mut loop_nesting: Vec<usize> = vec![];
  let mut value: usize;
  let mut nesting = 0;
  for ch in file_contents.chars() {
    match ch {
      '+' => instructions.push(InstUnits::InstIncrement),
      '-' => instructions.push(InstUnits::InstDecrement),
      '<' => instructions.push(InstUnits::InstPrevCell),
      '>' => instructions.push(InstUnits::InstNextCell),
      '.' => instructions.push(InstUnits::InstPrint),
      ',' => instructions.push(InstUnits::InstInput),
      '[' => {
        instructions.push(InstUnits::InstLoopStart(0));
        loop_nesting.push(instructions.len()-1);
        nesting += 1;
      },
      ']' => {
        if nesting==0 {
          return Err("Found closing bracket with no opening bracket to match".to_string());
        }
        value = loop_nesting.pop().expect("Loop ended unexpectedly");
        instructions[value] = InstUnits::InstLoopStart(instructions.len()-1);
        instructions.push(InstUnits::InstLoopEnd(value));
        nesting -= 1;
      },
      _   => {}
    }
  }
  if nesting!=0 {
    return Err("EOF reached but closing bracket not found".to_string());
  }
  Ok(instructions)
}

fn cell_increment(cell_pointer: isize, before_cells: &mut Vec<u8>, after_cells: &mut Vec<u8>) {
  if cell_pointer>=0 {
    if after_cells[cell_pointer as usize]==255 {
      after_cells[cell_pointer as usize] = 0;
    } else {
      after_cells[cell_pointer as usize] += 1;
    }
  } else {
    if before_cells[(-cell_pointer -1) as usize]==255 {
      before_cells[(-cell_pointer -1) as usize] = 0;
    } else {
      before_cells[(-cell_pointer -1) as usize] -= 1;
    }
  }
}

fn cell_decrement(cell_pointer: isize, before_cells: &mut Vec<u8>, after_cells: &mut Vec<u8>) {
  if cell_pointer>=0 {
    if after_cells[cell_pointer as usize]==0 {
      after_cells[cell_pointer as usize] = 255;
    } else {
      after_cells[cell_pointer as usize] -= 1;
    }
  } else {
    if before_cells[(-cell_pointer -1) as usize]==0 {
      before_cells[(-cell_pointer -1) as usize] = 255;
    } else {
      before_cells[(-cell_pointer -1) as usize] -= 1;
    }
  }
}

fn cell_next(cell_pointer: &mut isize, after_cells: &mut Vec<u8>) {
  *cell_pointer += 1;
  if *cell_pointer>=0 && after_cells.len()==*cell_pointer as usize {
    after_cells.push(0);
  } 
}

fn cell_prev(cell_pointer: &mut isize, before_cells: &mut Vec<u8>) {
  *cell_pointer = *cell_pointer-1;
  if *cell_pointer<0 && before_cells.len()==(-*cell_pointer-1) as usize {
    before_cells.push(0);
  } 
}

fn cell_print(cell_pointer: isize, before_cells: &mut Vec<u8>, after_cells: &mut Vec<u8>) {
  if cell_pointer>=0 {
    print!("{}", after_cells[cell_pointer as usize] as char);
  } else {
    print!("{}", before_cells[(-cell_pointer-1) as usize] as char)
  }
}

fn cell_input(cell_pointer: isize, before_cells: &mut Vec<u8>, after_cells: &mut Vec<u8>) {
  let mut buf: [u8; 1] = [0];
  stdin().read_exact(&mut buf).unwrap();
  if cell_pointer>=0 {
    after_cells[cell_pointer as usize] = buf[0];
  } else {
    before_cells[(-cell_pointer -1) as usize] = buf[0];
  }
}

fn cell_value(cell_pointer: isize, before_cells: &mut Vec<u8>, after_cells: &mut Vec<u8>) -> u8 {
  if cell_pointer>=0 {
    return after_cells[cell_pointer as usize];
  } else {
    return before_cells[(-cell_pointer -1) as usize];
  }
}

fn execute(inst: &Vec<InstUnits>, before_cells: &mut Vec<u8>, after_cells: &mut Vec<u8>) -> isize {
  let mut inst_pointer: usize = 0;
  let mut cell_pointer: isize = 0;

  while inst_pointer<inst.len().try_into().unwrap() {
    //println!("{:?}", inst[inst_pointer as usize]);
    //println!("{:?}", after_cells);
    match inst[inst_pointer] {
      InstUnits::InstRepeat(times) => {},
      InstUnits::InstIncrement => cell_increment(cell_pointer, before_cells, after_cells),
      InstUnits::InstDecrement => cell_decrement(cell_pointer, before_cells, after_cells),
      InstUnits::InstNextCell => cell_next(&mut cell_pointer, after_cells),
      InstUnits::InstPrevCell => cell_prev(&mut cell_pointer, before_cells),
      InstUnits::InstPrint => cell_print(cell_pointer, before_cells, after_cells),
      InstUnits::InstInput => cell_input(cell_pointer, before_cells, after_cells),
      InstUnits::InstLoopStart(goto) => {
        if cell_value(cell_pointer, before_cells, after_cells)==0 {
          inst_pointer = goto; 
        }
      },
      InstUnits::InstLoopEnd(goto) => {
        if cell_value(cell_pointer, before_cells, after_cells)!=0 {
          inst_pointer = goto;
        }
      }
    } 
    inst_pointer += 1;
  }
  0
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let file_path = &args[1];
  let file_contents = fs::read_to_string(file_path)
    .expect("File not found");
  let instructions = validate_code(file_contents).unwrap();

  let mut before_cells: Vec<u8> = vec![];
  let mut after_cells: Vec<u8> = vec![0];
  execute(&instructions, &mut before_cells, &mut after_cells);
}