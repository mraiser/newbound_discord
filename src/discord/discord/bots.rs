use ndata::dataobject::*;
use flowlang::flowlang::data::exists::exists;
use flowlang::flowlang::data::read::read;
use ndata::data::Data;
use flowlang::flowlang::data::write::write;
use ndata::dataarray::DataArray;

pub fn execute(o: DataObject) -> DataObject {
let a0 = o.get_property("list");
let ax = bots(a0);
let mut o = DataObject::new();
o.put_object("a", ax);
o
}

pub fn bots(list:Data) -> DataObject {
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
}

