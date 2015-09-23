Elm.Native.Converter = Elm.Native.Converter || {};
Elm.Native.Converter.make = function(_elm) {
  "use strict";
  _elm.Native.Converter = _elm.Native.Converter || {};
  if (_elm.Native.Converter.values) 
    return _elm.Native.Converter.values;

  var NativeList = Elm.Native.List.make(_elm);
  var List = Elm.List.make(_elm);
  var Ship = Elm.Contracts.Ship.make(_elm);
  var World = Elm.Contracts.World.make(_elm);

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
      case "Controls_brakes": 
        controls.setBrakes(new ActionsProto.Unit());
        break;
      case "Controls_active":
        var active = new ActionsProto.Active();
        active.setGroups(NativeList.toArray(elm_controls._0.groups));
        controls.setActive(active);
        break;
      default:
        throw "Unknown Constructor" + elm_controls.ctor;
    }
    action.setControls(controls)
    return action.toArrayBuffer();
  };

  var unmarshalSnapshot = function(messageEvent) {
    if (!ShipProto || !WorldProto) {
      return undefined;
    }
    var buffer = messageEvent.data;
    var snapshot_contract = WorldProto.Snapshot.decode(buffer);

    function convertStructureData(data_contract) {
      switch (data_contract.structure) {
        case "marker":
          debugger;
          break;
        case "tree":
          debugger;
          break;
        default:
          debugger;
      }
    }

    function convertStructure(structure_contract) {
      return Ship.Structure(
          A2(List.map,
            convertStructureData,
            NativeList.fromArray(
              structure_contract.attachments)));

    }
    
    return World.Snapshot(
        A2(List.map, 
          convertStructure, 
          NativeList.fromArray(
            snapshot_contract.ships)));
  };

  _elm.Native.Converter.values = {
    marshalControls: marshalControls,
    unmarshalSnapshot: unmarshalSnapshot 
  };

  return _elm.Native.Converter.values;
};
