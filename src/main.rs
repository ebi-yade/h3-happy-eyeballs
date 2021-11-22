fn main() {
  let mut args = std::env::args();
  let cmd = &args.next().unwrap();
  if args.len() != 1 {
      println!("Usage : {} URL", cmd);
      println!("invalid number of argument");
      println!("expected {}, but you passed {}", 1, args.len());
      return;
  }

  let url = url::Url::parse(&args.next().unwrap()).unwrap();
  println!("input url: {}", url);
}
