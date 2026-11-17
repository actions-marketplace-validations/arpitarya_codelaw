# js/string-content — cases

### case: silent without the option

```js
const loading = "Loading...";
```

```expect
(none)
```

### case: a string

```js options={"patterns":{"\\.\\.\\.":"…"}}
const loading = "Loading...";
```

```expect
1:17 js/string-content
```

```fixed
const loading = "Loading…";
```

### case: a template's text

```js options={"patterns":{"\\.\\.\\.":"…"}}
const text = `Wait...${n}...`;
```

```expect
1:14 js/string-content
```

```fixed
const text = `Wait…${n}…`;
```

### case: a fix that is only a suggestion

```js options={"patterns":{"colour":{"suggest":"color","fix":false}}}
const a = "colour";
```

```expect
1:11 js/string-content
```

### case: case does not matter

```js options={"patterns":{"colour":{"suggest":"color","caseSensitive":false}}}
const a = "Colour";
```

```expect
1:11 js/string-content
```

```fixed
const a = "color";
```

### case: the replacement is escaped for its quote

```js options={"patterns":{"\\.\\.\\.":"'"}}
const a = 'it...';
```

```expect
1:11 js/string-content
```

```fixed
const a = 'it\'';
```

### case: a tagged template of another language is not read

```js options={"patterns":{"\\.\\.\\.":"…"}}
const q = sql`select 1...`;
```

```expect
(none)
```

### case: an escape is not read

```js options={"patterns":{"\\.\\.\\.":"…"}}
const a = "a\u002e\u002e\u002e";
```

```expect
(none)
```
