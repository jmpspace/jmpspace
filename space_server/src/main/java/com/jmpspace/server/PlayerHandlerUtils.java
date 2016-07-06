package com.jmpspace.server;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.comsat.webactors.WebMessage;
import co.paralleluniverse.comsat.webactors.undertow.WebActorHandler;
import io.undertow.server.HttpServerExchange;

import java.util.concurrent.Callable;

/**
 * Created by John on 7/5/2016.
 */
public class PlayerHandlerUtils {

  private static final Callable<WebActorHandler> basicWebActorHandlerCreator = () -> new WebActorHandler(xch -> {

    PlayerClientActor actor = new PlayerClientActor();
    ActorRef<? extends WebMessage> actorRef = actor.spawn();

    return new WebActorHandler.DefaultContextImpl() {
      @Override
      public String getId() {
        return "CONSTANT";
      }

      @SuppressWarnings("unchecked")
      @Override
      public ActorRef<? extends WebMessage> getWebActor() {
        return actorRef;
      }

      @Override
      public void restart(HttpServerExchange r) {
        // Nothing to do
      }

      @Override
      public final boolean handlesWithWebSocket(String uri) {
        return uri.startsWith("/ws");
      }

      @Override
      public WatchPolicy watch() {
        return WatchPolicy.DIE;
      }

      @Override
      public final boolean handlesWithHttp(String uri) {
        return !handlesWithWebSocket(uri);
      }
    };
  });

}
