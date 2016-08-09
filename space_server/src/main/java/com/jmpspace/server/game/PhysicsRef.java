package com.jmpspace.server.game;

import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.fibers.instrument.SuspendablesScanner;
import co.paralleluniverse.spacebase.*;

abstract class PhysicsRef {

  private SpatialToken _token;

  abstract AABB calculateBounds();

  abstract void step(ElementUpdater<PhysicsRef> base);

  abstract void notifyOwner() throws SuspendExecution, InterruptedException;

  public SpatialToken get_token() {
    return _token;
  }

  public void set_token(SpatialToken _token) {
    this._token = _token;
  }
}
