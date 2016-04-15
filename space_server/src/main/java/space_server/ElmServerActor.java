package space_server;

import co.paralleluniverse.actors.*;
import co.paralleluniverse.comsat.webactors.*;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.fibers.io.FiberFileChannel;
import co.paralleluniverse.strands.channels.Channels;
import co.paralleluniverse.strands.channels.SendPort;

import com.google.common.base.Function;

import java.io.IOException;
import java.net.URI;
import java.nio.ByteBuffer;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.Collections;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;

import org.apache.commons.lang3.concurrent.ConcurrentException;
import org.apache.commons.lang3.concurrent.LazyInitializer;

@WebActor(httpUrlPatterns = {"/index.html"})
public class ElmServerActor extends BasicActor<WebMessage, Void> {
  // There is one actor for each client
  private static final Set<ActorRef<WebMessage>> actors =
    Collections.newSetFromMap(new ConcurrentHashMap<ActorRef<WebMessage>, Boolean>());

  // The client representation of this actor
  private SendPort<WebDataMessage> peer;

  private static LazyInitializer<String> indexSource = new LazyInitializer<String>() {
    @Override
    protected String initialize() throws ConcurrentException {
      try {
        return new String(Files.readAllBytes(Paths.get("../client/index.html")));
      } catch (IOException e) {
        throw new ConcurrentException(e);
      }
    }
  };

  @Override
  protected final Void doRun() throws InterruptedException, SuspendExecution {
    actors.add(self());
    try {
      //noinspection InfiniteLoopStatement
      for (;;) {
        final Object message = receive();
        System.out.println("elm server actor - received");
        if (message instanceof HttpRequest) {
          final HttpRequest msg = (HttpRequest) message;
          switch (msg.getRequestURI()) {
            case "/index.html":
              // FIXME: catch the ConcurrentException here, and return a 500 is there's a problem!
              try {
                msg.getFrom().send(HttpResponse.ok(self(), msg, indexSource.get()).setContentType("text/html").build());
              } catch (ConcurrentException e) {
                e.printStackTrace();
                msg.getFrom().send(HttpResponse.error(self(), msg, 500, "Server error: " + e.getMessage()).setContentType("text/plain").build());
              }
              break;
            default:
              msg.getFrom().send(HttpResponse.error(self(), msg, 404, "Not found").setContentType("text/plain").build());
              break;
          }
        }
      }
    } finally {
      actors.remove(self());
    }
  }

  @Override
  protected final WebMessage handleLifecycleMessage(LifecycleMessage m) {
    // while listeners might contain an SSE actor wrapped with Channels.map, the wrapped SendPort maintains the original actors hashCode and equals behavior
    if (m instanceof ExitMessage)
      actors.remove(((ExitMessage) m).getActor());
    return super.handleLifecycleMessage(m);
  }
}
