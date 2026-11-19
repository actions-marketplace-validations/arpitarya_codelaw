# js/comment-regex — cases

### case: silent without the option

```js
// FIXME: later
run();
```

```expect
(none)
```

### case: a line comment and a block comment

```js options={"regularExpression":"FIXME"}
// FIXME: later
/* FIXME and more */
// fine
run();
```

```expect
1:1 js/comment-regex
2:1 js/comment-regex
```

### case: the text is without the delimiters

```js options={"regularExpression":"^FIXME"}
//FIXME
// x FIXME
run();
```

```expect
1:1 js/comment-regex
```

### case: a pattern that is not a regular expression

```js options={"regularExpression":"("}
// a
run();
```

```expect
1:1 js/comment-regex
```
