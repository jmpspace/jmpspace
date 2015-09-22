Elm.Native.Converter = Elm.Native.Converter || {};
Elm.Native.Converter.make = function(_elm) {
  "use strict";
  _elm.Native.Converter = _elm.Native.Converter || {};
  if (_elm.Native.Converter.values) 
    return _elm.Native.Converter.values;

  var List = Elm.Native.List.make(_elm);

  var ActionsProto;
  dcodeIO.ProtoBuf.loadProtoFile("contracts/actions.proto", function(err, builder) {
    ActionsProto = builder.build("actions")
  });
  
  var ShipProto;
  dcodeIO.ProtoBuf.loadProtoFile("contracts/ship.proto", function(err, builder) {
    ShipProto = builder.build("ship")
  });

  var WorldProto;
  dcodeIO.ProtoBuf.loadProtoFile("contracts/world.proto", function(err, builder) {
    WorldProto = builder.build("world")
  });

  var marshalControls = function(elm_controls) {
    if (!ActionsProto) {
      return undefined;
    }
    var action = new ActionsProto.Action();
    var controls = new ActionsProto.Controls();
    switch (elm_controls.ctor) {
      case "Controls_Brakes": 
        controls.setBrakes(new Proto.Unit());
        break;
      case "Controls_Active":
        var active = new ActionsProto.Active();
        active.setGroups(List.toArray(elm_controls._0.groups));
        controls.setActive(active);
        break;
      default:
        throw "Unknown Constructor" + elm_controls.ctor;
    }
    action.setControls(controls)
    return action.toArrayBuffer();
  };

  var unmarshalSnapshot = function(buffer) {
    if (!ShipProto || !WorldProto) {
      return undefined;
    }
    debugger;
  };

  _elm.Native.Converter.values = {
    marshalControls: marshalControls,
    unmarshalSnapshot: unmarshalSnapshot 
  };

  return _elm.Native.Converter.values;
};
