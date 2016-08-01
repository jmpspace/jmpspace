package com.jmpspace.server.game.physics;

import co.paralleluniverse.spacebase.AABB;
import com.vividsolutions.jts.geom.Coordinate;
import com.vividsolutions.jts.geom.Envelope;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.util.AffineTransformation;

public class RigidObject {

  // Timeless
  private Geometry _geom;

  // First order changing values
  private Coordinate _position;
  private double _rotation; // Radians

  // Second order changing values
  private Coordinate _velocity;
  private double _rotVelocity;

  RigidObject(Geometry geom, Coordinate position, double rotation, Coordinate velocity, double rotVelocity) {
    _geom = geom;
    _position = position;
    _rotation = rotation;
    _velocity = velocity;
    _rotVelocity = rotVelocity;
  }

  RigidObject(Geometry geom, Coordinate position, double rotation) {
    this(geom, position, rotation, new Coordinate(0,0), 0);
  }

  AABB calculateAABB() {
    AffineTransformation tranform = new AffineTransformation();
    tranform.compose(AffineTransformation.rotationInstance(_rotation));
    tranform.compose(AffineTransformation.translationInstance(_position.x, _position.y));
    Envelope envelope = _geom.getEnvelopeInternal();
    return AABB.create(envelope.getMinX(), envelope.getMaxX(), envelope.getMinY(), envelope.getMaxY());
  }

  private void scratch() {

  }

}
