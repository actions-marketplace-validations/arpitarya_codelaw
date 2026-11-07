# js/prefer-classlist-toggle — cases

### case: a branch with a second statement stays

```js
if (open) {
  menu.classList.add("is-open");
  focus();
} else {
  menu.classList.remove("is-open");
}
```

```expect
(none)
```
