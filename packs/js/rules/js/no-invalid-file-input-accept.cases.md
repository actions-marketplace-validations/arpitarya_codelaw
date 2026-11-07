# js/no-invalid-file-input-accept — cases

### case: a bare extension

```jsx
const p = <input type="file" accept="png" />;
```

```expect
1:30 js/no-invalid-file-input-accept
```

```fixed
const p = <input type="file" accept=".png" />;
```

### case: uppercase

```jsx
const p = <input type="file" accept="IMAGE/*" />;
```

```expect
1:30 js/no-invalid-file-input-accept
```

```fixed
const p = <input type="file" accept="image/*" />;
```

### case: a valid list stays

```jsx
const p = <input type="file" accept="image/png, .png" />;
```

```expect
(none)
```

### case: another input stays

```jsx
const p = <input type="text" accept="png" />;
```

```expect
(none)
```
