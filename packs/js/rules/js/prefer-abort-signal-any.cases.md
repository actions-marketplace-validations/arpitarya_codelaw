# js/prefer-abort-signal-any — cases

### case: a loop over an array literal

```js
const abortController = new AbortController();
for (const signal of [firstSignal, secondSignal]) {
  signal.addEventListener('abort', () => abortController.abort());
}
run(abortController.signal);
```

```expect
1:7 js/prefer-abort-signal-any
```

```fixed
const abortSignal = AbortSignal.any([firstSignal, secondSignal]);
run(abortSignal);
```

### case: a loop over Array.of with the body on one line

```js
const controller = new AbortController();
for (const signal of Array.of(firstSignal, secondSignal)) signal.addEventListener('abort', () => controller.abort());
run(controller.signal);
```

```expect
1:7 js/prefer-abort-signal-any
```

```fixed
const signal = AbortSignal.any(Array.of(firstSignal, secondSignal));
run(signal);
```

### case: elements that are not signals by name

```js
const controller = new AbortController();
for (const signal of Array.of(a, b)) signal.addEventListener('abort', () => controller.abort());
run(controller.signal);
```

```expect
(none)
```

### case: a loop over a name nothing says is an array

```js
const abortController = new AbortController();
for (const signal of signals) {
  signal.addEventListener('abort', () => abortController.abort());
}
run(abortController.signal);
```

```expect
(none)
```

### case: a timeout source

```js
const abortController = new AbortController();
AbortSignal.timeout(1000).addEventListener('abort', () => abortController.abort());
otherSignal.addEventListener('abort', () => abortController.abort());
run(abortController.signal);
```

```expect
1:7 js/prefer-abort-signal-any
```

```fixed
const abortSignal = AbortSignal.any([AbortSignal.timeout(1000), otherSignal]);
run(abortSignal);
```

### case: the reason forwarded and options given

```js
const abortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort(firstSignal.reason));
secondSignal.addEventListener('abort', () => abortController.abort(), { once: true });
run(abortController.signal);
```

```expect
1:7 js/prefer-abort-signal-any
```

### case: one listener only

```js
const abortController = new AbortController();
signal.addEventListener('abort', () => abortController.abort());
run(abortController.signal);
```

```expect
(none)
```

### case: a listener on another event

```js
const abortController = new AbortController();
firstSignal.addEventListener('load', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
run(abortController.signal);
```

```expect
(none)
```

### case: a source that is not named like a signal

```js
const abortController = new AbortController();
first.addEventListener('abort', () => abortController.abort());
second.addEventListener('abort', () => abortController.abort());
run(abortController.signal);
```

```expect
(none)
```

### case: a callback that does more

```js
const abortController = new AbortController();
firstSignal.addEventListener('abort', () => { abortController.abort(); cleanup(); });
secondSignal.addEventListener('abort', () => abortController.abort());
run(abortController.signal);
```

```expect
(none)
```

### case: the controller is aborted elsewhere

```js
const abortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
button.onclick = () => abortController.abort();
run(abortController.signal);
```

```expect
(none)
```

### case: the reason is read

```js
const abortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
log(abortController.signal.reason);
```

```expect
(none)
```

### case: the signal is copied

```js
const abortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
const copy = abortController.signal;
```

```expect
(none)
```

### case: the controller is never read as a signal

```js
const abortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
```

```expect
(none)
```

### case: the controller signal is one of the sources

```js
const abortController = new AbortController();
abortController.signal.addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
run(abortController.signal);
```

```expect
(none)
```

### case: an already aborted source

```js
const abortController = new AbortController();
AbortSignal.abort().addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
run(abortController.signal);
```

```expect
(none)
```

### case: a source that calls something

```js
const abortController = new AbortController();
getSignal().addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
run(abortController.signal);
```

```expect
(none)
```

### case: let is not const

```js
let abortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
run(abortController.signal);
```

```expect
(none)
```

### case: AbortSignal is shadowed

```js
const AbortSignal = {};
const abortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
run(abortController.signal);
```

```expect
(none)
```

### case: the new name is taken

```js
const abortSignal = other;
const abortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
run(abortController.signal, abortSignal);
```

```expect
2:7 js/prefer-abort-signal-any
```

```fixed
const abortSignal = other;
const abortController = AbortSignal.any([firstSignal, secondSignal]);
run(abortController, abortSignal);
```

### case: a comment between the statements leaves no edit

```js
const abortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort());
// and the second
secondSignal.addEventListener('abort', () => abortController.abort());
run(abortController.signal);
```

```expect
1:7 js/prefer-abort-signal-any
```

### case: a typed array in TypeScript

```ts
function compose(signals: readonly AbortSignal[]) {
  const abortController = new AbortController();
  for (const signal of signals) {
    signal.addEventListener('abort', () => abortController.abort());
  }
  return abortController.signal;
}
```

```expect
2:9 js/prefer-abort-signal-any
```

```fixed
function compose(signals: readonly AbortSignal[]) {
  const abortSignal = AbortSignal.any([...signals]);
  return abortSignal;
}
```

### case: a controller annotation becomes a signal annotation

```ts
const abortController: AbortController = new AbortController();
firstSignal.addEventListener('abort', () => abortController.abort());
secondSignal.addEventListener('abort', () => abortController.abort());
run(abortController.signal);
```

```expect
1:7 js/prefer-abort-signal-any
```

```fixed
const abortSignal: AbortSignal = AbortSignal.any([firstSignal, secondSignal]);
run(abortSignal);
```
