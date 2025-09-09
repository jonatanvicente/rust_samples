

pub fn go() {
  let result = process(5, |x| x * 2);
  println!("{}", result);

}

pub fn process<F: Fn(i32) -> i32>(x:i32, callback: F) -> i32 {
  callback(x)
}