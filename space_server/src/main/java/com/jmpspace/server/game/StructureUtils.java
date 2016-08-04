package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.common.util.Function2;
import co.paralleluniverse.common.util.Function3;
import com.jmpspace.contracts.SpaceServer.Structure;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.GeometryCollection;
import com.vividsolutions.jts.geom.GeometryFactory;
import com.vividsolutions.jts.geom.util.AffineTransformation;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

class StructureUtils {

  static <E,R> List<R> mapWithIndex(Function2<Integer, E, R> f, List<E> input) {
    return IntStream.range(0, input.size()).mapToObj(i -> f.apply(i, input.get(i))).collect(Collectors.toList());
  }

  static <TResult> TResult foldStructureNode(Function3<Integer, Structure.AttachmentData, TResult, TResult> fAttach, Function2<Structure.Part, List<TResult>, TResult> fNode, Structure.StructureNode node) {
    List<TResult> subResults =
            mapWithIndex((i, attachment) ->
                    fAttach.apply(i, attachment.getData(), foldStructureNode(fAttach, fNode, attachment.getNode())), node.getAttachmentsList());
    return fNode.apply(node.getPart(), subResults);
  }

  static class CalculateGeometryState {
    AffineTransformation _currentOffset;
    Geometry _accum;

    public CalculateGeometryState() {
      GeometryFactory factory = new GeometryFactory();
      this._accum = new GeometryCollection(new Geometry[0], factory);
      this._currentOffset = new AffineTransformation();
    }
  }

  static Geometry calculateStructureGeometry(Structure.StructureNode tree) {

    Function3<Integer, Structure.AttachmentData, CalculateGeometryState, CalculateGeometryState> fAttach = (_i, data, state) -> {
      // TODO: translate the sub-tree
      return state;
    };

    Function2<Structure.Part, List<CalculateGeometryState>, CalculateGeometryState> fNode = (part, attachStates) -> {
      // TODO: combine all sub-geometry plus the geometry of this part
      return new CalculateGeometryState();
    };

    CalculateGeometryState state = StructureUtils.foldStructureNode(fAttach, fNode, tree);

    return state._accum;
  }

  static List<List<Integer>> findCryoTubes(Structure.StructureNode tree) {

    Function3<Integer, Structure.AttachmentData, List<List<Integer>>, List<List<Integer>>> fAttach = (i, _data, cryos) ->
            cryos.stream().map(cryo -> {
              // NO PREPEND ???
              cryo.add(0, i); return cryo;
            }).collect(Collectors.toList());

    Function2<Structure.Part, List<List<List<Integer>>>, List<List<Integer>>> fNode = (part, cryosResults) -> {
      List<List<Integer>> newCryos = new ArrayList<>();
      cryosResults.forEach(cryoResult -> cryoResult.forEach(cryoAddr -> newCryos.add(cryoAddr)));
      if (part.hasPlatform()) {
        Structure.Platform platform = part.getPlatform();
        platform.getPlacedItemsList().stream().filter(placedItem -> placedItem.getItem().hasCryoTube()).forEach(cryo -> newCryos.add(new ArrayList()));
      }
      return newCryos;
    };

    return StructureUtils.foldStructureNode(fAttach, fNode, tree);
  }

}
