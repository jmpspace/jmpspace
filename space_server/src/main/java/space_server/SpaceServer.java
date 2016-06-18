package space_server;

import co.paralleluniverse.comsat.webactors.undertow.AutoWebActorHandler;
import co.paralleluniverse.fibers.Fiber;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.fibers.io.FiberSocketChannel;
import co.paralleluniverse.fibers.io.FiberServerSocketChannel;
import co.paralleluniverse.spacebase.SpaceBase;
import co.paralleluniverse.spacebase.SpaceBaseBuilder;
import co.paralleluniverse.strands.SuspendableRunnable;

import com.google.protobuf.CodedInputStream;
import com.google.protobuf.InvalidProtocolBufferException;
import com.google.protobuf.Parser;

import io.undertow.server.session.InMemorySessionManager;
import io.undertow.server.session.SessionAttachmentHandler;
import io.undertow.server.session.SessionCookieConfig;
import io.undertow.server.session.SessionManager;
import io.undertow.Undertow;

import java.io.InputStream;
import java.io.IOException;
import java.lang.InterruptedException;
import java.net.InetSocketAddress;
import java.net.SocketAddress;
import java.nio.ByteBuffer;
import java.nio.channels.Channels;
import java.nio.channels.GatheringByteChannel;
import java.nio.channels.ScatteringByteChannel;
import java.util.concurrent.ExecutionException;

import org.apache.logging.log4j.Logger;
import org.apache.logging.log4j.LogManager;

import SpaceServer.Session;

class SpaceServer {

  static final Logger logger = LogManager.getLogger(SpaceServer.class.getName());

  static final int port = 8001; // TODO configure magic number
  static final SocketAddress addr = new InetSocketAddress(port);

  static public void main(String[] args) {

    logger.debug("Starting server fiber");

    // FIXME: execution context, parallel or concurrent
    SpaceBaseBuilder builder = new SpaceBaseBuilder().setDimensions(2);
    SpaceBase ambientBase = builder.build("ambient");
    SpaceBase largeBase = builder.build("large");
    SpaceBase smallBase = builder.build("small");
    
    final SessionManager sessionManager = new InMemorySessionManager("SESSION_MANAGER", 1, true);
    final SessionCookieConfig sessionConfig = new SessionCookieConfig();
    sessionConfig.setMaxAge(60);
    final SessionAttachmentHandler sessionAttachmentHandler =
          new SessionAttachmentHandler(sessionManager, sessionConfig);

    Undertow server = Undertow.builder().addHttpListener(port, "localhost")
           .setHandler(sessionAttachmentHandler.setNext(new AutoWebActorHandler())).build();

    server.start();

  }
}
