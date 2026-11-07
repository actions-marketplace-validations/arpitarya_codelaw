# js/require-text-decoder-streaming — cases

### case: another value stays

```js
for await (const chunk of response.body) {
  text += decoder.decode(header);
}
```

```expect
(none)
```

### case: a loop over something else stays

```js
for await (const chunk of chunks) {
  text += decoder.decode(chunk);
}
```

```expect
(none)
```
