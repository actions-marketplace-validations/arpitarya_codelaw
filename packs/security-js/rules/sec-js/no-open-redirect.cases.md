# sec-js/no-open-redirect — cases

### case: with a status

```js
res.redirect(302, req.query.to);
```

```expect
1:1 sec-js/no-open-redirect
```

### case: NextResponse

```js
NextResponse.redirect(searchParams.get("next"));
```

```expect
1:1 sec-js/no-open-redirect
```

### case: fix: same-origin path only

```js
res.redirect(req.query.next);
```

```expect
1:1 sec-js/no-open-redirect
```

```fixed
res.redirect((u => u.origin === "http://x.invalid" ? u.pathname + u.search : "/")(new URL(String(req.query.next), "http://x.invalid")));
```
