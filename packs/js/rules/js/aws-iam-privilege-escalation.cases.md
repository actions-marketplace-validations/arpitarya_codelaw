# js/aws-iam-privilege-escalation — cases

### case: a wildcard user ARN

```js
new iam.PolicyStatement({
  actions: ["iam:CreateAccessKey"],
  resources: ["arn:aws:iam::123456789012:user/*"],
});
```

```expect
1:1 js/aws-iam-privilege-escalation
```

### case: a deny statement stays

```js
new iam.PolicyStatement({
  effect: iam.Effect.DENY,
  actions: ["iam:CreateAccessKey"],
  resources: ["*"],
});
```

```expect
(none)
```

### case: a harmless action stays

```js
new iam.PolicyStatement({
  actions: ["s3:GetObject"],
  resources: ["*"],
});
```

```expect
(none)
```
