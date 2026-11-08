---
id: js/no-incomplete-accessor-override
title: "No overriding only the getter or only the setter of an inherited pair"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "whether the override needs the other accessor, and what it does, is the author's design"
fix_exception: restructure
stability: preview
ported_from: [eslint-plugin-unicorn/no-incomplete-accessor-override]
since: 0.5.0
---

A class that `extends` another class of the same file and defines only `get x` or only `set x`, where the nearest ancestor defining `x` has the other one (or both), is reported at the accessor's key. Defining one accessor replaces the whole inherited property, so the other half is gone. The ancestors are followed through class declarations and `const` aliases; an imported or decorated class, a computed key or a field stops the search.

**Why:** After `set v` alone, reading `v` on the subclass gives `undefined` instead of calling the inherited getter.

```codelaw
native: js/no-incomplete-accessor-override
```

## How to fix

1. Define the missing accessor too; it can forward to the parent: `get v() { return super.v; }`.

## Examples

### JavaScript

#### Don't

```js
class Base {
  get size() {
    return this.n;
  }
  set size(v) {
    this.n = v;
  }
}
class Fixed extends Base {
  set size(v) {}
}
```

#### Do

```js
class Base {
  get size() {
    return this.n;
  }
  set size(v) {
    this.n = v;
  }
}
class Fixed extends Base {
  get size() {
    return super.size;
  }
  set size(v) {}
}
```


### JSX

#### Don't

```jsx
class Base {
  get size() {
    return this.n;
  }
  set size(v) {
    this.n = v;
  }
}
class Fixed extends Base {
  set size(v) {}
}
```

#### Do

```jsx
class Base {
  get size() {
    return this.n;
  }
  set size(v) {
    this.n = v;
  }
}
class Fixed extends Base {
  get size() {
    return super.size;
  }
  set size(v) {}
}
```


### TypeScript

#### Don't

```ts
class Base {
  get size() {
    return this.n;
  }
  set size(v) {
    this.n = v;
  }
}
class Fixed extends Base {
  set size(v) {}
}
```

#### Do

```ts
class Base {
  get size() {
    return this.n;
  }
  set size(v) {
    this.n = v;
  }
}
class Fixed extends Base {
  get size() {
    return super.size;
  }
  set size(v) {}
}
```


### TSX

#### Don't

```tsx
class Base {
  get size() {
    return this.n;
  }
  set size(v) {
    this.n = v;
  }
}
class Fixed extends Base {
  set size(v) {}
}
```

#### Do

```tsx
class Base {
  get size() {
    return this.n;
  }
  set size(v) {
    this.n = v;
  }
}
class Fixed extends Base {
  get size() {
    return super.size;
  }
  set size(v) {}
}
```
