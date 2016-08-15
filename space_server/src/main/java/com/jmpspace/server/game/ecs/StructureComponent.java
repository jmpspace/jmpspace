package com.jmpspace.server.game.ecs;

import com.jmpspace.contracts.SpaceServer.StructureOuterClass.StructureNode;

public class StructureComponent extends ComponentBase<Void> {

  public StructureNode tree;

  public StructureComponent(Entity entity, StructureNode tree) {
    super(entity);
    this.tree = tree;
  }

}
