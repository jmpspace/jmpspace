package com.jmpspace.server.game.ecs;

abstract public class SystemImpl<System> {

  abstract void register(ComponentBase<System> component);

  abstract void stepSystem();

}
