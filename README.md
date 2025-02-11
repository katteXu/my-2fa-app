## 跨平台编译

### Mac

```sh
# windows aarch64-pc-windows-msvc
bun run tauri build --runner cargo-xwin --target aarch64-pc-windows-msvc
```

```sh
# windows x86_64-pc-windows-msvc
bun run tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc
```

```sh
# mac aarch64-apple-darwin
bun run tauri build --target aarch64-apple-darwin
```

### 添加 github action 跨平台编译
