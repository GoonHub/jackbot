use clap::{App, ArgMatches};

pub fn app() -> App<'static> {
  App::new("jackbot")
    .about("Chat with Jack")
    .subcommand(App::new("chat").about("Talk to Jack through your terminal"))
    .subcommand(App::new("server").about("Start the Discord bot server"))
}

pub async fn match_cmd(matches: ArgMatches) {
  match matches.subcommand() {
    Some(("chat", chat_m)) => cmd_chat(chat_m).await,
    Some(("server", server_m)) => cmd_server(server_m),
    _ => eprintln!("subcommand is required, use help for more details"),
  }
}

async fn cmd_chat(_: &ArgMatches) {
  match crate::stdio::run(crate::context::from_env()).await {
    Ok(_) => println!("Done"),
    Err(error) => eprintln!("Something went wrong: {}", error),
  }
}

fn cmd_server(_: &ArgMatches) {
  println!("Not implemented")
}
