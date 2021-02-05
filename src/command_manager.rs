extern crate serenity;

use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use std::io::*;
use std::path::Path;



mod commands_common;
pub use commands_common::*;

mod commands_poll;
pub use commands_poll::*;



/////////////////////////////////////////////////////
pub static mut PREFIX: char = '+'; //unsafe! Too bad. 
/////////////////////////////////////////////////////

pub fn parse_command(ctx: Context, msg: Message){
    println!("Received: '{}' on channel: '{}'", msg.content, msg.channel_id);
   
    //remove first char
    let mut v_received: Vec<char> = msg.content.chars().collect();
    v_received.remove(0);//this may cause error
    let received: String = v_received.into_iter().collect();
    
    //save args and separate command
    let split_received = received.split(' ');
    let mut args: Vec<&str> = split_received.collect();
    let command: &str = args[0];
    args.remove(0);
    
    match command {
        "help" => c_help(ctx, msg),
        "info" => c_info(ctx, msg, args),
        "prefix" => c_setprefix(ctx, msg, args),
        "source" => c_sourcefiles(ctx, msg),
        
        "poll" => c_poll(ctx, msg, args); 

        _ => c_error_notfound(ctx, msg)
    }
}
