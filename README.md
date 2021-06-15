# `@napi-rs/notify`

![https://github.com/Brooooooklyn/notify/actions](https://github.com/Brooooooklyn/notify/workflows/CI/badge.svg)

[notify](https://github.com/notify-rs/notify) Node.js binding via [napi-rs](https://napi.rs).

## Install this package

```
yarn add @napi-rs/notify
```

## Support matrix

### Operating Systems

|                  | node12 | node14 | node16 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      |
| Android arm64    | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      |

## API

```ts
const unwatch = watch('/some/path/to/watch', (err: Error | null, event: NotifyEvent) => {
  // some logic
})

unwatch() // unwatch later

```