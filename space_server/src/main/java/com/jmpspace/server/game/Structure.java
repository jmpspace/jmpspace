package com.jmpspace.server.game;

import java.util.ArrayList;

import co.paralleluniverse.actors.ActorRef;
import co.paralleluniverse.actors.BasicActor;
import co.paralleluniverse.common.util.Function2;
import co.paralleluniverse.common.util.Function3;
import co.paralleluniverse.common.util.Tuple;
import co.paralleluniverse.fibers.SuspendExecution;
import com.jmpspace.contracts.SpaceServer.Structure.Attachment;
import com.jmpspace.contracts.SpaceServer.Structure.AttachmentData;
import com.jmpspace.contracts.SpaceServer.Structure.Part;
import com.jmpspace.contracts.SpaceServer.Structure.StructureNode;
import com.vividsolutions.jts.geom.Geometry;
import com.vividsolutions.jts.geom.util.AffineTransformation;

import java.util.List;
import java.util.concurrent.ConcurrentMap;
import java.util.function.Function;

public class Structure extends BasicActor<Structure.Request, Void> {

  class PlayerRef {
    ActorRef<Player.Request> actor;
  }

  private ConcurrentMap<List<Integer>, PlayerRef> _playersOnBoard;
  private StructureNode _tree;
  private Geometry _geom;

  abstract static class State {
    ActorRef<Structure.Request> _owner;
  }

  static class StructureUtils {

    static <TResult> void foldStructure(Function2<AttachmentData, TResult, TResult> fAttach, Function2<Part, List<TResult>, TResult> fNode, StructureNode tree) {
      List<Attachment> attachments = tree.getAttachmentsList();

//      new List()

      attachments.forEach(attachment -> {

      });
    }

  }

  class CalcualteGeometryState {
    AffineTransformation _currentOffset;
    List<Geometry> _accum;

    public CalcualteGeometryState() {
      this._accum = new ArrayList<>();
      this._currentOffset = new AffineTransformation();
    }
  }

  private void calculateGeometry() {

    Function2<AttachmentData, CalcualteGeometryState, CalcualteGeometryState> fAttach = (data, state) -> {
      // TODO: translate the sub-tree
      return state;
    };

    Function2<Part, List<CalcualteGeometryState>, CalcualteGeometryState> fNode = (part, attachStates) -> {
      // TODO: combine all sub-geometry plus the geometry of this part
      return new CalcualteGeometryState();
    };

    StructureUtils.foldStructure(fAttach, fNode, _tree);
  }

  @Override
  protected Void doRun() throws InterruptedException, SuspendExecution {
    return null;
  }

  abstract class Request {}

  class Board extends Request {
    PlayerRef player;
    List<Integer> platformPath;
  }
}
