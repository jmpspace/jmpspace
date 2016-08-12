package com.jmpspace.server.game.physics;

import co.paralleluniverse.spacebase.ElementUpdater;
import co.paralleluniverse.spacebase.SpatialJoinVisitor;
import co.paralleluniverse.spacebase.SpatialModifyingVisitor;
import co.paralleluniverse.spacebase.SpatialToken;
import com.jmpspace.server.game.PhysicsRef;

import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentMap;
import java.util.function.BiFunction;

public class Visitors {

  public static class PhysicsStep implements SpatialModifyingVisitor<PhysicsRef> {

    @Override
    public void visit(ElementUpdater<PhysicsRef> elementUpdater) {
      elementUpdater.elem().stepPhysics(elementUpdater);
    }

    @Override
    public void done() {

    }
  }

  public static class SaveVisibleEntities implements SpatialJoinVisitor<PhysicsRef, PhysicsRef> {

    private ConcurrentMap<Integer, ArrayList<PhysicsRef>> visibleEntities;

    public SaveVisibleEntities(ConcurrentMap<Integer, ArrayList<PhysicsRef>> visibleEntities) {
      this.visibleEntities = visibleEntities;
    }

    @Override
    public void visit(PhysicsRef playerRef, SpatialToken spatialToken, PhysicsRef o2, SpatialToken spatialToken1) {
      Integer playerRefKey = playerRef.getId();
      visibleEntities.putIfAbsent(playerRefKey, new ArrayList<>());
      visibleEntities.computeIfPresent(playerRefKey, (integer, physicsRefs) -> {
        ArrayList<PhysicsRef> newRefs = (ArrayList<PhysicsRef>) physicsRefs.clone();
        newRefs.add(o2);
        return newRefs;
      });
    }
  }
}
