package com.jmpspace.server.game.ecs;

abstract class ComponentBase<System> {

  Entity entity;

  ComponentBase(Entity entity) {
    this.entity = entity;
  }

}
