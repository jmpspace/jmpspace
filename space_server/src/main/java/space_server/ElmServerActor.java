package space_server;

import co.paralleluniverse.actors.*;
import co.paralleluniverse.comsat.webactors.*;
import co.paralleluniverse.fibers.SuspendExecution;
import co.paralleluniverse.strands.channels.Channels;
import co.paralleluniverse.strands.channels.SendPort;
import com.google.common.base.Function;
import java.util.Collections;
import java.util.Set;
import java.util.concurrent.ConcurrentHashMap;

@WebActor(httpUrlPatterns = {"/*"})
public class ElmServerActor extends BasicActor<WebMessage, Void> {
  // There is one actor for each client
  private static final Set<ActorRef<WebMessage>> actors =
    Collections.newSetFromMap(new ConcurrentHashMap<ActorRef<WebMessage>, Boolean>());

  // The client representation of this actor
  private SendPort<WebDataMessage> peer;

  @Override
    protected final Void doRun() throws InterruptedException, SuspendExecution {
      actors.add(self());
      try {
        //noinspection InfiniteLoopStatement
        for (;;) {
          final Object message = receive();
          if (message instanceof HttpRequest) {
            final HttpRequest msg = (HttpRequest) message;
            switch (msg.getRequestURI()) {
              case "/":
                msg.getFrom().send(HttpResponse.ok(self(), msg, "ELM CODE HERE").setContentType("text/plain").build());
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
