package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.SpaceBase;
import com.vividsolutions.jts.geom.Envelope;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.util.AffineTransformation;

import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.UUID;
import java.util.concurrent.ConcurrentMap;

import static com.jmpspace.contracts.SpaceServer.WorldOuterClass.*;

public class StructureActor extends BasicActor<StructureActor.Request, Void> {

  private ActorRef<Instance.Request> _instanceRef;

  class CryoTubeAddress {
    UUID _uuid;
    List<Integer> _address;

    public CryoTubeAddress(UUID uuid, List<Integer> address) {
      _uuid = uuid;
      _address = address;
    }
  }

  private Map<UUID, CryoTubeAddress> cryoTubes = new HashMap<>();


  public StructureActor(FloatingStructureRef floatingStructureRef, ActorRef<Instance.Request> instanceRef) {
    _floatingStructureRef = floatingStructureRef;
    _instanceRef = instanceRef;

    List<List<Integer>> structureCryoTubes = StructureUtils.findCryoTubes(_floatingStructureRef._floatingStructure.getStructure());

    structureCryoTubes.forEach(cryoTube -> {
      UUID uuid = UUID.randomUUID();
      CryoTubeAddress cryoTubeAddress = new CryoTubeAddress(uuid, cryoTube);
      cryoTubes.put(uuid, cryoTubeAddress);
    });
  }

  class PlayerRef {
    ActorRef<Player.Request> actor;
  }

  static class FloatingStructureRef extends PhysicsRef {

    FloatingStructure _floatingStructure;
    Geometry _staticGeometry;

    public FloatingStructureRef(FloatingStructure floatingStructure) {
      _floatingStructure = floatingStructure;
      _staticGeometry = StructureUtils.calculateStructureGeometry(floatingStructure.getStructure());
    }

    @Override
    AABB calculateBounds() {
      PhysicsState physicsState = _floatingStructure.getPhysicsState();
      Vector2 position = physicsState.getPosition();
      AffineTransformation floatingTransform = new AffineTransformation();
      floatingTransform.rotate(physicsState.getOrientation());
      floatingTransform.translate(position.getX(), position.getY());
      Geometry floatingGeometry = (Geometry)_staticGeometry.clone();
      floatingGeometry.apply(floatingTransform);
      Envelope bounds = floatingGeometry.getEnvelopeInternal();
      return AABB.create(bounds.getMinX(), bounds.getMaxX(), bounds.getMinY(), bounds.getMaxY());
    }

    @Override
    void step(SpaceBase<PhysicsRef> base) {
      // TODO
    }

  }

  private FloatingStructureRef _floatingStructureRef;
  private Geometry _geom;

  private ConcurrentMap<List<Integer>, PlayerRef> _playersOnBoard;


  abstract static class State {
    ActorRef<StructureActor.Request> _owner;
  }


  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {

    _instanceRef.send(new RegisterCryoTubes(cryoTubes.keySet()));

    for (;;) {
      Request message = (Request)receive();
    }

  }

  abstract class Request {}

  class Board extends Request {
    PlayerRef player;
    List<Integer> platformPath;
  }
}
