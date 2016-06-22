package com.jmpspace.server;

import co.paralleluniverse.actors.*;
import co.paralleluniverse.comsat.webactors.*;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.strands.channels.Channels;
import co.paralleluniverse.strands.channels.SendPort;
import com.google.common.base.Function;
import java.util.Collections;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;

@WebActor(webSocketUrlPatterns = {"/"})
public class PlayerClientActor extends BasicActor<WebMessage, Void> {
  // There is one actor for each client
  private static final Set<ActorRef<WebMessage>> actors =
    Collections.newSetFromMap(new ConcurrentHashMap<ActorRef<WebMessage>, Boolean>());

  // The client representation of this actor
  private SendPort<WebDataMessage> peer;

  public class PlayerClientState {
    
  }

  @Override
  protected final Void doRun() throws InterruptedException, SuspendExecution {
    actors.add(self());
    try {
      //noinspection InfiniteLoopStatement
      for (;;) {
        final Object message = receive();
        System.out.println("human client actor - received");
        if (message instanceof WebStreamOpened) {
          final WebStreamOpened msg = (WebStreamOpened) message;
          watch(msg.getFrom()); // will call handleLifecycleMessage with ExitMessage when the session ends

          SendPort<WebDataMessage> p = msg.getFrom();
          if (msg instanceof HttpStreamOpened)
            p = wrapAsSSE(p);
          this.peer = p;

          //                    p.send(new WebDataMessage(self(), "Welcome. " + actors.size() + " listeners"));
        } // -------- WebSocket message received --------
        else if (message instanceof WebDataMessage) {
          postMessage((WebDataMessage) message);
        }
      }
    } finally {
      actors.remove(self());
    }
  }

  private SendPort<WebDataMessage> wrapAsSSE(SendPort<WebDataMessage> actor) {
    return Channels.mapSend(actor, new Function<WebDataMessage, WebDataMessage>() {
      @Override
      public final WebDataMessage apply(WebDataMessage f) {
        return new WebDataMessage(f.getFrom(), SSE.event(f.getStringBody()));
      }
    });
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
