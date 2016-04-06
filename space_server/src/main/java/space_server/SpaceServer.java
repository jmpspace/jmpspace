package space_server;

import co.paralleluniverse.fibers.Fiber;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.fibers.io.FiberSocketChannel;
import co.paralleluniverse.fibers.io.FiberServerSocketChannel;
import co.paralleluniverse.strands.SuspendableRunnable;

import com.google.protobuf.CodedInputStream;
import com.google.protobuf.InvalidProtocolBufferException;
import com.google.protobuf.Parser;

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

import spaceServer.session.Session.AuthCredential;
import spaceServer.session.Session.AuthRequest;

class SpaceServer {

  static final Logger logger = LogManager.getLogger(SpaceServer.class.getName());

  static final int port = 3000; // TODO configure magic number
  static final SocketAddress addr = new InetSocketAddress(port);

  // interface GatheringScatteringByteChannel extends GatheringByteChannel, ScatteringByteChannel {}
  // java complains about duck typing...

  static class MessageStream {

    public static <M> M readMessage(ScatteringByteChannel channel, Parser<M> parser) throws InvalidProtocolBufferException, IOException, SuspendExecution {
      ByteBuffer msgLengthBuf = ByteBuffer.allocate(4);
      long status = channel.read(msgLengthBuf);
      logger.debug("Read: " + status + " bytes");
      int msgLength = msgLengthBuf.getInt(0);
      logger.debug("Message length is: " + msgLength);
      ByteBuffer msgBuf = ByteBuffer.allocate(msgLength);
      status = channel.read(msgBuf);
      logger.debug("Read: " + status + " bytes");
      return parser.parseFrom(msgBuf.array());
    }

  }

  static void handleClient(FiberSocketChannel clientChannel) throws SuspendExecution {
    logger.debug("Handling client channel");
    try {
      AuthRequest authReq = MessageStream.readMessage(clientChannel, AuthRequest.PARSER);
      AuthCredential authCred = authReq.getCredential();
      String authUsername = authCred.getUsername();
      String authPassword = authCred.getPassword();
      // TODO actually check something :-)
      logger.info("Authenticated: '" + authUsername + "' ('" + authPassword + "')");
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
