---
id: js/x-powered-by
title: "An Express app hides its X-Powered-By header"
languages: [javascript, jsx, typescript, tsx]
fix: suggest
fix_reason: "the line is added after the app is made; the reviewer checks no middleware sets the header back"
stability: preview
ported_from: [eslint-plugin-sonarjs/x-powered-by]
since: 0.5.0
---

A file that makes an Express app (`const app = express()`) turns off the `X-Powered-By` header, with `app.disable("x-powered-by")` or helmet.

**Why:** The header tells every client which framework, and so which known weaknesses, the server has.

```codelaw
match:
  kind: lexical_declaration
  pattern: "const $APP = express();"
  inside:
    kind: program
    not:
      has:
        stopBy: end
        any:
          - pattern: '$X.disable("x-powered-by")'
          - pattern: "$X.disable('x-powered-by')"
          - pattern: "helmet($$$H)"
          - pattern: "$X.hidePoweredBy($$$H)"
message: "Disable the X-Powered-By header"
fix:
  - insert-after: "\n$APP.disable(\"x-powered-by\");"
```

## Examples

### JavaScript

#### Don't

```js
const app = express();
app.listen(3000);
```

#### Do

```js
const app = express();
app.disable("x-powered-by");
app.listen(3000);
```


### JSX

#### Don't

```jsx
const app = express();
app.listen(3000);
```

#### Do

```jsx
const app = express();
app.disable("x-powered-by");
app.listen(3000);
```


### TypeScript

#### Don't

```ts
const app = express();
app.listen(3000);
```

#### Do

```ts
const app = express();
app.disable("x-powered-by");
app.listen(3000);
```


### TSX

#### Don't

```tsx
const app = express();
app.listen(3000);
```

#### Do

```tsx
const app = express();
app.disable("x-powered-by");
app.listen(3000);
```
