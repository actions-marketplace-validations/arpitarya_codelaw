# js/prefer-abort-signal-timeout — cases

### case: the timer is kept

```js
const ac = new AbortController();
const t = setTimeout(() => ac.abort(), 5);
run(ac.signal);
```

```expect
(none)
```

### case: aborted with a reason

```js
const ac = new AbortController();
setTimeout(() => ac.abort(why), 5);
run(ac.signal);
```

```expect
(none)
```
