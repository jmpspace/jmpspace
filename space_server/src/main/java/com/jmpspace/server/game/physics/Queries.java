package com.jmpspace.server.game.physics;

import co.paralleluniverse.db.tree.QueryResult;
import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.SpatialQuery;
import co.paralleluniverse.spacebase.queries.DistanceJoin;
import com.jmpspace.server.game.PhysicsRef;

public class Queries {

  public static class AllOfPhysicsStepType implements SpatialQuery<PhysicsRef> {

    private PhysicsRef.PhysicsStepType physicsStepType;

    public AllOfPhysicsStepType(PhysicsRef.PhysicsStepType physicsStepType) {
      this.physicsStepType = physicsStepType;
    }

    @Override
    public QueryResult queryContainer(AABB aabb) {
      return QueryResult.ALL;
    }

    @Override
    public boolean queryElement(AABB aabb, PhysicsRef physicsRef) {
      return physicsRef.get_physicsType() == physicsStepType;
    }
  }

  public static class PlayerVisibility extends DistanceJoin<PhysicsRef, PhysicsRef> {

    private static final int playerVisbilityDistance = 50;

    public PlayerVisibility() {
      super(playerVisbilityDistance);
    }

    @Override
    public boolean joinElements(AABB b1, PhysicsRef playerRef, AABB b2, PhysicsRef other) {
      return playerRef.get_hasPlayerCamera();
    }
  }

}
