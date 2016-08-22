package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.SpaceBase;
import co.paralleluniverse.spacebase.SpatialToken;
import com.jmpspace.contracts.SpaceServer.Physics.Vector2;
import com.jmpspace.server.game.common.CommonRequest;
import com.jmpspace.server.game.ecs.Entity;
import com.jmpspace.server.game.entities.FloatingStructure;
import com.jmpspace.server.game.entities.Platform;
import com.jmpspace.server.game.entities.PlayerOnBoard;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.util.*;
import java.util.concurrent.atomic.AtomicInteger;

class StructureActor extends BasicActor<StructureActor.Request, Void> {

  private static final Logger logger = LogManager.getLogger(StructureActor.class.getName());

  private FloatingStructure _floatingStructureRef;
  private ActorRef<Instance.Request> _instanceRef;
  private SpaceBase<Entity.HasPhysics> spaceBase;
  private List<Platform> platforms;
  private Map<Integer, CryoTubeAddress> cryoTubeAddresses = new HashMap<>();

//  static class PlatformWrapper {
//    int platformId;
//    StructureOuterClass.Platform platform;
//    AffineTransformation platformRelativeLocation;
//    FloatingStructure floatingStructureRef;
//
//    public PlatformWrapper(int platformId, StructureOuterClass.Platform platform, AffineTransformation platformRelativeLocation, FloatingStructure floatingStructureRef) {
//      this.platformId = platformId;
//      this.platform = platform;
//      this.platformRelativeLocation = platformRelativeLocation;
//      this.floatingStructureRef = floatingStructureRef;
//    }
//  }

  private class CryoTubeAddress {
    int id;
    Platform platform;
    Vector2 position;

    public CryoTubeAddress(int id, Platform platform, Vector2 position) {
      this.id = id;
      this.platform = platform;
      this.position = position;
    }
  }

  StructureActor(FloatingStructure floatingStructureRef, ActorRef<Instance.Request> instanceRef, SpaceBase<Entity.HasPhysics> spaceBase) {
    _floatingStructureRef = floatingStructureRef;
    _instanceRef = instanceRef;
    this.spaceBase = spaceBase;
    platforms = StructureUtils.findPlatforms(floatingStructureRef);

//    List<CryoTubeAddress> cryoTubeAddresses = new ArrayList<>();
    AtomicInteger cryoTubeCounter = new AtomicInteger();

    platforms.forEach(platform -> {

      spaceBase.insert(platform, platform.physicsComponent().calculateAABB());

      platform.platformPart
              .getPlacedItemsList().stream()
              .filter(placedItem -> placedItem.getItem().hasCryoTube())
              .forEach(placedCryoTube -> {
                int id = cryoTubeCounter.getAndIncrement();
                Vector2 position = Vector2
                        .newBuilder()
                        .setX(placedCryoTube.getOffsetX())
                        .setY(placedCryoTube.getOffsetY())
                        .build();
                cryoTubeAddresses.put(
                        id,
                        new CryoTubeAddress(id, platform, position));
              });

    });
  }

  // Note: probably the responsibility of Platform now
  private Map<ActorRef<Player.Request>, PlayerOnBoard> _playersOnBoard = new HashMap<>();

//  abstract static class State {
//    ActorRef<StructureActor.Request> _owner;
//  }

  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {

    logger.info("Registering {} cryo tubes", cryoTubeAddresses.size());

    _instanceRef.send(new Instance.RegisterCryoTubes(self(), cryoTubeAddresses.keySet()));

    //noinspection InfiniteLoopStatement
    for (;;) {
      Request message = receive();

      if (message instanceof Spawn) {

        Spawn spawn = (Spawn) message;
        ActorRef<Player.Request> player = ((Spawn) message)._player;

        assert ! _playersOnBoard.containsKey(player);

        CryoTubeAddress cryoTubeAddress = cryoTubeAddresses.get(spawn._cryoTubeId);

        Platform platform = cryoTubeAddress.platform;

        Vector2 position = cryoTubeAddress.position;

        logger.info("Spawning player {} on platform {}", player, platform);

        PlayerOnBoard playerOnBoard = new PlayerOnBoard(platform, position, player);
        _playersOnBoard.put(player, playerOnBoard);

        SpatialToken playerOnBoardToken = spaceBase.insert(playerOnBoard, playerOnBoard.physicsComponent().calculateAABB());

        player.send(new Player.Spawned(playerOnBoard));

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
    private int _cryoTubeId;

    Spawn(ActorRef<Player.Request> player, int cryoTubeId) {
      _player = player;
      _cryoTubeId = cryoTubeId;
    }
  }
}
