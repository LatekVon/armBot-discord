extern crate serenity;

use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;

mod command_manager;
use command_manager::*;

mod config_manager;
use config_manager::*;

mod commands_common;
mod commands_poll;

///////////////////////

const TOKEN: &str = "ODAwOTkyNTEzNzQxMDI5NDE4.YAaMaA.McSUT2ABAY69MyDdAXO0Dvb_q3o";

///////////////////////


struct Handler;
impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message){
        if msg.content.chars().next().unwrap() == unsafe {PREFIX} {
            parse_command(ctx, msg);
        }
    }

    fn ready(&self, _ctx: Context, _ready: Ready){
        println!();
        println!("All loaded up and ready to operate!");
    }

}

fn main(){

    let mut client = Client::new(&TOKEN, Handler)
                        .expect("Error creating client");
    if let Err(msg) = client.start(){
        println!("Error {:?}", msg);
    }

}
