use std::io;

pub async fn run(mut context: crate::context::Context) -> io::Result<()> {
  println!("What should we call you? ");

  let mut name = String::new();
  io::stdin().read_line(&mut name)?;
  name = name.trim().into();

  if name.is_empty() {
    eprintln!("You must provide a name")
  }

  loop {
    let mut message = String::new();
    io::stdin().read_line(&mut message)?;

    context.add_message(name.clone(), message);
    context.write_messages();

    match context.completion().await {
      Ok(response) => println!("{}", response),
      Err(error) => eprintln!("Bad response: {}", error),
    }

    context.write_messages();
  }
}
