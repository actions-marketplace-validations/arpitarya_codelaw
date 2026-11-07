---
id: js/aws-iam-privilege-escalation
title: "No IAM permission that lets a principal escalate its own privileges on every resource"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "which resources the permission is meant for is the owner's to say"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-iam-privilege-escalation]
since: 0.5.0
---

An AWS CDK `PolicyStatement` that allows an IAM action an attacker can use to raise their own privileges — `iam:CreatePolicyVersion`, `iam:SetDefaultPolicyVersion`, `iam:CreateAccessKey`, `iam:CreateLoginProfile`, `iam:UpdateLoginProfile`, the `Attach…Policy` and `Put…Policy` actions, `iam:AddUserToGroup`, `iam:UpdateAssumeRolePolicy` — on all resources (`*`, or an ARN ending in `/*`) is reported. A `Deny` statement is left. Upstream also detects the vectors that need two actions together (`iam:PassRole` with `lambda:CreateFunction`, …); those are not reported here.

**Why:** Any principal holding the policy can make itself an administrator.

```codelaw
match:
  kind: new_expression
  all:
    - has: { field: constructor, regex: '(^|\.)PolicyStatement$' }
    - has:
        field: arguments
        has:
          kind: object
          all:
            - has: { kind: pair, has: { field: key, regex: '^["'']?actions["'']?$' }, regex: '["''](iam:(CreatePolicyVersion|SetDefaultPolicyVersion|CreateAccessKey|CreateLoginProfile|UpdateLoginProfile|AttachUserPolicy|AttachGroupPolicy|AttachRolePolicy|PutUserPolicy|PutGroupPolicy|PutRolePolicy|AddUserToGroup|UpdateAssumeRolePolicy)|iam:\*|\*)["'']' }
            - has: { kind: pair, has: { field: key, regex: '^["'']?resources["'']?$' }, regex: '["''](\*|[^"'']*/\*)["'']' }
            - not: { has: { kind: pair, has: { field: key, regex: '^["'']?effect["'']?$' }, regex: 'DENY' } }
message: "This statement lets its holder escalate privileges on every resource"
```

## How to fix

1. Limit `resources` to the specific ARNs the principal must act on (its own user, a named role).
2. Or drop the escalating action if the principal does not need it.

## Examples

### JavaScript

#### Don't

```js
new iam.PolicyStatement({
  actions: ["iam:UpdateLoginProfile"],
  resources: ["*"],
});
```

#### Do

```js
new iam.PolicyStatement({
  actions: ["iam:UpdateLoginProfile"],
  resources: [user.userArn],
});
```


### JSX

#### Don't

```jsx
new iam.PolicyStatement({
  actions: ["iam:UpdateLoginProfile"],
  resources: ["*"],
});
```

#### Do

```jsx
new iam.PolicyStatement({
  actions: ["iam:UpdateLoginProfile"],
  resources: [user.userArn],
});
```


### TypeScript

#### Don't

```ts
new iam.PolicyStatement({
  actions: ["iam:UpdateLoginProfile"],
  resources: ["*"],
});
```

#### Do

```ts
new iam.PolicyStatement({
  actions: ["iam:UpdateLoginProfile"],
  resources: [user.userArn],
});
```


### TSX

#### Don't

```tsx
new iam.PolicyStatement({
  actions: ["iam:UpdateLoginProfile"],
  resources: ["*"],
});
```

#### Do

```tsx
new iam.PolicyStatement({
  actions: ["iam:UpdateLoginProfile"],
  resources: [user.userArn],
});
```
