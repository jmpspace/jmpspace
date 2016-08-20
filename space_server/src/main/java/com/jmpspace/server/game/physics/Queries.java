package com.jmpspace.server.game.physics;

import co.paralleluniverse.db.tree.QueryResult;
import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.SpatialQuery;
import co.paralleluniverse.spacebase.queries.DistanceJoin;
import com.jmpspace.server.game.PhysicsRef;
import com.jmpspace.server.game.ecs.Entity;
import com.jmpspace.server.game.ecs.Entity.HasCamera;
import com.jmpspace.server.game.ecs.Entity.HasPhysics;
import com.jmpspace.server.game.ecs.Entity.HashSerializeEntity;
import com.jmpspace.server.game.ecs.PhysicsComponent;
import com.jmpspace.server.game.ecs.PhysicsComponent.PhysicsStepType;

public class Queries {

  public static class AllOfPhysicsStepType implements SpatialQuery<HasPhysics> {

    private PhysicsStepType physicsStepType;

    public AllOfPhysicsStepType(PhysicsStepType physicsStepType) {
      this.physicsStepType = physicsStepType;
    }

    @Override
    public QueryResult queryContainer(AABB aabb) {
      return QueryResult.SOME;
    }

    @Override
    public boolean queryElement(AABB aabb, HasPhysics physicsRef) {
      return physicsRef.physicsComponent().stepType == physicsStepType;
    }
  }

  public static class PlayerVisibility extends DistanceJoin<HasPhysics, HasPhysics> {

    private static final int playerVisbilityDistance = 50;

    public PlayerVisibility() {
      super(playerVisbilityDistance);
    }

    @Override
    public boolean joinElements(AABB b1, HasPhysics playerRef, AABB b2, HasPhysics other) {
      boolean valid = (playerRef instanceof HasCamera) && (other instanceof HashSerializeEntity);
      return valid;
    }
  }

}
