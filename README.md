# bookos-rp2040

[Rustで始める自作組込みOS入門](https://garasubo.com/embedded-book/) を RP2040 上で動かすことを目指します。

方針として、極力外部の SDK やツールに依存しないようにします。

## 動作環境

- ホスト
  - **Ubuntu**
- ターゲット
  - **Rapsberry Pi Pico H**
    - H がついているとシリアルワイヤデバッグがしやすくて便利です。
    - W がついていると L チカするのに CYW43439 を通さなければいけないのでおすすめしません。
    - Pico 2 では動きません。
  - **Raspberry Pi デバッグプローブ**
    - あると便利です。

## 環境構築

```bash
rustup target add thumbv6m-none-eabi
sudo apt install binutils-arm-none-eabi
```

## ビルド

```bash
cargo build
```

TODO: リリースビルドに対応する

## 書き込みと実行

[picotool](https://github.com/raspberrypi/picotool) を使います。

Rapsberry Pi Pico の BOOTSEL ボタンを押しながらホストに接続し、以下を実行します。

```bash
sudo picotool load -x target/thumbv6m-none-eabi/debug/bookos-rp2040 -t elf
```

TODO: UF2 への変換を自分で行う

## デバッグ

### 動作環境

[ドキュメント](https://www.raspberrypi.com/documentation/microcontrollers/debug-probe.html) に従って Rapsberry Pi Pico H と Raspberry Pi デバッグプローブを接続します。

### 環境構築

OpenOCD と gdb-multiarch をインストールします。

```bash
sudo apt install openocd gdb-multiarch
```

### 実行

OpenOCD サーバを開始します。

```bash
openocd -f interface/cmsis-dap.cfg -f target/rp2040.cfg -c "adapter speed 5000"
```

GDB で接続します。

```bash
gdb-multiarch target/thumbv6m-none-eabi/debug/bookos-rp2040
(gdb) target remote localhost:3333
(gdb) load
(gdb) monitor reset init
```

## 参考文献
- 公式情報
  - [RP2040 Datasheet](https://datasheets.raspberrypi.com/rp2040/rp2040-datasheet.pdf)
  - [Getting started with Raspberry Pi Pico-series](https://datasheets.raspberrypi.com/pico/getting-started-with-pico.pdf)
  - [raspberrypi/pico-sdk](https://github.com/raspberrypi/pico-sdk)
  - [raspberrypi/picotool](https://github.com/raspberrypi/picotool)
