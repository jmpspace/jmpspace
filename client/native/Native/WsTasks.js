Elm.Native = Elm.Native || {};
Elm.Native.WsTasks = {};
Elm.Native.WsTasks.make = function(localRuntime) {
  localRuntime.Native = localRuntime.Native || {};
  localRuntime.Native.WsTasks = localRuntime.Native.WsTasks || {};
  if (localRuntime.Native.WsTasks.values) {
    return localRuntime.Native.WsTasks.values;
  }

  // Probably not totally pure, taking a shortcut
  function relativeUri(s) {
    var l = window.location;
    return ((l.protocol === "https:") ? "wss://" : "ws://") + l.host + s;
  }

  function open(uri) {
    var ws = new WebSocket(uri);
    ws.onopen = function(e) {
      console.log(e);
    }
    ws.onclose = function(e) {
      console.log(e);
    }
    ws.onerror = function(e) {
      console.log(e);
    }
    return ws;
  }

  return localRuntime.Native.WsTasks.values = {
    open: open,
    relativeUri: relativeUri
  }
}
