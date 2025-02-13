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

1. 安装证书 .p12 文件
2. security find-identity -v -p codesigning 拿到证书第一列标识
3. 将标识符添加到 src-tauri > tauri.config.json 文件下

   {
   "bundle": {
   "macOS": {
   "signingIdentity":"标识符"
   }
   }
   }

4. 安装依赖 bun install
5. 本地构建 bun run tauri build --bundles dmg
