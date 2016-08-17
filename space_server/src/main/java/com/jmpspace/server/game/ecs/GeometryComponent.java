package com.jmpspace.server.game.ecs;

import com.vividsolutions.jts.geom.Geometry;

public class GeometryComponent extends ComponentBase<GeometryComponent.GeometrySystem> {

  static class GeometrySystem {}

  public Geometry geometry;

  public GeometryComponent(Entity entity, Geometry geometry) {
    super(entity);
    this.geometry = geometry;
  }

}
