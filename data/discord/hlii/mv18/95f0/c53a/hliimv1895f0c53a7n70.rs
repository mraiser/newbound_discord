let dbid = "runtime".to_string();
let ctlid = "discord_bot_list".to_string();

if list.is_string(){
  let list = list.string();
  if list.starts_with("["){
    let a = DataArray::from_string(&list);
    let mut d = DataObject::new();
    d.put_array("list", a.clone());
    write(dbid.clone(), ctlid.clone(), d.clone(), DataArray::new(), DataArray::new());
    return d;
  }
}

if exists(dbid.clone(), ctlid.clone()) {
  let d = read(dbid.clone(), ctlid.clone());
  let d = d.get_object("data");
  return d;
}

let a = DataArray::new();
let mut d = DataObject::new();
d.put_array("list", a.clone());
write(dbid.clone(), ctlid.clone(), d.clone(), DataArray::new(), DataArray::new());

d