package com.jmpspace.server.game.physics;

import co.paralleluniverse.spacebase.ElementUpdater;
import co.paralleluniverse.spacebase.SpatialJoinVisitor;
import co.paralleluniverse.spacebase.SpatialModifyingVisitor;
import co.paralleluniverse.spacebase.SpatialToken;
import com.jmpspace.server.game.PhysicsRef;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

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

    private ConcurrentMap<Integer, ConcurrentMap<PhysicsRef, Boolean>> visibleEntities;

    public SaveVisibleEntities(ConcurrentMap<Integer, ConcurrentMap<PhysicsRef, Boolean>> visibleEntities) {
      this.visibleEntities = visibleEntities;
    }

    @Override
    public void visit(PhysicsRef playerRef, SpatialToken spatialToken, PhysicsRef o2, SpatialToken spatialToken1) {
      Integer playerRefKey = playerRef.getId();
      visibleEntities.putIfAbsent(playerRefKey, new ConcurrentHashMap<>());
      visibleEntities.get(playerRefKey).put(o2, true);
    }
  }
}
