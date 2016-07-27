package com.jmpspace.server;

import co.paralleluniverse.actors.*;
import co.paralleluniverse.comsat.webactors.*;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.strands.channels.SendPort;
import com.google.protobuf.ByteString;
import com.google.protobuf.InvalidProtocolBufferException;
import com.jmpspace.contracts.SpaceServer.Server;
import com.jmpspace.contracts.SpaceServer.Session;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.util.Collections;
import java.util.Set;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.ConcurrentHashMap;

import static com.jmpspace.server.PlayerClientActor.PlayerClientState.LoggedIn;
import static io.undertow.websockets.core.WebSocketFrameType.PING;

//@SuppressWarnings("WeakerAccess")
@WebActor(webSocketUrlPatterns = {"/"})
public class PlayerClientActor extends BasicActor<Object, Void> {

  private static final Logger logger = LogManager.getLogger(PlayerClientActor.class.getName());

  private static final ConcurrentMap<String, ActorRef<Object>> activePlayers = new ConcurrentHashMap<>();

  // There is one actor for each client
  private static final Set<ActorRef<Object>> actors =
    Collections.newSetFromMap(new ConcurrentHashMap<ActorRef<Object>, Boolean>());

  // The client representation of this actor
  private SendPort<WebDataMessage> peer;

  enum PlayerClientState {
    Unauthenticated,
    ForceWait,
    LoggedIn
  }

  private class ForceLogoff {
    ActorRef<Object> forcingActor;
    ForceLogoff(ActorRef<Object> forcingActor) {
      this.forcingActor = forcingActor;
    }
  }

  Session.LoginRequest forceWaitLoginRequest;

  private class ForceLogoffComplete {}

  private class PlayerClientStateModel {

    private PlayerClientState state = PlayerClientState.Unauthenticated;
    private String playerName = null;
    private int playerId = -1; // TODO: faking it

    void handleForceLoginRetry(Session.LoginRequest loginRequest) throws InterruptedException, SuspendExecution {
      Server.Response.Builder response = Server.Response.newBuilder();
      boolean isResponseBuilt = false;
        String requestedPlayerName = loginRequest.getPlayerName();
        ActorRef<Object> existingPlayer = activePlayers.putIfAbsent(requestedPlayerName, self());
        if (existingPlayer != null) {
          String error = String.format("Requested player name '%s' is already active", requestedPlayerName);
          if (loginRequest.getForceLogin()) {
            state = PlayerClientState.ForceWait;
            forceWaitLoginRequest = loginRequest;
            existingPlayer.send(new ForceLogoff(self()));
          } else {
            logger.debug(error);
            state = PlayerClientState.Unauthenticated;
            response.setUnauthenticated(Session.Unauthenticated.newBuilder().setError(error));
            isResponseBuilt = true;
          }
        } else {
          logger.info(String.format("Logging in: %s", loginRequest.getPlayerName()));
          playerName = requestedPlayerName;
          playerId = -1; // TODO: faking it
          response.setLoggedIn(Session.LoggedIn
                  .newBuilder()
                  .setPlayerName(playerName)
                  .setPlayerId(playerId));
          isResponseBuilt = true;
          state = LoggedIn;

      }
      if (isResponseBuilt) {
        postMessage(new WebDataMessage(self(), response.build().toByteString().asReadOnlyByteBuffer()));
      }
    }

