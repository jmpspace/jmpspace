package com.jmpspace.server.game;

import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.*;

import java.util.concurrent.atomic.AtomicInteger;

public abstract class PhysicsRef {

  private static AtomicInteger identityGenerator = new AtomicInteger(0);
  private int id;
  public int getId() {
    return id;
  }

  protected PhysicsRef() {
    id = identityGenerator.getAndIncrement();
  }

  public enum PhysicsStepType { Static, Floating, Attached }
  public abstract PhysicsStepType get_physicsType();
  public abstract void stepPhysics(ElementUpdater<PhysicsRef> base);

  public boolean get_hasPlayerCamera() {
    return false;
  }

  private SpatialToken _token;

  abstract AABB calculateBounds();

  abstract void notifyOwner() throws SuspendExecution, InterruptedException;

  public SpatialToken get_token() {
    return _token;
  }

  public void set_token(SpatialToken _token) {
    this._token = _token;
  }
}
