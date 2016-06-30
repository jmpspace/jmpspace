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

@WebActor(webSocketUrlPatterns = {"/"})
public class PlayerClientActor extends BasicActor<WebMessage, Void> {

  static final Logger logger = LogManager.getLogger(PlayerClientActor.class.getName());

  static final ConcurrentMap<String, ActorRef<WebMessage>> activePlayerNames = new ConcurrentHashMap<String, ActorRef<WebMessage>>();

  // There is one actor for each client
  private static final Set<ActorRef<WebMessage>> actors =
    Collections.newSetFromMap(new ConcurrentHashMap<ActorRef<WebMessage>, Boolean>());

  // The client representation of this actor
  private SendPort<WebDataMessage> peer;

  enum PlayerClientState {
    Unauthenticated,
    LoggedIn
  }

  class PlayerClientStateModel {

    private PlayerClientState state = PlayerClientState.Unauthenticated;

    void handleMessage(WebDataMessage message) throws InterruptedException, SuspendExecution {
      if (!message.isBinary()) {
        logger.warn("Receieved a non-binary message, ignoring");
        return;
      }

      ByteString buf = ByteString.copyFrom(message.getByteBufferBody());

      try {

        Server.Request request = Server.Request.parseFrom(buf);
        Server.Response.Builder response = Server.Response.newBuilder();

        if (request.getRequestCase() == Server.Request.RequestCase.SESSIONSTATE) {
          Session.SessionStateResponse.Builder sessionStateResponse = Session.SessionStateResponse.newBuilder();
          switch (state) {
            case Unauthenticated:
              sessionStateResponse.setUnauthenticated(Session.SessionStateUnauthenticated.newBuilder());
              break;
            case LoggedIn:
              sessionStateResponse.setActive(Session.SessionStateActive.newBuilder());
              break;
          }
        } else {

          switch (state) {
            case Unauthenticated:

              switch (request.getRequestCase()) {
                case LOGIN:
                  Session.LoginRequest loginRequest = request.getLogin();
                  String requestedPlayerName = loginRequest.getPlayerName();
                  Session.LoginResponse.Builder loginResponse = Session.LoginResponse.newBuilder();
                  ActorRef<WebMessage> existingPlayer = activePlayerNames.putIfAbsent(requestedPlayerName, self());
                  if (existingPlayer != null) {
                    String error = String.format("Requested player name '%s' is already active", requestedPlayerName);
                    logger.debug(error);
                    loginResponse.setFailure(Session.LoginFailure.newBuilder().setError(error));
                  } else {
                    logger.info(String.format("Logging in: %s", loginRequest.getPlayerName()));
                    // TODO: player ID will be needed soon
                    loginResponse.setSuccess(Session.LoginSuccess.newBuilder().setPlayerId(0).setPlayerName(requestedPlayerName));
                    state = LoggedIn;
                  }
                  response.setLogin(loginResponse);
              }
              break;
            case LoggedIn:
              switch (request.getRequestCase()) {
              }
          }

        }

        postMessage(new WebDataMessage(self(), response.build().toByteString().asReadOnlyByteBuffer()));

      } catch (InvalidProtocolBufferException e) {
        logger.error("Failed to parse something probably", e);
      }

      //postMessage(message);
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

          SendPort<WebDataMessage> p = msg.getFrom();
          this.peer = p;

          // hello message can go here
        }
        else if (message instanceof WebDataMessage) {
          logger.debug("Just a regular message");
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
      for (final SendPort actor : actors)
        if (actor != self())
          //noinspection unchecked
          actor.send(webDataMessage);
  }

  @Override
  protected final WebMessage handleLifecycleMessage(LifecycleMessage m) {
    // while listeners might contain an SSE actor wrapped with Channels.map, the wrapped SendPort maintains the original actors hashCode and equals behavior
    if (m instanceof ExitMessage)
      actors.remove(((ExitMessage) m).getActor());
    return super.handleLifecycleMessage(m);
  }
}
