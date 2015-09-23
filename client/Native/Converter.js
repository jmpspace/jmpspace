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

    function convertVessel(vessel_contract) {
      var width = vessel_contract.width;
      var length = vessel_contract.length;
      return A2(Ship.Vessel, width, length);
    }

    function convertFuelTank(fuelTank_contract) {
      var radius = fuelTank_contract.radius;
      var length = fuelTank_contract.length;
      return A2(Ship.FuelTank, radius, length);
    }

    function convertEngine(engine_contract) {
      var radius = engine_contract.radius;
      var length = engine_contract.length;
      var group = engine_contract.group;
      return A3(Ship.Engine, radius, length, group);
    }

    function convertPart(part_contract) {
      switch (part_contract.part) {
        case "vessel":
          var vessel = convertVessel(part_contract.vessel);
          return Ship.Part_vessel(vessel);
        case "fuelTank":
          var fuelTank = convertFuelTank(part_contract.fuelTank);
          return Ship.Part_fuelTank(fuelTank);
        case "engine":
          var engine = convertEngine(part_contract.engine);
          return Ship.Part_engine(engine);
        default:
          throw "unknown protocase";
      }
    }

    function convertBeam(beam_contract) {
      return Ship.Beam(beam_contract.length);
    }

    function convertStructureNode(node_contract) {
      switch (node_contract.node) {
        case "beam":
          var beam = convertBeam(node_contract.beam);
          return Ship.StructureNode_beam(beam);
        case "part":
          var part = convertPart(node_contract.part);
          return Ship.StructureNode_part(part);
        default:
          throw "unknown protocase";
      }
    }

    function convertAttach(attach_contract) {
      var location = attach_contract.location;
      var rotation = attach_contract.rotation;
      return A2(Ship.Attach,location,rotation);
    }

    function convertStructureLink(link_contract) {
      switch (link_contract.link) {
        case "root":
          return Ship.StructureLink_root(Ship.Root);
        case "attach":
          var attach = convertAttach(link_contract.attach);
          return Ship.StructureLink_attach(attach);
        default:
          throw "unknown protocase";
      }
    }

    function convertStructureTree(tree_contract) {
      var node = convertStructureNode(tree_contract.node);
      var link = convertStructureLink(tree_contract.link);
      return A2(Ship.StructureTree, node, link);
    }

    function convertStructureData(data_contract) {
      switch (data_contract.structure) {
        case "marker":
          return Ship.StructureData_marker(Ship.EndMarker);
        case "tree":
          var tree = convertStructureTree(data_contract.tree);
          return Ship.StructureData_tree(tree);
        default:
          throw "unknown protocase";
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
