# js/dynamically-constructed-templates — cases

### case: a template literal with a substitution

```js
const render = pug.compile(`h1 Hello ${name}`);
```

```expect
1:16 js/dynamically-constructed-templates
```

### case: a literal template stays

```js
const render = pug.compile("h1 Hello");
```

```expect
(none)
```
