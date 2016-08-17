package com.jmpspace.server.game.ecs;

import co.paralleluniverse.actors.ActorRef;
import com.jmpspace.server.game.Player;

import java.util.concurrent.atomic.AtomicInteger;

public class CameraComponent extends ComponentBase<Void> {

  static AtomicInteger cameraCounter = new AtomicInteger();

  public Integer id;
  ActorRef<Player.Request> owner;

  public CameraComponent(Entity entity, ActorRef<Player.Request> owner) {
    super(entity);
    this.id = cameraCounter.getAndIncrement();
    this.owner = owner;
  }
}
