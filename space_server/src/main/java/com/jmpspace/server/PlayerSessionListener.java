package com.jmpspace.server;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.comsat.webactors.WebMessage;
import co.paralleluniverse.comsat.webactors.undertow.WebActorHandler;
import io.undertow.server.HttpServerExchange;
import io.undertow.server.session.Session;
import io.undertow.server.session.SessionListener;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

/**
 * Created by John on 7/4/2016.
 */
public class PlayerSessionListener implements SessionListener {

  protected static final String ACTOR_KEY = "co.paralleluniverse.comsat.webactors.sessionActor";

  private static final Logger logger = LogManager.getLogger(PlayerSessionListener.class.getName());

  @Override
  public void sessionCreated(Session session, HttpServerExchange exchange) {

  }

  @Override
  public void sessionDestroyed(Session session, HttpServerExchange exchange, SessionDestroyedReason reason) {
    logger.info("Destroyed");
    WebActorHandler.Context context = (WebActorHandler.Context) session.getAttribute(ACTOR_KEY);
    ActorRef<? extends WebMessage> actor = context.getWebActor();
//    actor;
  }

  @Override
  public void attributeAdded(Session session, String name, Object value) {

  }

  @Override
  public void attributeUpdated(Session session, String name, Object newValue, Object oldValue) {

  }

  @Override
  public void attributeRemoved(Session session, String name, Object oldValue) {

  }

  @Override
  public void sessionIdChanged(Session session, String oldSessionId) {

  }
}
