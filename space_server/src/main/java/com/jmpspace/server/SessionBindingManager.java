//package com.jmpspace.server;
//
//import co.paralleluniverse.actors.Actor;
//import co.paralleluniverse.actors.ActorRef;
//import co.paralleluniverse.fibers.SuspendExecution;
//
//import java.util.Date;
//import java.util.HashMap;
//import java.util.Map;
//
//public class SessionBindingManager extends Actor
//{
//
//    private Map<String, Activity> userActivity;
//
//    public SessionBindingManager() {
//        userActivity = new HashMap<String, Activity>();
//    }
//
//    public class HijackMessage {
//        Activity activity;
//    }
//
//    public class PingMessage {
//        Activity activity;
//    }
//
//    public class Activity {
//        Date lastEventStamp;
//        ActorRef<?> actor;
//    }
//
//    @Override
//    protected Object doRun() throws InterruptedException, SuspendExecution {
//        try {
//            final Object message = receive();
//        } catch (Exception e) {
//
//        }
//    }
//}
