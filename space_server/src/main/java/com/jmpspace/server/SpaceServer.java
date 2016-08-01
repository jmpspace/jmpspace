package com.jmpspace.server;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.comsat.webactors.undertow.AutoWebActorHandler;
import com.jmpspace.server.game.Instance;
import io.undertow.Undertow;
import io.undertow.server.session.InMemorySessionManager;
import io.undertow.server.session.SessionAttachmentHandler;
import io.undertow.server.session.SessionCookieConfig;
import io.undertow.server.session.SessionManager;
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;

import java.util.HashMap;
import java.util.Map;

class SpaceServer {

  private static final Logger logger = LogManager.getLogger(SpaceServer.class.getName());

  private static final int port = 8001; // TODO configure magic number

  static public void main(String[] args) {

    logger.debug("Starting server fiber");

    final SessionManager sessionManager = new InMemorySessionManager("SESSION_MANAGER", 100, true);
    final SessionCookieConfig sessionConfig = new SessionCookieConfig();
    sessionConfig.setMaxAge(60 * 5);
    final SessionAttachmentHandler sessionAttachmentHandler =
          new SessionAttachmentHandler(sessionManager, sessionConfig);

    ActorRef<Instance.Request> instanceRef = (new Instance()).spawn();

    Map<Class<?>, Object[]> classParams = new HashMap<>();
    classParams.put(PlayerClientActor.class, new Object[]{ instanceRef });

    AutoWebActorHandler handler = new AutoWebActorHandler(classParams);

    Undertow server = Undertow.builder().addHttpListener(port, "localhost")
           .setHandler(sessionAttachmentHandler.setNext(handler)).build();


    server.start();

  }
}
