# sec-py/no-bind-all-interfaces — cases

### case: uvicorn

```py
uvicorn.run(app, host="0.0.0.0")
```

```expect
1:1 sec-py/no-bind-all-interfaces
```

### case: socket

```py
s.bind(("0.0.0.0", 80))
```

```expect
1:1 sec-py/no-bind-all-interfaces
```

### case: loopback

```py
s.bind(("127.0.0.1", 80))
```

```expect
(none)
```

### case: from the environment

```py
app.run(host=HOST)
```

```expect
(none)
```

### case: the IPv4 wildcard becomes loopback

```py
s.bind(("0.0.0.0", 80))
```

```expect
1:1 sec-py/no-bind-all-interfaces
```

```fixed
s.bind(("127.0.0.1", 80))
```

### case: the IPv6 wildcard becomes its loopback

```py
app.run(host="::")
```

```expect
1:1 sec-py/no-bind-all-interfaces
```

```fixed
app.run(host="::1")
```

### case: a host and port string has no edit

```py
server.run(bind="0.0.0.0:8000")
```

```expect
1:1 sec-py/no-bind-all-interfaces
```
