# 2020-lecture-02

[p-ray](https://p-ray.oskt.us/)の2020年度講義における, 古典的レイトレーサーの実装の講義用リポジトリです.

## Requirements

* C++17
* CMake 3.12 or Higher

## ディレクトリ構造

```
.
├── CMakeLists.txt
├── LICENSE
├── README.md
├── ref           リファレンス実装
└── src           演習で実装していく部分
```

## ビルド

```
mkdir build
cd build
cmake ..
make
```

## 実装方法

リポジトリをforkし, `src`にヘッダオンリーに実装していきます. ビルドは上記方法で行えます.