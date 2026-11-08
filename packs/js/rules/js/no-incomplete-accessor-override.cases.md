# js/no-incomplete-accessor-override — cases

### case: an imported base is not followed

```js
import Base from "./base.js";
export class B extends Base {
  set v(x) {}
}
```

```expect
(none)
```

### case: static accessors

```js
class A {
  static get v() {}
  static set v(x) {}
}
class B extends A {
  static get v() {}
}
```

```expect
6:14 js/no-incomplete-accessor-override
```

### case: a method in the parent is not an accessor

```js
class A {
  v() {}
}
class B extends A {
  get v() {}
}
```

```expect
(none)
```
