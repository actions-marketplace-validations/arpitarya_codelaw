# sec-js/no-prototype-pollution — cases

### case: a merge of the request body

```js
_.merge(config, req.body);
```

```expect
1:1 sec-js/no-prototype-pollution
```

### case: a literal key

```js
config["theme"] = req.body.theme;
```

```expect
(none)
```

### case: defaultsDeep

```js
_.defaultsDeep(cfg, req.body);
```

```expect
1:1 sec-js/no-prototype-pollution
```

### case: set with a key

```js
_.set(obj, req.query.path, 1);
```

```expect
1:1 sec-js/no-prototype-pollution
```

### case: fix: own property defined

```js
config[req.body.key] = req.body.value;
```

```expect
1:1 sec-js/no-prototype-pollution
```

```fixed
Object.defineProperty(config, req.body.key, { value: req.body.value, writable: true, enumerable: true, configurable: true });
```
