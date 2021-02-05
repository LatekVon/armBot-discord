extern crate serenity;

use serenity::prelude::*;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use std::io::*;
use std::path::Path;

//pub mod config_manager;


//to be moved to config file, and then to txt file

/*
 * Command template
 *
pub fn c_error_notfound(ctx: Context, msg: Message, args: Vec<&str>){
    let mut msg_send = String::from("Komenda nie znaleziona");
    
        //Code in here

    if let Err(why) = msg.channel_id.say(ctx.http, msg_send){
        println!("Received error '{}' while sending a message", why);
    }
}
*/


//--- HELP ---
pub fn c_help(ctx: Context, msg: Message){
    let msg_send = String::from("```c++
Dopiero tworze tego bota, ale już coś działa.
Tutaj daje komendy, które już można używać:
            
    help - pokazuje informacje o bocie
    info - pokazuje szczegółowe informacje
           o wybranej komendzie
    prefix - zmienia akutalny prefix na 
             dowolny inny o długości 1 litery
    ```");
    
    if let Err(why) = msg.channel_id.say(ctx.http, msg_send){
        println!("Received error '{}' while sending a message", why);
    }
}

//--- INFO ---
pub fn c_info(ctx: Context, msg: Message, args: Vec<&str>){
    
    let mut msg_send = String::new();

    if args.len() > 0 {
        msg_send.push_str(":arrow_forward: ");
        
        match args[0] {
            "help" => msg_send.push_str("help ``Komenda prezentująca naszego nowego bota``"),
            "info" => msg_send.push_str("info ``Co jest z tobą nie tak?``"),
            "prefix" => msg_send.push_str("prefix ``Zmienia przedrostek komend``"),
            "source" => msg_send.push_str("source ``Pokazuje pliki źródłowe bota``"),
            
            "poll" => {
                msg_send.push_str("poll ``[create|remove|setchannel|rmchannel]``");
                msg_send.push_str("    create ``[M:D:H:M:S|REMAINING:TIME, TITLE]``");
                msg_send.push_str("        ``tworzy ankietę która skończy się w odpowiednim czasie``");
                msg_send.push_str("    remove ``[TITLE|'recent']`` ``usuwa podaną ankietę`` ");
            }

            _ => {
                msg_send.push_str("Nie istnieje dokumentacja komendy ``");
                msg_send.push_str(args[0]);
                msg_send.push_str("``");
                msg_send.push_str(":exclamation:");
            }
        }    
    }
    if args.len() == 0{
       msg_send.push_str(":exclamation: Nieodpowiednie użycie komendy 'info' :exclamation:"); 
    }
    
    if let Err(why) = msg.channel_id.say(ctx.http, msg_send){
        println!("Received error '{}' while sending a message", why);
    }
}
// --- PREFIX ---
pub fn c_setprefix(ctx: Context, msg: Message, args: Vec<&str>){
    let mut msg_send = String::new();
    
    if args.len() != 1 || args[0].len() != 1{ 
        msg_send.push_str(":exclamation: Nieodpowiednie użycie komendy 'prefix' :exclamation:");
    }else if args.len() == 1 && args[0].len() == 1{
        println!();
        unsafe{    
            print!("{} -> ", PREFIX);
            PREFIX = args[0].chars().next().unwrap();
            println!("{}", PREFIX);
        }
        println!();
        msg_send.push_str("Prefix został pomyślnie zmieniony na '");
        msg_send.push_str(args[0]);
        msg_send.push_str("'"); 
    }

    if let Err(why) = msg.channel_id.say(ctx.http, msg_send){
        println!("Received error '{}' while sending a message", why);
    }
}

// --- ERROR ---
pub fn c_error_notfound(ctx: Context, msg: Message){
    let msg_send = String::from("Komenda nie znaleziona");

    if let Err(why) = msg.channel_id.say(ctx.http, msg_send){
        println!("Received error '{}' while sending a message", why);
    }
}

pub fn c_sourcefiles(ctx: Context, msg: Message){
    let msg_send = String::from("https://github.com/LatekVon/armBot-discord");

    if let Err(why) = msg.channel_id.say(ctx.http, msg_send){
        println!("Received error '{}' while sending a message", why);
    }
}

