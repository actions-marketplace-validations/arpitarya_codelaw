---
id: js/aws-restricted-ip-admin-access
title: "No SSH or RDP open to the internet"
languages: [javascript, jsx, typescript, tsx]
fix: manual
fix_reason: "the allowed range is the owner's"
fix_exception: security
stability: preview
ported_from: [eslint-plugin-sonarjs/aws-restricted-ip-admin-access]
since: 0.5.0
---

A security group does not open port 22 or 3389 (or all traffic) to `Peer.anyIpv4()` or `Peer.anyIpv6()`.

**Why:** Admin ports open to the internet are scanned and brute-forced within minutes.

```codelaw
match:
  kind: call_expression
  has:
    field: function
    regex: \.addIngressRule$
  all:
  - has:
      field: arguments
      has:
        nthChild: 1
        regex: Peer\.(anyIpv4|anyIpv6)\(\)
      all:
      - has:
          nthChild: 2
          regex: Port\.(tcp\((22|3389)\)|allTraffic\(\))
message: SSH or RDP is open to every address; restrict it to known ranges or use a bastion
```

## How to fix

1. Restrict the peer to the ranges that need access, or reach the host through Systems Manager or a bastion.

## Examples

### JavaScript

#### Don't

```js
sg.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.tcp(22));
```

#### Do

```js
sg.addIngressRule(ec2.Peer.ipv4("203.0.113.0/24"), ec2.Port.tcp(22));
```

### JSX

#### Don't

```jsx
sg.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.tcp(22));
```

#### Do

```jsx
sg.addIngressRule(ec2.Peer.ipv4("203.0.113.0/24"), ec2.Port.tcp(22));
```

### TypeScript

#### Don't

```ts
sg.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.tcp(22));
```

#### Do

```ts
sg.addIngressRule(ec2.Peer.ipv4("203.0.113.0/24"), ec2.Port.tcp(22));
```

### TSX

#### Don't

```tsx
sg.addIngressRule(ec2.Peer.anyIpv4(), ec2.Port.tcp(22));
```

#### Do

```tsx
sg.addIngressRule(ec2.Peer.ipv4("203.0.113.0/24"), ec2.Port.tcp(22));
```
