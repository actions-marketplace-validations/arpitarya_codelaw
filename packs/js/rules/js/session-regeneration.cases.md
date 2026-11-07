# js/session-regeneration — cases

### case: session off stays

```js
app.post("/token", passport.authenticate("jwt", { session: false }), (req, res) => {
  res.json(req.user);
});
```

```expect
(none)
```
