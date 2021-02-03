extern crate serenity;

use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;

mod command_manager;
pub use command_manager::*;

mod config_manager;
pub use config_manager::*;

///////////////////////

const TOKEN: &str = "ODAwOTkyNTEzNzQxMDI5NDE4.YAaMaA.mpVbNlpO91Bl8S2ZjXI08emVz2M";

///////////////////////

struct Handler;
impl EventHandler for Handler {

    fn message(&self, ctx: Context, msg: Message){
        if msg.content.chars().next().unwrap() == PREFIX {
            println!("Received: '{}' on channel: '{}'", msg.content, msg.channel_id);
            
            //remove first char
            let mut v_received: Vec<char> = msg.content.chars().collect();
            v_received.remove(0);//this may cause error
            let received: String = v_received.into_iter().collect();
            
            //save and remove command
            let split_received = received.split(' ');
            let mut args: Vec<&str> = split_received.collect();
            let command: &str = args[0];
            args.remove(0);
            
            match command {
                "help" => c_help(ctx, msg),
                "info" => c_info(ctx, msg, args),
                "prefix" => c_setprefix(ctx, msg, args),
                
                _ => c_error_notfound(ctx, msg)
            }
        }
    }

    fn ready(&self, _: Context, _ready: Ready){
        println!("All loaded up and ready");
    }
}

fn main(){

    let mut client = Client::new(&TOKEN, Handler)
                        .expect("Error creating client");
    if let Err(msg) = client.start(){
        println!("Error {:?}", msg);
    }

}
