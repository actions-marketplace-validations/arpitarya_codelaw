# js/no-unnecessary-parameters — cases

### case: a middle parameter

```js
function format(value, unit, digits) {
  return `${value.toFixed(digits)}${unit}`;
}
format(1, 'px', 0);
format(2, 'px', 1);
```

```expect
1:24 js/no-unnecessary-parameters
```

```fixed
function format(value, digits) {
  return `${value.toFixed(digits)}${'px'}`;
}
format(1, 0);
format(2, 1);
```

### case: a binding the function already sees

```js
const limit = 10;
function clamp(value, maximum) {
  return Math.min(value, maximum);
}
clamp(1, limit);
clamp(2, limit);
```

```expect
2:23 js/no-unnecessary-parameters
```

```fixed
const limit = 10;
function clamp(value) {
  return Math.min(value, limit);
}
clamp(1);
clamp(2);
```

### case: always the default

```js
function print(message, extra = false) {
  console.log(message, extra);
}
print('Starting');
print('Finished', false);
print('Done', undefined);
```

```expect
1:25 js/no-unnecessary-parameters
```

```fixed
function print(message) {
  console.log(message, false);
}
print('Starting');
print('Finished');
print('Done');
```

### case: always undefined

```js
function pair(first, second) {
  return [first, second];
}
pair(1);
pair(2);
```

```expect
1:22 js/no-unnecessary-parameters
```

```fixed
function pair(first) {
  return [first, (void 0)];
}
pair(1);
pair(2);
```

### case: a function called once

```js
function log(message, level) {
  console.log(level, message);
}
log('Starting', 'info');
```

```expect
(none)
```

### case: different values

```js
function log(message, level) {
  console.log(level, message);
}
log('Starting', 'info');
log('Finished', 'warn');
```

```expect
(none)
```

### case: a function that escapes

```js
function log(message, level) {
  console.log(level, message);
}
log('Starting', 'info');
log('Finished', 'info');
register(log);
```

```expect
(none)
```

### case: an exported function

```js
export function log(message, level) {
  console.log(level, message);
}
log('Starting', 'info');
log('Finished', 'info');
```

```expect
(none)
```

### case: a spread argument

```js
function log(message, level) {
  console.log(level, message);
}
log(...args);
log('Finished', 'info');
```

```expect
(none)
```

### case: recursion forwarding the parameter does not count as a value

```js
function walk(node, seen = new Set()) {
  if (seen.has(node)) {
    return seen;
  }
  seen.add(node);
  return node.next ? walk(node.next, seen) : seen;
}
walk({ next: {} });
walk({});
```

```expect
(none)
```

### case: the function reads arguments

```js
function log(message, level) {
  return arguments.length + level;
}
log('Starting', 'info');
log('Finished', 'info');
```

```expect
(none)
```

### case: a shallow destructured property

```js
function run({ path, verbose, dryRun = false }) {
  console.log(path, verbose, dryRun);
}
run({ path: 'first', verbose: true });
run({ path: 'second', verbose: true, dryRun: false });
```

```expect
1:22 js/no-unnecessary-parameters
1:31 js/no-unnecessary-parameters
```

### case: a global of the language is reported without an edit

```js
function pick(list, math) {
  return math.max(...list);
}
pick([1], Math);
pick([2], Math);
```

```expect
1:21 js/no-unnecessary-parameters
```

### case: an arrow function with one parameter

```js
const twice = (n) => n * 2;
twice(2);
twice(2);
```

```expect
1:16 js/no-unnecessary-parameters
```

```fixed
const twice = () => 2 * 2;
twice();
twice();
```

### case: a constructor called with new

```js
class Box {
  constructor(width, height) {
    this.area = width * height;
  }
}
new Box(1, 3);
new Box(2, 3);
```

```expect
2:22 js/no-unnecessary-parameters
```

```fixed
class Box {
  constructor(width) {
    this.area = width * 3;
  }
}
new Box(1);
new Box(2);
```

### case: a class used any other way

```js
class Box {
  constructor(width, height) {
    this.area = width * height;
  }
}
new Box(1, 3);
new Box(2, 3);
list.push(Box);
```

```expect
(none)
```

### case: a private method

```js
class A {
  #show(value, label) {
    return label + value;
  }
  run() {
    this.#show(1, 'n');
    this.#show(2, 'n');
  }
}
```

```expect
2:16 js/no-unnecessary-parameters
```

```fixed
class A {
  #show(value) {
    return 'n' + value;
  }
  run() {
    this.#show(1);
    this.#show(2);
  }
}
```

### case: one parameter is fixed at a time

```js
function f(a, b) {
  return a + b;
}
f(1, 2);
f(1, 2);
```

```expect
1:12 js/no-unnecessary-parameters
1:15 js/no-unnecessary-parameters
```

```fixed
function f() {
  return 1 + 2;
}
f();
f();
```

### case: a comment in the call leaves no edit

```js
function log(message, level) {
  console.log(level, message);
}
log('Starting', /* the level */ 'info');
log('Finished', 'info');
```

```expect
1:23 js/no-unnecessary-parameters
```
