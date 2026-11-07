# js/prefer-observer-apis — cases

### case: a scroll position read stays

```js
window.addEventListener("scroll", () => {
  header.classList.toggle("stuck", window.scrollY > 0);
});
```

```expect
(none)
```
