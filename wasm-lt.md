<!-- $theme: default -->

# WASM
Takamichi Tsutsumi
***
## Wasmとは
> WebAssembly (abbreviated Wasm) is a binary instruction format for a stack-based virtual machine. Wasm is designed as a portable target for compilation of high-level languages like C/C++/Rust, enabling deployment on the web for client and server applications.

[公式](https://webassembly.org/)


拙訳
*WebAssemblyとはスタックベースのVMをターゲットとしたバイナリインストラクションのフォーマットで、C/C++/Rustのような高水準言語をコンパイルしたターゲットファイルをwebのクライアントあるいはサーバーにデプロイできるようにしたもの。*

***
## Agenda
- フロントエンド今昔
- フロントエンドの課題
- Solutions
- Wasm

***
## フロントエンド今昔
昔
![](/Users/uu107017/Documents/ScreenShots/スクリーンショット%202018-07-02%2022.52.10.png)
***
今
![](/Users/uu107017/Documents/ScreenShots/スクリーンショット%202018-07-02%2022.52.49.png)


***

リッチなCPUを備えたデバイスが普及したことによって

Documentをやりとりするためのwebから
アプリケーションプラットフォームとしてのwebへと進化している


***

## Solutions
- React: 仮想DOMの概念でリッチなアプリケーション(SPA)を実現する手段
- v8: JITベースの高速なJavaScript実行環境
- Servo: Mozilaが作ってる並列レンダリングを中心に据えたブラウザレンダリングエンジン

***

JavaScript on webの問題もある

- 実行速度(V8などで高速化しているとはいえ速度的な制約はある)
- 動的型付けに起因する実行時エラー
- バンドルサイズの肥大化(Reactアプリケーションのデカさ)
![](/Users/uu107017/Documents/ScreenShots/スクリーンショット%202018-07-02%2023.02.39.png)


***

そこでWASM
- 高速
- 小ファイルサイズ
- 型安全
- 既存の資産の活用 (OpenCV, Unityとかつかえる)


***

## ブラウザ対応状況
![](/Users/uu107017/Documents/ScreenShots/スクリーンショット%202018-06-29%2015.28.21.png)


***
# DEMO

***

## Fibonacci

***

## Markdown parser

***
## Wasmフレンズ募集中