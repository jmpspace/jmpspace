package com.jmpspace.server.game;

import co.paralleluniverse.common.util.Function2;
import co.paralleluniverse.common.util.Function3;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass.AttachmentData;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass.Part;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass.StructureNode;
import com.jmpspace.server.game.entities.FloatingStructure;
import com.jmpspace.server.game.entities.Platform;
import com.vividsolutions.jts.geom.Coordinate;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.GeometryCollection;
import com.vividsolutions.jts.geom.GeometryFactory;
import com.vividsolutions.jts.geom.util.AffineTransformation;
import com.vividsolutions.jts.util.GeometricShapeFactory;

import java.util.*;
import java.util.concurrent.atomic.AtomicInteger;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class StructureUtils {

  static final double atomicRotation = Math.PI / 6;
  static final double atomicDistance = 1.0;

  static AffineTransformation attachmentTransform(AttachmentData attachmentData) {
    AffineTransformation floatingTransform = new AffineTransformation();
    floatingTransform.rotate(attachmentData.getOrientationFromParent() * atomicRotation);
    floatingTransform.translate(attachmentData.getOffset() * atomicDistance, 0);
    floatingTransform.rotate(attachmentData.getOrientationOfNode() * atomicRotation);
    return floatingTransform;
  }

  static Geometry applyAttachmentTransform(AttachmentData attachmentData, Geometry inputGeometry) {
    AffineTransformation floatingTransform = attachmentTransform(attachmentData);
    Geometry floatingGeometry = (Geometry) inputGeometry.clone();
    floatingGeometry.apply(floatingTransform);
    return floatingGeometry;
  }

  static <E, R> List<R> mapWithIndex(Function2<Integer, E, R> f, List<E> input) {
    return IntStream.range(0, input.size()).mapToObj(i -> f.apply(i, input.get(i))).collect(Collectors.toList());
  }

  static <TResult> TResult foldStructureNode(Function3<Integer, AttachmentData, TResult, TResult> fAttach, Function2<Part, List<TResult>, TResult> fNode, StructureNode node) {
    // TODO: replace with JOOL
    List<TResult> subResults =
            mapWithIndex((i, attachment) ->
                    fAttach.apply(i, attachment.getAttachment().getData(), foldStructureNode(fAttach, fNode, attachment.getAttachment().getNode())), node.getAttachmentsList());
    return fNode.apply(node.getPart(), subResults);
  }

  public static Geometry calculatePartGeometry(Part part) {
    switch (part.getPartCase()) {
      case PLATFORM:
        StructureOuterClass.Platform platform = part.getPlatform();
        GeometricShapeFactory shapeFactory = new GeometricShapeFactory();
        shapeFactory.setCentre(new Coordinate(0.0, 0.0));
        shapeFactory.setWidth(platform.getWidth() * atomicDistance);
        shapeFactory.setHeight(platform.getLength() * atomicDistance);
        return shapeFactory.createRectangle();
      default:
        throw new RuntimeException("Unhandled part case");
    }
  }

  public static Geometry calculateStructureGeometry(StructureNode tree) {

    GeometryFactory factory = new GeometryFactory();

    Function3<Integer, AttachmentData, Geometry, Geometry> fAttach = (_i, data, state) -> applyAttachmentTransform(data, state);

    Function2<Part, List<Geometry>, Geometry> fNode = (part, attachStates) -> {
      // TODO: combine all sub-geometry plus the geometry of this part
      Geometry partGeometry = calculatePartGeometry(part);

      attachStates.add(partGeometry);

      // TODO: check for overlapping structures

      return new GeometryCollection(attachStates.toArray(new Geometry[0]), factory);
    };

    Geometry state = StructureUtils.foldStructureNode(fAttach, fNode, tree);

    return state;
  }

  static List<List<Integer>> findCryoTubes(StructureNode tree) {

    Function3<Integer, AttachmentData, List<List<Integer>>, List<List<Integer>>> fAttach = (i, _data, cryos) ->
            cryos.stream().map(cryo -> {
              // NO PREPEND ???
              cryo.add(0, i);
              return cryo;
            }).collect(Collectors.toList());

    Function2<Part, List<List<List<Integer>>>, List<List<Integer>>> fNode = (part, cryosResults) -> {
      List<List<Integer>> newCryos = new ArrayList<>();
      cryosResults.forEach(cryoResult -> cryoResult.forEach(cryoAddr -> newCryos.add(cryoAddr)));
      if (part.hasPlatform()) {
        StructureOuterClass.Platform platform = part.getPlatform();
        platform.getPlacedItemsList().stream().filter(placedItem -> placedItem.getItem().hasCryoTube()).forEach(cryo -> newCryos.add(new ArrayList<>()));
      }
      return newCryos;
    };

    return StructureUtils.foldStructureNode(fAttach, fNode, tree);
  }

  static List<Platform> findPlatforms(FloatingStructure floatingStructureRef) {

    AtomicInteger platformCounter = new AtomicInteger();

    Function3<Integer, AttachmentData, List<Platform>, List<Platform>> fAttach = (_i, attachmentData, platforms) -> {
      AffineTransformation transform = attachmentTransform(attachmentData);
      return platforms.stream().map(platformWrapper -> {
        platformWrapper.staticRelativeTransform.composeBefore(transform);
        return platformWrapper;
      }).collect(Collectors.toList());
    };

    Function2<Part, List<List<Platform>>, List<Platform>> fNode = (part, platformResults) -> {

      List<Platform> newPlatforms = new ArrayList<>();
      platformResults.forEach(platformResult -> platformResult.forEach(platformWrapper -> newPlatforms.add(platformWrapper)));
      if (part.hasPlatform()) {
        StructureOuterClass.Platform platformPart = part.getPlatform();
        Platform wrapper = new Platform(floatingStructureRef, platformPart, new AffineTransformation());
        newPlatforms.add(wrapper);
      }
      return newPlatforms;
    };

    return StructureUtils.foldStructureNode(fAttach, fNode, floatingStructureRef.structureComponent().tree);

  }

}
