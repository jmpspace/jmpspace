package com.jmpspace.server.game;

import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.SpaceBase;
import co.paralleluniverse.spacebase.SpatialQueries;
import co.paralleluniverse.spacebase.SpatialToken;

abstract class PhysicsRef {

  private SpatialToken _token;

  abstract void step(SpaceBase<PhysicsRef> base);

  public SpatialToken get_token() {
    return _token;
  }

  public void set_token(SpatialToken _token) {
    this._token = _token;
  }
}
