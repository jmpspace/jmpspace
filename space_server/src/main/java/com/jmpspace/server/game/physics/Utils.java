package com.jmpspace.server.game.physics;

import com.jmpspace.contracts.SpaceServer.Physics;
import com.jmpspace.contracts.SpaceServer.Physics.PhysicsState;
import com.vividsolutions.jts.geom.util.AffineTransformation;

public class Utils {

  public static AffineTransformation absoluteTransform(Physics.PhysicsStateOrBuilder physicsState) {
    Physics.Vector2 position = physicsState.getPosition();
    AffineTransformation floatingTransform = new AffineTransformation();
    floatingTransform.rotate(physicsState.getOrientation());
    floatingTransform.translate(position.getX(), position.getY());
    return floatingTransform;
  }
}
