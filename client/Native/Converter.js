Elm.Native.Converter = Elm.Native.Converter || {};
Elm.Native.Converter.make = function(_elm) {
  "use strict";
  _elm.Native.Converter = _elm.Native.Converter || {};
  if (_elm.Native.Converter.values) 
    return _elm.Native.Converter.values;

  var List = Elm.Native.List.make(_elm);

  var Proto;
  dcodeIO.ProtoBuf.loadProtoFile("contracts/actions.proto", function(err, builder) {
    Proto = builder.build("actions")
  });

  var marshalControls = function(elm_controls) {
    if (!Proto) {
      return undefined;
    }
    var action = new Proto.Action();
    var controls = new Proto.Controls();
    switch (elm_controls.ctor) {
      case "Controls_Brakes": 
        controls.setBrakes(new Proto.Unit());
        break;
      case "Controls_Active":
        var active = new Proto.Active();
        active.setGroups(List.toArray(elm_controls._0.groups));
        controls.setActive(active);
        break;
      default:
        throw "Unknown Constructor" + elm_controls.ctor;
    }
    action.setControls(controls)
    return action.toArrayBuffer();
  };

  _elm.Native.Converter.values = {
    marshalControls: marshalControls
  };

  return _elm.Native.Converter.values;
};
