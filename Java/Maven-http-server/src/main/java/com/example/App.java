package com.example;

import org.apache.catalina.LifecycleException;
import org.apache.catalina.startup.Tomcat;

import java.io.File;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.logging.Level;
import java.util.logging.Logger;

public class App {
    private static final Logger LOGGER = Logger.getLogger(App.class.getName());

    public static void main(String[] args) {
        int serverCount = 3; // Number of Tomcat instances
        int basePort = 57890;

        ExecutorService executor = Executors.newFixedThreadPool(serverCount);
        List<TomcatServer> servers = new ArrayList<>();

        for (int i = 0; i < serverCount; i++) {
            int port = basePort + i;
            TomcatServer server = new TomcatServer(port);
            servers.add(server);
            executor.submit(server);
        }

        // Allow servers to run in parallel
        executor.shutdown();
    }
}

class TomcatServer implements Runnable {
    private final int port;

    public TomcatServer(int port) {
        this.port = port;
    }

    @Override
    public void run() {
        try {
            Tomcat tomcat = new Tomcat();
            tomcat.setPort(port);
            tomcat.getConnector().setProperty("address", "0.0.0.0");

            String webappDir = new File("src/main/webapp").getAbsolutePath();
            tomcat.addWebapp("/", webappDir);

            tomcat.start();
            System.out.println("Tomcat server started on http://0.0.0.0:" + port);
            tomcat.getServer().await();
        } catch (LifecycleException e) {
            Logger.getLogger(TomcatServer.class.getName()).log(Level.SEVERE, "Error starting Tomcat on port " + port, e);
        }
    }
}
