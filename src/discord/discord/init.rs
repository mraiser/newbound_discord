use ndata::dataobject::*;
use serenity::{
          	async_trait,
          	model::{channel::Message, gateway::Ready},
          	prelude::*,
          };
use serenity::model::Timestamp;
use std::path::Path;
use std::sync::Once;
use std::thread;
use flowlang::flowlang::data::read::read;
use flowlang::flowlang::data::exists::exists;
use std::collections::HashMap;
use flowlang::command::Command;

pub fn execute(_o: DataObject) -> DataObject {
let ax = init();
let mut o = DataObject::new();
o.put_object("a", ax);
o
}

pub fn init() -> DataObject {
static START: Once = Once::new();


struct Handler {
  lib: String,
  cmd: HashMap<String, String>
}

#[async_trait]
impl EventHandler for Handler {
  async fn message(&self, ctx: Context, msg: Message) {
    //let u = msg.author.id.as_u64().to_string();
    let input = msg.content;
    if input.starts_with("!") {
      let prompt = input[1..].to_string();
      for k in self.cmd.keys() {
        let prefix = k.to_string() + " ";
        if prompt.starts_with(&prefix) || &prompt == k {
          let mut n = prefix.len();
          if &prompt == k { n -= 1; }
          let prompt = prompt[n..].to_string();
          
          let c = Command::new(&self.lib, self.cmd.get(k).unwrap());
          
          let mut d = DataObject::new();
          d.put_string("prompt", &prompt);
          d.put_int("author_id", *msg.author.id.as_u64() as i64);
          d.put_string("author_name", &msg.author.name);
          
          let d = c.execute(d).unwrap();
          println!("RESULT {}", d.to_string());
          
          let p = d.get_property("a");
          if p.is_string() {
            let txt = p.string();
            if let Err(why) = msg.channel_id.say(&ctx.http, &txt).await {
              println!("Error sending message: {:?}", why);
            }
          }
          else if p.is_object() {
            let o = p.object();

            let mut content = "".to_string();
            if o.has("content") { content = o.get_string("content"); }
            
            let mut files = Vec::new();
            let _msg = msg
            .channel_id
            .send_message(&ctx.http, |m| {
              m
              .content(&content)
              .embed(|e| {
                if o.has("title") { e.title(&o.get_string("title")); }
                if o.has("description") { e.description(&o.get_string("description")); }
//                if o.has("footer") { e.footer(&o.get_string("footer")); }
                if o.has("timestamp") { e.timestamp(Timestamp::now()); }
                if o.has("fields") {
                  let a = o.get_array("fields");
                  for a in a.objects(){
                    let a = a.array();
                    if a.get_property(0).is_string(){
                      e.field(&a.get_string(0), &a.get_string(1), a.get_boolean(2));
                    }
                    else {
                      let mut v = Vec::new();
                      for a in a.objects() {
                        let a = a.array();
                        v.push((a.get_string(0), a.get_string(1), a.get_boolean(2)));
                      }
                      e.fields(v);
                    }
                  }
                }
                e
              });
              if o.has("files") {
                let a = o.get_array("files");
                for f in a.objects(){
                  let f = f.string();
                  let f = Path::new(&f);
                  files.push(f.to_path_buf());
                }
                m.add_files(&files);
              }
              m
            })
            .await;
          }
          
          break;
        }
      }
    }
  }
  
  async fn ready(&self, _: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);
  }
}

async fn xxx(o:DataObject) {
  let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;
  
  let token = o.get_string("token");
  let lib = o.get_string("botlib");
  let ctl = o.get_string("botctl");
  let d = read(lib.clone(), ctl.clone());
  let d = d.get_object("data");
  let a = d.get_array("cmd");
  let mut cmd = HashMap::new();
  for o in a.objects(){
    let o = o.object();
    let id = o.get_string("id");
    let name = o.get_string("name");
    cmd.insert(name, id);
  }
  
  let mut client = Client::builder(&token, intents)
  .event_handler(Handler{
    lib: lib,
    cmd: cmd
  })
  .await
  .expect("Err creating client");

  if let Err(why) = client.start().await {
    println!("Client error: {:?}", why);
  }
}

START.call_once(|| { 
  
  let dbid = "runtime".to_string();
  let ctlid = "discord_bot_list".to_string();

  if exists(dbid.clone(), ctlid.clone()) {
    let d = read(dbid.clone(), ctlid.clone());
    let d = d.get_object("data");
    let d = d.get_array("list");
    for o in d.objects(){
      let o = o.object();
  
      thread::spawn(move || {
        let future = xxx(o);
        tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(future);
      });
    }
  }
});

DataObject::new()
}