    void handleWebDataMessage(WebDataMessage message) throws InterruptedException, SuspendExecution {
      if (!message.isBinary()) {
        logger.warn("Receieved a non-binary message, ignoring");
        return;
      }

      ByteString buf = ByteString.copyFrom(message.getByteBufferBody());

      try {

        boolean isResponseBuilt = false;
        Server.Request request = Server.Request.parseFrom(buf);
        Server.Response.Builder response = Server.Response.newBuilder();

        if (request.getRequestCase() == Server.Request.RequestCase.SESSIONSTATE) {

          // Disregarding state, session state requests are always answered as disconnects / refreshes can happen

          switch (state) {
            case Unauthenticated:
              response.setUnauthenticated(Session.Unauthenticated.newBuilder());
              isResponseBuilt = true;
              break;
            case LoggedIn:
              response.setLoggedIn(Session.LoggedIn
                      .newBuilder()
                      .setPlayerName(playerName)
                      .setPlayerId(playerId));
              isResponseBuilt = true;
              break;
          }

        } else {

          switch (state) {

            case Unauthenticated:
              switch (request.getRequestCase()) {

                case SESSIONSTATE:
                  break;
                case LOGIN:
                  Session.LoginRequest loginRequest = request.getLogin();
                  String requestedPlayerName = loginRequest.getPlayerName();
                  ActorRef<Object> existingPlayer = activePlayers.putIfAbsent(requestedPlayerName, self());
                  if (existingPlayer != null) {
                    String error = String.format("Requested player name '%s' is already active", requestedPlayerName);
                    if (loginRequest.getForceLogin()) {
                      state = PlayerClientState.ForceWait;
                      forceWaitLoginRequest = loginRequest;
                      existingPlayer.send(new ForceLogoff(self()));
                    } else {
                      logger.debug(error);
                      state = PlayerClientState.Unauthenticated;
                      response.setUnauthenticated(Session.Unauthenticated.newBuilder().setError(error));
                      isResponseBuilt = true;
                    }
                  } else {
                    logger.info(String.format("Logging in: %s", loginRequest.getPlayerName()));
                    playerName = requestedPlayerName;
                    playerId = -1; // TODO: faking it
                    response.setLoggedIn(Session.LoggedIn
                            .newBuilder()
                            .setPlayerName(playerName)
                            .setPlayerId(playerId));
                    isResponseBuilt = true;
                    state = LoggedIn;
                  }
                  break;

                case LOGOUT:
                  break;
                case REQUEST_NOT_SET:
                  break;
              }
              break;

            case LoggedIn:
              switch (request.getRequestCase()) {
                case SESSIONSTATE:
                  break;
                case LOGIN:
                  break;
                case LOGOUT:
                  break;
                case PING:
                  request.getPing();
                  response.setPong(Session.Pong.newBuilder());
                  isResponseBuilt = true;
                case REQUEST_NOT_SET:
                  break;
              }
              break;

          }

        }

        if (isResponseBuilt) {
          postMessage(new WebDataMessage(self(), response.build().toByteString().asReadOnlyByteBuffer()));
        }

      } catch (InvalidProtocolBufferException e) {
        logger.error("Failed to parse something probably", e);
      }

    }

  }

  private PlayerClientStateModel clientState = new PlayerClientStateModel();

  @Override
  protected final Void doRun() throws InterruptedException, SuspendExecution {
    actors.add(self());
    try {
      //noinspection InfiniteLoopStatement
      for (;;) {
        final Object message = receive();
        logger.debug("Got a message");

        if (message instanceof WebSocketOpened) {
          logger.info("Opened client socket");

          final WebSocketOpened msg = (WebSocketOpened) message;
          watch(msg.getFrom()); // will call handleLifecycleMessage with ExitMessage when the session ends

          this.peer = msg.getFrom();

          // hello message can go here
        }
        else if (message instanceof WebDataMessage) {
          logger.debug("Web data message");
          final WebDataMessage msg = (WebDataMessage) message;
          clientState.handleWebDataMessage(msg);
        }
        else if (message instanceof ForceLogoff) {
          ActorRef<Object> activePlayer = activePlayers.get(clientState.playerName);
          if (activePlayer != null && activePlayer == self()) {
            clientState.state = PlayerClientState.Unauthenticated;
            logger.info("Removing self from active player");
            activePlayers.remove(clientState.playerName);
            logger.info("Notifying client of kick");
            Server.Response.Builder response = Server.Response.newBuilder();
            response.setUnauthenticated(Session.Unauthenticated.newBuilder().setError("Forced off"));
            postMessage(new WebDataMessage(self(), response.build().toByteString().asReadOnlyByteBuffer()));
            logger.info("Notifying forcing actor to retry");
            ((ForceLogoff) message).forcingActor.send(new ForceLogoffComplete());
          }
        }
        else if (clientState.state == PlayerClientState.ForceWait && message instanceof ForceLogoffComplete) {
          logger.info("Retrying login");
          clientState.handleForceLoginRetry(forceWaitLoginRequest);
        }
      }
    } finally {
      actors.remove(self());
    }
  }

  private void postMessage(final WebDataMessage webDataMessage) throws InterruptedException, SuspendExecution {
    if (peer != null)
      peer.send(webDataMessage);
//    if (webDataMessage.getFrom().equals(peer))
//      for (final SendPort<Object> actor : actors)
//        if (actor != self())
//          noinspection unchecked
//          actor.send(webDataMessage);
  }

  @Override
  protected final Object handleLifecycleMessage(LifecycleMessage m) {
    // while listeners might contain an SSE actor wrapped with Channels.map, the wrapped SendPort maintains the original actors hashCode and equals behavior
    if (m instanceof ExitMessage) {
      logger.info("Exit");
      actors.remove(((ExitMessage) m).getActor());
      if (clientState.state == LoggedIn && clientState.playerName != null) {
        activePlayers.remove(clientState.playerName);
      }
    }
    if (m instanceof ShutdownMessage) {
      logger.info("Shutdown");
    }
    return super.handleLifecycleMessage(m);
  }
}
