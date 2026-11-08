# md/relative-links — cases

### case: a target with no file near it stays manual

````md
<!-- docs/guide.md -->
Start with [the setup](./setup.md), then read the [FAQ](../FAQ.md).
<!-- docs/setup.md -->
# Setup
````

```expect
docs/guide.md:1:51 md/relative-links
```
