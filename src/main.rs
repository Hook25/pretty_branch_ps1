use std::io;

fn main() -> io::Result<()> {
  let mut branch_name = String::new();
  io::stdin().read_line(&mut branch_name)?;
  let mut c_iter = branch_name.chars();
  if c_iter.clone().count() > 0 {
    let start_c = c_iter.next().unwrap();
    if start_c.is_digit(10){
      print!("{}-...", branch_name.split('-').collect::<Vec<&str>>()[0]);
    }
  }else{
    print!("{}", branch_name);
  }
  Ok(())
}
