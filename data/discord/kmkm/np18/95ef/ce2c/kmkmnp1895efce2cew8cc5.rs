let dbid = "runtime".to_string();
let ctlid = "discord_bot_list".to_string();
if exists(dbid.clone(), ctlid.clone()) {
  return read(dbid, ctlid);
}
DataObject::new()