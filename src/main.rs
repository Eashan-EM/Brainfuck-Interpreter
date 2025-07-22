use std::env;
use std::fs;

#[derive(Debug)]
enum InstUnits {
  InstIncrement,
  InstDecrement,
  InstNextCell,
  InstPrevCell,
  InstPrint,
  InstInput,
  InstLoopStart,
  InstLoopEnd,
  InstRepeat(u8)
}

fn validate_code(file_contents: String) -> Result<Vec<InstUnits>, String> {
  let mut instructions: Vec<InstUnits> = vec![];
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
        instructions.push(InstUnits::InstLoopStart);
        nesting += 1;
      },
      ']' => {
        instructions.push(InstUnits::InstLoopEnd);
        if nesting==0 {
          return Err("Found closing bracket with no opening bracket to match".to_string());
        }
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
    after_cells[cell_pointer as usize] = (((after_cells[cell_pointer as usize]+1) as i16)%256) as u8;
  } else {
    before_cells[(-cell_pointer -1) as usize] = (((before_cells[(-cell_pointer -1) as usize]+1) as i16)%256) as u8;
  }
}

fn cell_decrement(cell_pointer: isize, before_cells: &mut Vec<u8>, after_cells: &mut Vec<u8>) {
  if cell_pointer>=0 {
    after_cells[cell_pointer as usize] = (((after_cells[cell_pointer as usize]-1) as i16)%256) as u8;
  } else {
    before_cells[(-cell_pointer -1) as usize] = (((before_cells[(-cell_pointer -1) as usize]-1) as i16)%256) as u8;
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
  if cell_pointer>=0 {
    after_cells[cell_pointer as usize] = after_cells[cell_pointer as usize]-1;
  } else {
    before_cells[(-cell_pointer -1) as usize] = before_cells[(-cell_pointer -1) as usize]-1;
  }
}

fn cell_value(cell_pointer: isize, before_cells: &mut Vec<u8>, after_cells: &mut Vec<u8>) -> u8 {
  if cell_pointer>=0 {
    return after_cells[cell_pointer as usize];
  } else {
    return before_cells[(-cell_pointer -1) as usize];
  }
}

fn execute(loop_start: isize, cell_ptr: isize, inst: &Vec<InstUnits>, before_cells: &mut Vec<u8>, after_cells: &mut Vec<u8>) -> isize {
  let mut inst_pointer: isize = loop_start;
  let mut cell_pointer: isize = cell_ptr;

  while inst_pointer<inst.len().try_into().unwrap() {
    //println!("{:?}", inst[inst_pointer as usize]);
    //println!("{:?}", after_cells);
    match inst[inst_pointer as usize] {
      InstUnits::InstRepeat(_times) => {},
      InstUnits::InstIncrement => cell_increment(cell_pointer, before_cells, after_cells),
      InstUnits::InstDecrement => cell_decrement(cell_pointer, before_cells, after_cells),
      InstUnits::InstNextCell => cell_next(&mut cell_pointer, after_cells),
      InstUnits::InstPrevCell => cell_prev(&mut cell_pointer, before_cells),
      InstUnits::InstPrint => cell_print(cell_pointer, before_cells, after_cells),
      InstUnits::InstInput => cell_input(cell_pointer, before_cells, after_cells),
      InstUnits::InstLoopStart => inst_pointer = execute(inst_pointer+1, cell_pointer, inst, before_cells, after_cells),
      InstUnits::InstLoopEnd => {
        if cell_value(cell_pointer, before_cells, after_cells)==0 {
          return inst_pointer;
        } else {
          inst_pointer = loop_start-1;
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
  execute(0, 0, &instructions, &mut before_cells, &mut after_cells);
}