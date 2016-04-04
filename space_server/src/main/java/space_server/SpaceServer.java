package space_server;

import co.paralleluniverse.fibers.Fiber;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.fibers.io.FiberSocketChannel;
import co.paralleluniverse.fibers.io.FiberServerSocketChannel;
import co.paralleluniverse.strands.SuspendableRunnable;

import java.io.InputStream;
import java.io.IOException;
import java.lang.InterruptedException;
import java.net.InetSocketAddress;
import java.net.SocketAddress;
import java.nio.channels.Channels;
import java.util.concurrent.ExecutionException;

import org.apache.logging.log4j.Logger;
import org.apache.logging.log4j.LogManager;

import spaceServer.session.Session.AuthCredential;
import spaceServer.session.Session.AuthRequest;

class SpaceServer {

  static final Logger logger = LogManager.getLogger(SpaceServer.class.getName());

  static final int port = 3000; // TODO configure magic number
  static final SocketAddress addr = new InetSocketAddress(port);

  static void handleClient(FiberSocketChannel clientChannel) throws SuspendExecution {
    logger.debug("Handling client channel");
    try {
      InputStream reader = Channels.newInputStream(clientChannel);
      AuthRequest authReq = AuthRequest.parseFrom(reader);
      AuthCredential authCred = authReq.getCredential();
      String authUsername = authCred.getUsername();
      String authPassword = authCred.getPassword();
      // TODO actually check something :-)
      logger.info("Authenticated:" + authUsername + " (" + authPassword + ")");
    }
    catch (IOException e) {
      e.printStackTrace();
    }
  }

  static public void main(String[] args) {

    logger.debug("Starting server fiber");

    Fiber serverFiber = new Fiber("SERVER", new SuspendableRunnable() {
      @Override public void run() throws SuspendExecution {
        try {
          logger.debug("Opening and binding server channel");
          FiberServerSocketChannel serverChannel = FiberServerSocketChannel.open().bind(addr);

          logger.debug("Starting accept loop");
          while (true) {
            logger.debug("Waiting for next client");
            FiberSocketChannel clientChannel = serverChannel.accept();
            logger.debug("Accepted client channel");
            handleClient(clientChannel);
          }
        }
        catch (IOException e) {
          e.printStackTrace();
        }
      }
    }).start();

    try {
      serverFiber.join();
    } catch (ExecutionException e) {
      e.printStackTrace();
    } catch (InterruptedException e) {
      e.printStackTrace();
    }

  }
}
