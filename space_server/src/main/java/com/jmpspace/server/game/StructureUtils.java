package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.common.util.Function2;
import co.paralleluniverse.common.util.Function3;
import com.jmpspace.contracts.SpaceServer.Structure;
import com.jmpspace.contracts.SpaceServer.WorldOuterClass;
import com.vividsolutions.jts.geom.Coordinate;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.GeometryCollection;
import com.vividsolutions.jts.geom.GeometryFactory;
import com.vividsolutions.jts.geom.util.AffineTransformation;
import com.vividsolutions.jts.shape.GeometricShapeBuilder;
import com.vividsolutions.jts.util.GeometricShapeFactory;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

class StructureUtils {

  static final double atomicRotation = Math.PI / 6;
  static final double atomicDistance = 1.0;

  static Geometry attachmentTransform(Structure.AttachmentData attachmentData, Geometry inputGeometry) {
    AffineTransformation floatingTransform = new AffineTransformation();
    floatingTransform.translate(attachmentData.getOffset() * 1.0, 0);
    floatingTransform.rotate(attachmentData.getOrientation() * atomicRotation);
    Geometry floatingGeometry = (Geometry) inputGeometry.clone();
    floatingGeometry.apply(floatingTransform);
    return floatingGeometry;
  }

  static <E, R> List<R> mapWithIndex(Function2<Integer, E, R> f, List<E> input) {
    return IntStream.range(0, input.size()).mapToObj(i -> f.apply(i, input.get(i))).collect(Collectors.toList());
  }

  static <TResult> TResult foldStructureNode(Function3<Integer, Structure.AttachmentData, TResult, TResult> fAttach, Function2<Structure.Part, List<TResult>, TResult> fNode, Structure.StructureNode node) {
    // TODO: replace with JOOL
    List<TResult> subResults =
            mapWithIndex((i, attachment) ->
                    fAttach.apply(i, attachment.getData(), foldStructureNode(fAttach, fNode, attachment.getNode())), node.getAttachmentsList());
    return fNode.apply(node.getPart(), subResults);
  }

  static Geometry calculateStructureGeometry(Structure.StructureNode tree) {

    GeometryFactory factory = new GeometryFactory();

    Function3<Integer, Structure.AttachmentData, Geometry, Geometry> fAttach = (_i, data, state) -> attachmentTransform(data, state);

    Function2<Structure.Part, List<Geometry>, Geometry> fNode = (part, attachStates) -> {
      // TODO: combine all sub-geometry plus the geometry of this part
      Geometry partGeometry;

      switch (part.getPartCase()) {
        case PLATFORM:
          Structure.Platform platform = part.getPlatform();
          GeometricShapeFactory shapeFactory = new GeometricShapeFactory();
          shapeFactory.setBase(new Coordinate(0.0, 0.0));
          shapeFactory.setWidth(platform.getWidth() * atomicDistance);
          shapeFactory.setHeight(platform.getLength() * atomicDistance);
          partGeometry = shapeFactory.createRectangle();
          break;
        default:
          throw new RuntimeException("Unhandled part case Geometry");
      }

      attachStates.add(partGeometry);

      // TODO: check for collisions

      return new GeometryCollection(attachStates.toArray(new Geometry[0]), factory);
    };

    Geometry state = StructureUtils.foldStructureNode(fAttach, fNode, tree);

    return state;
  }

  static List<List<Integer>> findCryoTubes(Structure.StructureNode tree) {

    Function3<Integer, Structure.AttachmentData, List<List<Integer>>, List<List<Integer>>> fAttach = (i, _data, cryos) ->
            cryos.stream().map(cryo -> {
              // NO PREPEND ???
              cryo.add(0, i);
              return cryo;
            }).collect(Collectors.toList());

    Function2<Structure.Part, List<List<List<Integer>>>, List<List<Integer>>> fNode = (part, cryosResults) -> {
      List<List<Integer>> newCryos = new ArrayList<>();
      cryosResults.forEach(cryoResult -> cryoResult.forEach(cryoAddr -> newCryos.add(cryoAddr)));
      if (part.hasPlatform()) {
        Structure.Platform platform = part.getPlatform();
        platform.getPlacedItemsList().stream().filter(placedItem -> placedItem.getItem().hasCryoTube()).forEach(cryo -> newCryos.add(new ArrayList<>()));
      }
      return newCryos;
    };

    return StructureUtils.foldStructureNode(fAttach, fNode, tree);
  }

}
