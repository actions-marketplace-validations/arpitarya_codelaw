# js/id-denylist — cases

### case: silent without the option

```js
const data = load();
const temp = 1;
```

```expect
(none)
```

### case: a declaration, a parameter and a catch clause

```js options={"names":["data","temp","e"]}
const data = load();
function run(temp) {
  return temp;
}
try {
  run(data);
} catch (e) {
  throw e;
}
```

```expect
1:7 js/id-denylist
3:8 js/id-denylist
4:10 js/id-denylist
8:10 js/id-denylist
9:9 js/id-denylist
```

### case: a call's callee and arguments are not checked

```js options={"names":["data"]}
const result = format(data);
const again = data(1);
```

```expect
(none)
```

### case: a property is checked when it is assigned to

```js options={"names":["data"]}
item.data = 1;
const x = item.data;
```

```expect
1:6 js/id-denylist
```

### case: a global the file does not declare

```js options={"names":["data"]}
const copy = data;
```

```expect
(none)
```

### case: an object key

```js options={"names":["data"]}
const options = { data: 1 };
```

```expect
1:19 js/id-denylist
```

### case: a destructuring key and a renamed import are the other name's

```js options={"names":["data"]}
const { data: records } = source;
import { data as items } from "./store";
```

```expect
(none)
```

### case: a private name

```js options={"names":["data"]}
class Store {
  #data = 1;
}
```

```expect
2:3 js/id-denylist
```
