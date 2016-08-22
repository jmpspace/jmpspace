package com.jmpspace.server.game.physics;

import co.paralleluniverse.spacebase.ElementUpdater;
import co.paralleluniverse.spacebase.SpatialJoinVisitor;
import co.paralleluniverse.spacebase.SpatialModifyingVisitor;
import co.paralleluniverse.spacebase.SpatialToken;
import com.jmpspace.server.game.Instance;
import com.jmpspace.server.game.ecs.Entity;
import com.jmpspace.server.game.ecs.Entity.HasCamera;
import com.jmpspace.server.game.ecs.Entity.HasPhysics;
import com.jmpspace.server.game.ecs.Entity.HashSerializeEntity;
import com.jmpspace.server.game.ecs.PhysicsComponent;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

public class Visitors {

  public static class PhysicsStep implements SpatialModifyingVisitor<HasPhysics> {

    private static final Logger logger = LogManager.getLogger(PhysicsStep.class.getName());

    @Override
    public void visit(ElementUpdater<HasPhysics> elementUpdater) {
      PhysicsComponent physicsComponent = elementUpdater.elem().physicsComponent();
      physicsComponent.step(elementUpdater);
      logger.debug("Stepped physics for {}", elementUpdater.elem().physicsComponent().id);
    }

    @Override
    public void done() {

    }
  }

  public static class SaveVisibleEntities implements SpatialJoinVisitor<HasPhysics, HasPhysics> {

    private ConcurrentMap<Integer, ConcurrentMap<HashSerializeEntity, Boolean>> visibleEntities;

    public SaveVisibleEntities(ConcurrentMap<Integer, ConcurrentMap<HashSerializeEntity, Boolean>> visibleEntities) {
      this.visibleEntities = visibleEntities;
    }

    private void registerVisibleEntity(HasCamera cam, HashSerializeEntity visible) {
      Integer cameraId = cam.cameraComponent().id;
      visibleEntities.putIfAbsent(cameraId, new ConcurrentHashMap<>());
      visibleEntities.get(cameraId).put(visible, true);
    }

    @Override
    public void visit(HasPhysics o1, SpatialToken spatialToken, HasPhysics o2, SpatialToken spatialToken1) {

      boolean cam1sees2 = (o1 instanceof HasCamera) && (o2 instanceof HashSerializeEntity);

      if (cam1sees2) {
        registerVisibleEntity((HasCamera)o1, (HashSerializeEntity) o2);
      }

      boolean cam2sees1 = (o1 instanceof HashSerializeEntity) && (o2 instanceof HasCamera);

      if (cam2sees1) {
        registerVisibleEntity((HasCamera)o2, (HashSerializeEntity) o1);
      }

    }
  }
}
