to run maven main class
```
mvn exec:java -Dexec.mainClass="com.mycompany.app.App"
```

to plain run java, from target folder:
```
java -cp my-app-1.0-SNAPSHOT.jar com.mycompany.app.App
```


```
dot_clean -mn /Volumes/Samsung\ USB/test/netty-echo-server/my-app
```


to send to server
```
echo "hello ievgen" | nc 127.0.0.1 9090
```


to run server + client from ch2

from root project
```
mvn clean package
```

from ch02/Server folder + the same from ch02/Client folder
```
mvn exec:java
```