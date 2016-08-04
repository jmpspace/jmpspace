package com.jmpspace.server.game;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.spacebase.AABB;
import co.paralleluniverse.spacebase.SpaceBase;
import com.jmpspace.contracts.SpaceServer.Structure.Platform;
import com.jmpspace.server.game.common.CommonRequest;
import com.vividsolutions.jts.geom.Envelope;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.util.AffineTransformation;
import org.jooq.lambda.Seq;

import java.util.*;

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

  static class FloatingStructureRef extends PhysicsRef {

    FloatingStructure _floatingStructure;
    Geometry _staticGeometry;

    ActorRef<Request> _owner;

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
    void step(SpaceBase<PhysicsRef> base) throws InterruptedException, SuspendExecution {
      // TODO
    }

  }

  private FloatingStructureRef _floatingStructureRef;
  private Geometry _geom;

  static class PlayerOnBoard {
    private ActorRef<Player.Request> _player;
    private List<Integer> _platformAddress;
    private Platform _platform;
    private Vector2 _position;

    public PlayerOnBoard(ActorRef<Player.Request> player, List<Integer> platformAddress, Platform platform, Vector2 position) {
      _player = player;
      _platformAddress = platformAddress;
      _platform = platform;
      _position = position;
    }
  }

  private Map<ActorRef<Player.Request>, PlayerOnBoard> _playersOnBoard = new HashMap<>();

  abstract static class State {
    ActorRef<StructureActor.Request> _owner;
  }

  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {

    _instanceRef.send(new Instance.RegisterCryoTubes(self(), cryoTubes.keySet()));

    for (;;) {
      Request message = (Request)receive();

      if (message instanceof Spawn) {

        Spawn spawn = (Spawn) message;
        ActorRef<Player.Request> player = ((Spawn) message)._player;

        assert ! _playersOnBoard.containsKey(player);

        CryoTubeAddress cryoTube = cryoTubes.get(spawn._cryoTubeId);

        Platform platform = Seq.foldLeft(
                cryoTube._address,
                _floatingStructureRef._floatingStructure.getStructure(),
                (node, i) -> node.getAttachments(i).getNode()
                ).getPart().getPlatform();

        Vector2 position = Vector2.getDefaultInstance();

        PlayerOnBoard playerOnBoard = new PlayerOnBoard(player, cryoTube._address, platform, position);
        _playersOnBoard.put(player, playerOnBoard);

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
