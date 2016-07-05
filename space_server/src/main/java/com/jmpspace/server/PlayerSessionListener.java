package com.jmpspace.server;

import io.undertow.server.HttpServerExchange;
import io.undertow.server.session.Session;
import io.undertow.server.session.SessionListener;

/**
 * Created by John on 7/4/2016.
 */
public class PlayerSessionListener implements SessionListener {
  @Override
  public void sessionCreated(Session session, HttpServerExchange exchange) {

  }

  @Override
  public void sessionDestroyed(Session session, HttpServerExchange exchange, SessionDestroyedReason reason) {

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
