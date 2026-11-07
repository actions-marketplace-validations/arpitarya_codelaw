# js/prefer-toggle-attribute — cases

### case: a branch with a second statement stays

```js
if (busy) {
  button.setAttribute("disabled", "");
  spin();
} else {
  button.removeAttribute("disabled");
}
```

```expect
(none)
```
