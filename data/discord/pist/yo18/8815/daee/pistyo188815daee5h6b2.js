var me = this; 
var ME = $('#'+me.UUID)[0];

me.uiReady = function(ui){
  me.ui = ui;
  $(ME).find('.wrap').css('display', 'block');
};

me.ready = function(){
  send_bots(null, function(result){
    var el = $(ME).find('.botlist');
    var d = result.data;
    me.data = d.list;
    d.allowadd = true;
    d.title = 'Installed Bots';
    d.emptytext = "<i>No bots found</i>";
    d.click_add = me.click_add;
    d.click_edit = me.click_edit;
    d.on_delete = me.save;
    installControl(el[0], "app", "list", function(api){
      me.list = api;
    }, d);
  });
};

me.save = function(){
  send_bots(me.data, function(result){
    console.log(result);
  });
};

me.click_edit = function(data, index){
  me.index = index;
  set(data);
  $(ME).find('.dialog-title').text('Edit Bot');
  var d = {
    "selector": ".editbotpopup",
    "closeselector": ".close-add-dialog",
    "modal": true
  };
  $('.cadb').css('display', 'none');
  $('.dosavebutton').css('display', 'block');
  me.ui.popup(d);
};

$(ME).find('.dosavebutton').click(function(){
  var d = extract();
  me.list.set_item(d, me.index);
  me.save();
});

me.click_add = function(){
  $(ME).find('.dialog-title').text('Add Bot');
  var d = {
    "selector": ".editbotpopup",
    "closeselector": ".close-add-dialog",
    "modal": true
  };
  $('.cadb').css('display', 'none');
  $('.doaddbutton').css('display', 'block');
  me.ui.popup(d);
};

$(ME).find('.doaddbutton').click(function(){
  var d = extract();
  me.list.add_item(d);
  me.save();
});

function extract(){
  var name = $(ME).find('#newbotname').val();
  var token = $(ME).find('#newbottoken').val();
  var botlib = $(ME).find('.selectlibwrap').find('select').val();
  var sel = $(ME).find('.selectctlwrap').find('select');
  var botctl = sel.val();
  var botctlname = sel[0].options[sel[0].selectedIndex].text;
  var displayname = '<b>'+name+'</b><br><font size="x-small">'+botlib+':'+botctlname+'</font>';
  var d = {
    "name": name,
    "token": token,
    "botlib": botlib,
    "botctl": botctl,
    "botctlname": botctlname,
    "displayname": displayname,
    "id": botctl
  };
  console.log(d);
  return d;  
}

function set(d){
  $(ME).find('#newbotname').val(d.name);
  $(ME).find('#newbottoken').val(d.token)
  //FIXME
  $(ME).find('.selectlibwrap').find('select').val(d.botlib).trigger("change");
  setTimeout(function(){
    $(ME).find('.selectctlwrap').find('select').val(d.botctl).trigger("change");
  }, 1000);
}