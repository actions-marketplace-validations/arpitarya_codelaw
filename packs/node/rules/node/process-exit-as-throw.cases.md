# node/process-exit-as-throw — cases

### case: what follows an exit never runs

```js
function stop() {
  process.exit(1);
  cleanup();
}
```

```expect
3:3 node/process-exit-as-throw
```

### case: a file with its own process

```js
const process = mine();
function stop() {
  process.exit(1);
  cleanup();
}
```

```expect
(none)
```

### case: a conditional exit lets the rest run

```js
function stop(code) {
  if (code) process.exit(code);
  cleanup();
}
```

```expect
```

### case: both branches exit

```js
function stop(code) {
  if (code) {
    process.exit(1);
  } else {
    process.exit(0);
  }
  cleanup();
}
```

```expect
7:3 node/process-exit-as-throw
```

### case: after a return, no-unreachable's

```js
function stop() {
  return 1;
  cleanup();
}
```

```expect
```
