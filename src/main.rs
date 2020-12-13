extern crate discord;

use discord::model::Event;
use discord::Discord;
use eliza::Eliza;

fn main() {
    let mut eliza = Eliza::from_str(include_str!("doctor.json")).unwrap();

    // Log in to Discord using a bot token from the environment
    let discord =
        Discord::from_bot_token("Nzg3NDIxNTc4NzE5Nzg5MTA2.X9UtfQ.Aw74II3ERJWF-Upbe5BmUT5hFUc")
            .expect("login failed");

    // Establish and use a websocket connection
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready.");
    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                if message.author.name == "Eliza" {
                    continue;
                } else {
                    if message.content == "&start" {
                        let _ =
                            discord.send_message(message.channel_id, &eliza.greet(), "", false);
                    } else if message.content == "&leave" {
                        let _ = discord.send_message(
                            message.channel_id,
                            &eliza.farewell(),
                            "",
                            false,
                        );
                    } else if message.content == "&help" {
                        let _ = discord.send_message(
							    message.channel_id,
							    &format!("This is a discord implementation of the early 'chatbot' program ELIZA.The original program was developed from 1964 to 1966 at the MIT Artificial Intelligence Laboratory by Joseph Weizenbaum. \n\nCommands:\n&start - Start a session\n&respond <TEXT> - Reply to an answer\n&leave - Leave the session\n\nDeveloped with love, in Rust, by Grant Handy :penguin:"),
							    "",
							    false,
						    );
                    } else if message.content.contains("&respond") {
                        let response: String = message.content.chars().skip(7).collect();

                        let _ = discord.send_message(
                            message.channel_id,
                            &eliza.respond(&response),
                            "",
                            false,
                        );

                        println!("{}", response);
                    }
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Receive error: {:?}", err),
        }
    }
}
