# html/valid-for — cases

### case: no labelable control to point at stays manual

```html
<label for="name">Name</label>
<div id="name"></div>
```

```expect
1:13 html/valid-for
```
