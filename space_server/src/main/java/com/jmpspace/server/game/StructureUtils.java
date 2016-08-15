package com.jmpspace.server.game;

import co.paralleluniverse.common.util.Function2;
import co.paralleluniverse.common.util.Function3;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass.AttachmentData;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass.Part;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass.Platform;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass.StructureNode;
import com.jmpspace.server.game.entities.FloatingStructure;
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
                    fAttach.apply(i, attachment.getData(), foldStructureNode(fAttach, fNode, attachment.getNode())), node.getAttachmentsList());
    return fNode.apply(node.getPart(), subResults);
  }

  public static Geometry calculateStructureGeometry(StructureNode tree) {

    GeometryFactory factory = new GeometryFactory();

    Function3<Integer, AttachmentData, Geometry, Geometry> fAttach = (_i, data, state) -> applyAttachmentTransform(data, state);

    Function2<Part, List<Geometry>, Geometry> fNode = (part, attachStates) -> {
      // TODO: combine all sub-geometry plus the geometry of this part
      Geometry partGeometry;

      switch (part.getPartCase()) {
        case PLATFORM:
          Platform platform = part.getPlatform();
          GeometricShapeFactory shapeFactory = new GeometricShapeFactory();
          shapeFactory.setCentre(new Coordinate(0.0, 0.0));
          shapeFactory.setWidth(platform.getWidth() * atomicDistance);
          shapeFactory.setHeight(platform.getLength() * atomicDistance);
          partGeometry = shapeFactory.createRectangle();
          break;
        default:
          throw new RuntimeException("Unhandled part case Geometry");
      }

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
        Platform platform = part.getPlatform();
        platform.getPlacedItemsList().stream().filter(placedItem -> placedItem.getItem().hasCryoTube()).forEach(cryo -> newCryos.add(new ArrayList<>()));
      }
      return newCryos;
    };

    return StructureUtils.foldStructureNode(fAttach, fNode, tree);
  }

  static List<StructureActor.PlatformWrapper> findPlatforms(FloatingStructure floatingStructureRef) {

//    List<StructureActor.PlatformWrapper> platforms;

    AtomicInteger platformCounter = new AtomicInteger();

    Function3<Integer, AttachmentData, List<StructureActor.PlatformWrapper>, List<StructureActor.PlatformWrapper>> fAttach = (_i, attachmentData, platforms) -> {
      AffineTransformation transform = attachmentTransform(attachmentData);
      return platforms.stream().map(platformWrapper -> {
        platformWrapper.platformRelativeLocation.composeBefore(transform);
        return platformWrapper;
      }).collect(Collectors.toList());
    };

    Function2<Part, List<List<StructureActor.PlatformWrapper>>, List<StructureActor.PlatformWrapper>> fNode = (part, platformResults) -> {

      List<StructureActor.PlatformWrapper> newPlatforms = new ArrayList<>();
      platformResults.forEach(platformResult -> platformResult.forEach(platformWrapper -> newPlatforms.add(platformWrapper)));
      if (part.hasPlatform()) {
        Platform platform = part.getPlatform();
        int platformId = platformCounter.incrementAndGet();
        StructureActor.PlatformWrapper wrapper =
                new StructureActor.PlatformWrapper(platformId, platform, new AffineTransformation(), floatingStructureRef);
        newPlatforms.add(wrapper);
      }
      return newPlatforms;
    };

    return StructureUtils.foldStructureNode(fAttach, fNode, floatingStructureRef.structureComponent().tree);

  }

}
