package com.jmpspace.server.game;

import com.vividsolutions.jts.geom.Coordinate;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.util.GeometricShapeFactory;

public class PlayerUtils {

  static final Geometry playerGeometry = playerGeometryOneoff();

  private static final double playerDiameter = 0.3;

  private static Geometry playerGeometryOneoff() {
    GeometricShapeFactory factory = new GeometricShapeFactory();
    factory.setCentre(new Coordinate(0.0, 0.0));
    factory.setHeight(playerDiameter);
    factory.setWidth(playerDiameter);
    return factory.createCircle();
  }

}
