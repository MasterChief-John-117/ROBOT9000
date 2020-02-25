use std::env;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use lazy_static::lazy_static;
use sqlite;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
    //utils::MessageBuilder,
};
pub mod messages;

lazy_static! {
    static ref DB: Mutex<sqlite::Connection> = Mutex::new(sqlite::open("test.db").unwrap());
}

struct Handler;

impl EventHandler for Handler {
    fn message(&self, _context: Context, msg: Message) {
        let clean_message = messages::clean(&msg.content);

        let mut count: i32 = 0;
        DB.lock()
            .iterate(format!("SELECT * FROM messages WHERE channelid IS {} AND content IS \"{}\"", msg.channel_id, clean_message), |pairs| {
                for &(_column, _value) in pairs.iter() {
                    count += 1;
                }
                true
            }).unwrap();

        if count != 0 {
            println!("Similar message to {} detected!", clean_message);
        }
        else {
            DB.lock().execute(format!("INSERT INTO messages VALUES ('{}', {})", clean_message, msg.channel_id).as_str()).unwrap();
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected DISCORD_TOKEN in the environment");


    match DB.lock().execute("CREATE TABLE messages (content TEXT, channelid NUM);") {
        Ok(_) => (),
        _ => ()
    }
    match DB.lock().execute("CREATE TABLE prevmutes (userid NUM, channelid NUM, length NUM, endtime NUM);") {
        Ok(_) => (),
        _ => ()
    }
    match DB.lock().execute("CREATE TABLE currmutes (userid NUM, channelid NUM, endtime NUM);") {
        Ok(_) => (),
        _ => ()
    }
    
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_millis(500));

            let start = SystemTime::now();
            let since_the_epoch = start.duration_since(UNIX_EPOCH)
                .expect("Time went backwards");
            println!("{:?}", since_the_epoch.as_secs());
            
            DB.lock()
            .iterate(format!("SELECT * FROM currmutes"), |pairs| {
                for &(column, value) in pairs.iter() {
                    println!("{}: {}", column, value.unwrap());
                }
                true
            }).unwrap();

        }
    });

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}