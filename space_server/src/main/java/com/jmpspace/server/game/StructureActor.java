package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.ElementUpdater;
import co.paralleluniverse.spacebase.quasar.SpaceBase;
import co.paralleluniverse.spacebase.SpatialToken;
import com.jmpspace.contracts.SpaceServer.Physics;
import com.jmpspace.contracts.SpaceServer.Physics.Vector2;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass.Platform;
import com.jmpspace.contracts.SpaceServer.StructureOuterClass.StructureNode;
import com.jmpspace.server.game.common.CommonRequest;
import com.jmpspace.server.game.ecs.*;
import com.jmpspace.server.game.ecs.Entity;
import com.jmpspace.server.game.entities.FloatingStructure;
import com.vividsolutions.jts.geom.Envelope;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.util.AffineTransformation;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.util.*;
import java.util.concurrent.atomic.AtomicInteger;

import static com.jmpspace.contracts.SpaceServer.WorldOuterClass.*;

class StructureActor extends BasicActor<StructureActor.Request, Void> {

  private static final Logger logger = LogManager.getLogger(StructureActor.class.getName());

  private FloatingStructure _floatingStructureRef;
  private ActorRef<Instance.Request> _instanceRef;
  private SpaceBase<Entity.HasPhysics> _playersBase;
  private List<PlatformWrapper> _platforms;
  private Map<UUID, CryoTubeAddress> _cryoTubes = new HashMap<>();

  static class PlatformWrapper {
    int platformId;
    Platform platform;
    AffineTransformation platformRelativeLocation;
    FloatingStructure floatingStructureRef;

    public PlatformWrapper(int platformId, Platform platform, AffineTransformation platformRelativeLocation, FloatingStructure floatingStructureRef) {
      this.platformId = platformId;
      this.platform = platform;
      this.platformRelativeLocation = platformRelativeLocation;
      this.floatingStructureRef = floatingStructureRef;
    }
  }

  private class CryoTubeAddress {
    int id;
    PlatformWrapper platform;

    public CryoTubeAddress(int id, PlatformWrapper platform) {
      this.id = id;
      this.platform = platform;
    }
  }

  StructureActor(FloatingStructure floatingStructureRef, ActorRef<Instance.Request> instanceRef, SpaceBase<Entity.HasPhysics> playersBase) {
    _floatingStructureRef = floatingStructureRef;
    _instanceRef = instanceRef;
    _playersBase = playersBase;
    _platforms = StructureUtils.findPlatforms(floatingStructureRef);

    List<CryoTubeAddress> cryoTubeAddresses = new ArrayList<>();
    AtomicInteger cryoTubeCounter = new AtomicInteger();

    _platforms.forEach(platform -> {
      platform.platform
              .getPlacedItemsList().stream()
              .filter(placedItem -> placedItem.getItem().hasCryoTube())
              .forEach(placedCryoTube -> {
                int id = cryoTubeCounter.getAndIncrement();
                cryoTubeAddresses.add(new CryoTubeAddress(id, platform));
              });
    });
  }

  public static class PlayerOnBoard extends PhysicsRef {
    private ActorRef<Player.Request> _player;
    private PlatformWrapper _platform;
    private Vector2 _position;

    PlayerOnBoard(ActorRef<Player.Request> player, PlatformWrapper platform, Vector2 position) {
      super();
      _player = player;
      _platform = platform;
      _position = position;
    }

    @Override
    AABB calculateBounds() {
      AffineTransformation transform = new AffineTransformation();
      transform.translate(_position.getX(), _position.getY());
      transform.composeBefore(_platform.platformRelativeLocation);
      transform.composeBefore(_platform.floatingStructureRef.absoluteTransform());
      Geometry playerGeometry = (Geometry)PlayerUtils.playerGeometry.clone();
      playerGeometry.apply(transform);
      Envelope bounds = playerGeometry.getEnvelopeInternal();
      return AABB.create(bounds.getMinX(), bounds.getMaxX(), bounds.getMinY(), bounds.getMaxY());
    }

    @Override
    public PhysicsStepType get_physicsType() {
      return PhysicsStepType.Attached;
    }

    @Override
    public void stepPhysics(ElementUpdater<PhysicsRef> base) {

    }

    @Override
    void notifyOwner() throws SuspendExecution, InterruptedException {

    }

    @Override
    public boolean get_hasPlayerCamera() {
      return true;
    }
  }

  private Map<ActorRef<Player.Request>, PlayerOnBoard> _playersOnBoard = new HashMap<>();

//  abstract static class State {
//    ActorRef<StructureActor.Request> _owner;
//  }

  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {

    logger.info("Registering {} cryo tubes", _cryoTubes.size());

    _instanceRef.send(new Instance.RegisterCryoTubes(self(), _cryoTubes.keySet()));

    //noinspection InfiniteLoopStatement
    for (;;) {
      Request message = receive();

      if (message instanceof Spawn) {

        Spawn spawn = (Spawn) message;
        ActorRef<Player.Request> player = ((Spawn) message)._player;

        assert ! _playersOnBoard.containsKey(player);

        CryoTubeAddress cryoTubeAddress = _cryoTubes.get(spawn._cryoTubeId);

        PlatformWrapper platform = cryoTubeAddress.platform;

        Vector2 position = Vector2.getDefaultInstance();

        logger.info("Spawning player {} on platform {}", player, platform);

        PlayerOnBoard playerOnBoard = new PlayerOnBoard(player, platform, position);
        _playersOnBoard.put(player, playerOnBoard);

        SpatialToken playerOnBoardToken = _playersBase.insert(playerOnBoard, playerOnBoard.calculateBounds());

        playerOnBoard.set_token(playerOnBoardToken);

        // TODO: send a message back to the Player? Or start a stream of messages to each player on board with position?

      }
    }

  }

  static abstract class Request extends CommonRequest {}

//  class Board extends Request {
//    PlayerOnBoard player;
//    List<Integer> platformPath;
//  }

  static class Spawn extends Request {

    private ActorRef<Player.Request> _player;
    private UUID _cryoTubeId;

    Spawn(ActorRef<Player.Request> player, UUID cryoTubeId) {
      _player = player;
      _cryoTubeId = cryoTubeId;
    }
  }
}
