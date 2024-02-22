#[derive(argsproc::HelloMacro)]
struct Hoge {}

fn main() {
  let _ = Hoge {};
  Hoge::a();
  println!("{}", boilerplate::hello_world());

  // let sec = std::time::Duration::from_secs(10);
  // let now = std::time::Instant::now();
  // std::thread::sleep(sec);
}