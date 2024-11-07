# Wake Up!

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Rocket](https://img.shields.io/badge/rocket-%23d43949.svg?style=for-the-badge&logo=rocket&logoColor=white)
![Alpine Linux](https://img.shields.io/badge/Alpine_Linux-%230D597F.svg?style=for-the-badge&logo=alpine-linux&logoColor=white)
![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)

## Pages

### Login

The prupose of this page is to authentify the user. You need to use enter the password declared in the configuration file.

### Wake up

In this page you can wake up an computer or a group of computer.

## API

+ **POST** */api/login*
+ **POST** */api/hosts/<name>*
+ **POST** */api/groups/<groupname>*
+ **POST** */api/groups/<groupname>/<hostname>*
+ **GET** */api/configuration/reload*

## Configuration

Here is example of configuration file:
```yml
password: DontUseThisPassword
groups:
  groupe1:
    machine1.1:
      port: 9
      address: 3A:1F:5D:7C:8A:3B
    machine1.2:
      port: 7
      address: C4:22:5B:0D:9E
  groupe2:
    machine2.1:
      address: 9D:2B:4F:7A:12
    machine1.2:
      port: 6
      address: A0:8C:3D:5E:9F:76
hosts:
  machine1.1:
    address: F1:6A:4B:3C:9D:21
  machine3:
    port: 6
    address: B3:11:8E:9F:4A:5D
```

## Build from source

If you don't find a binary or docker image that satisfies your need, you can follow this process.
```sh
# Download the rust toolchain 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # From https://rustup.rs/
# Clone this repo 
git clone https://github.com/TheBaronMc/wake-up.git
```

### Build for your machine

It is pretty straightforward.
```sh
cd wake-up
cargo build --release
```
After that you will find a binary named `wake-up' under `./target/release`.

### Build for a specific platform

You can find all available targets by running this command.
```sh
rustup target list
rustup target add <target>
```
Then run specify the target in the build command.
```sh
cd wake-up
cargo build --release --target <target>
```

## Developer

```sh
git config --local include.path ../.gitconfig
```
