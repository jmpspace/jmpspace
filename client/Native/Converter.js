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

  var marshalControls = function(controls) {
    if (!Proto) {
      return undefined;
    }
    var toEncode;
    switch (controls.ctor) {
      case "Controls_Brakes": 
        toEncode = new Proto.Controls();
        toEncode.setBrakes(new Proto.Unit());
        break;
      case "Controls_Active":
        toEncode = new Proto.Controls();
        var active = new Proto.Active();
        active.setGroups(List.toArray(controls._0.groups));
        toEncode.setActive(active);
        break;
      default:
        throw "Unknown Constructor" + ctor;
    }
    return toEncode.toArrayBuffer();
  };

  _elm.Native.Converter.values = {
    marshalControls: marshalControls
  };

  return _elm.Native.Converter.values;
};
