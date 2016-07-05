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

//@SuppressWarnings("WeakerAccess")
@WebActor(webSocketUrlPatterns = {"/"})
public class PlayerClientActor extends BasicActor<WebMessage, Void> {

  private static final Logger logger = LogManager.getLogger(PlayerClientActor.class.getName());

  private static final ConcurrentMap<String, ActorRef<WebMessage>> activePlayerNames = new ConcurrentHashMap<>();

  // There is one actor for each client
  private static final Set<ActorRef<WebMessage>> actors =
    Collections.newSetFromMap(new ConcurrentHashMap<ActorRef<WebMessage>, Boolean>());

  // The client representation of this actor
  private SendPort<WebDataMessage> peer;

  enum PlayerClientState {
    Unauthenticated,
    LoggedIn
  }

  private class PlayerClientStateModel {

    private PlayerClientState state = PlayerClientState.Unauthenticated;
    private String playerName = null;
    private int playerId = -1; // TODO: faking it

    void handleMessage(WebDataMessage message) throws InterruptedException, SuspendExecution {
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
                  ActorRef<WebMessage> existingPlayer = activePlayerNames.putIfAbsent(requestedPlayerName, self());
                  if (existingPlayer != null) {
                    String error = String.format("Requested player name '%s' is already active", requestedPlayerName);
                    logger.debug(error);
                    response.setUnauthenticated(Session.Unauthenticated.newBuilder().setError(error));
                    isResponseBuilt = true;
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
          clientState.handleMessage(msg);
        }
      }
    } finally {
      actors.remove(self());
    }
  }

  private void postMessage(final WebDataMessage webDataMessage) throws InterruptedException, SuspendExecution {
    if (peer != null)
      peer.send(webDataMessage);
    if (webDataMessage.getFrom().equals(peer))
      for (final SendPort<WebMessage> actor : actors)
        if (actor != self())
          //noinspection unchecked
          actor.send(webDataMessage);
  }

  @Override
  protected final WebMessage handleLifecycleMessage(LifecycleMessage m) {
    // while listeners might contain an SSE actor wrapped with Channels.map, the wrapped SendPort maintains the original actors hashCode and equals behavior
    if (m instanceof ExitMessage) {
      logger.info("Exit");
      actors.remove(((ExitMessage) m).getActor());
      if (clientState.state == LoggedIn && clientState.playerName != null) {
        activePlayerNames.remove(clientState.playerName);
      }
    }
    if (m instanceof ShutdownMessage) {
      logger.info("Shutdown");
    }
    return super.handleLifecycleMessage(m);
  }
}
