use std::io;

/*
Reads branch from stdin
- Outputs N-... for branches starting with a number
- Outputs the branch name for others

Useful for PS1 when using gitlab where the default name
for MR is NNN-issue-name
*/

//function from: https://stackoverflow.com/questions/37888042/remove-single-trailing-newline-from-string-without-cloning
fn trim_newline(s: &mut String){
  if s.ends_with('\n') {
      s.pop();
      if s.ends_with('\r') {
          s.pop();
      }
  }
}
fn main() -> io::Result<()> {
  let mut branch_name = String::new();
  io::stdin().read_line(&mut branch_name)?;
  let mut c_iter = branch_name.chars();
  if c_iter.clone().count() > 0 {
    let start_c = c_iter.next().unwrap();
    if start_c.is_digit(10){
      let mut branch_number = branch_name.split('-')
        .collect::<Vec<&str>>()[0].to_string();
      trim_newline(&mut branch_number);
      print!("{}-...", branch_number);
    }else{
      let mut branch_name_stripped = branch_name.to_string();
      trim_newline(&mut branch_name_stripped);
      print!("{}", branch_name_stripped);
    }
  }
  Ok(())
}
