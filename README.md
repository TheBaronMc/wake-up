# Wake Up!

<p align="center">
  <img src="https://github.com/user-attachments/assets/1f31e31a-a39a-4b22-b352-a0fe538aab2c" width="120" alt="Wake Up! Logo" />
</p>
<p align="center">Wake On Lan web application</p>
<p align="center">
  <a href="https://www.rust-lang.org/" target="_blank"><img src="https://img.shields.io/badge/rust-%23000000.svg?logo=rust&logoColor=white" alt="Rust" /></a>
  <a href="https://rocket.rs/" target="_blank"><img src="https://img.shields.io/badge/rocket-%23d43949.svg?logo=rocket&logoColor=white" alt="Rocket" /></a>
  <a href="https://alpinelinux.org/" target="_blank"><img src="https://img.shields.io/badge/Alpine_Linux-%230D597F.svg?logo=alpine-linux&logoColor=white" alt="Alpine" /></a>
  <a href="https://www.docker.com/" target="_blank"><img src="https://img.shields.io/badge/docker-%230db7ed.svg?logo=docker&logoColor=white" alt="Docker" /></a>
</p>

## Pages

### Login

The prupose of this page is to authentify the user. To do so, use the password delcared in the configuration file.

### Wake up

In this page you can wake up a computer or a group of computer.

## API

<details>
 <summary><b>POST</b> <code>/api/login</code> Authentify and give api token</summary>

##### Parameters

> | name          |  type     | data type               | description                                                           |
> |---------------|-----------|-------------------------|-----------------------------------------------------------------------|
> | password      |  required | object (JSON)           | User password |


##### Responses

> | http code     | content-type                      | response                                                            |
> |---------------|-----------------------------------|---------------------------------------------------------------------|
> | `200`         | `application/json`                | `{ "token": your_token }`                                           |
> | `401`         | `application/json`                | `{"code":"401","message": error_message}`                           |

</details>

<details>
 <summary><b>POST</b> <code>/api/groups/<b>{group_name}</b></code> Wakes up the host with the corresponding name</summary>
> ⚠️ 🛑 A token is required to use this route 🛑 ⚠️

##### Parameters

> None


##### Responses

> | http code     | content-type                      | response                                                            |
> |---------------|-----------------------------------|---------------------------------------------------------------------|
> | `200`         | `application/json`                | None                                                                |
> | `401`         | `application/json`                | `{"code":"401","message": error_message}`                           |
> | `404`         | `application/json`                | `{"code":"404","message": "Not Found"}`                             |
> | `500`         | `application/json`                | `{"code":"500"}`                                                    |

</details>

<details>
 <summary><b>POST</b> <code>/api/groups/<b>{name}</b></code> Wakes up the group with the corresponding name</summary>
> ⚠️ 🛑 A token is required to use this route 🛑 ⚠️

##### Parameters

> None


##### Responses

> | http code     | content-type                      | response                                                            |
> |---------------|-----------------------------------|---------------------------------------------------------------------|
> | `200`         | `application/json`                | None                                                                |
> | `401`         | `application/json`                | `{"code":"401","message": error_message}`                           |
> | `404`         | `application/json`                | `{"code":"404","message": "Not Found"}`                             |
> | `500`         | `application/json`                | `{"code":"500"}`                                                    |

</details>

<details>
 <summary><b>POST</b> <code>/api/groups/<b>{group_name}</b>/<b>{host_name}</b></code> Wakes up the host with the corresponding name within the group with the corresponding name</summary>
> ⚠️ 🛑 A token is required to use this route 🛑 ⚠️

##### Parameters

> None


##### Responses

> | http code     | content-type                      | response                                                            |
> |---------------|-----------------------------------|---------------------------------------------------------------------|
> | `200`         | `application/json`                | None                                                                |
> | `401`         | `application/json`                | `{"code":"401","message": error_message}`                           |
> | `404`         | `application/json`                | `{"code":"404","message": "Not Found"}`                             |
> | `500`         | `application/json`                | `{"code":"500"}`                                                    |

</details>

<details>
 <summary><b>GET</b> <code>/api/groups/reload</code> Reload the configuration</summary>

> ⚠️ 🛑 A token is required to use this route 🛑 ⚠️

##### Parameters

> None


##### Responses

> | http code     | content-type                      | response                                                            |
> |---------------|-----------------------------------|---------------------------------------------------------------------|
> | `200`         | `application/json`                | None                                                                |
> | `400`         | `application/json`                | `{"code":"400","message": error_message}`                           |
> | `401`         | `application/json`                | `{"code":"401","message": error_message}`                           |

</details>

## Configuration

### Configuration file

Located at the root of the projet, somberly named *configuration.yml*, the file content is in YAML format.

| Level | Keyword       | Description        | Default value |
|-------|---------------|--------------------|---------------|
| 0     | `password`    | Login Password     | wake-up!      |
| 0     | `api_enabled` | Enable the API     | true          |
| 0     | `web_enabled` | Enable the Web app | true          |
| 0     | `port`        | Application port   | 8999          |
| 0     | `groups`      | Group List         | /             |
| 1     | `<GroupName>` |                    | /             |
| 2     | `<HostName>`  |                    | /             |
| 3     | `port`        | WakeOnLan port     | 6             |
| 3     | `address`     | MAC address        | /             |
| 0     | `hosts`       |                    | /             |
| 1     | `<HostName>`  |                    | /             |
| 2     | `port`        | WakeOnLan port     | 6             |
| 2     | `address`     | MAC address        | /             |

Here is example of configuration file:
```yml
password: wake-up!
api_enabled: false
web_enabled: true
port: 12345
groups:
  DHCP:
    Server1:
      address: 9D:2B:4F:7A:12
    Server2:
      port: 6
      address: A0:8C:3D:5E:9F:76
hosts:
  RaspberryPi:
    address: F1:6A:4B:3C:9D:21
  NAS:
    port: 6
    address: B3:11:8E:9F:4A:5D
```

### Environment variables

You can overwrite configuration by using the follow environment variables:
+ `WAKE_UP_PORT`
+ `WAKE_UP_PASSWORD`
+ `WAKE_UP_API_ENABLED`
+ `WAKE_UP_WEB_ENABLED`

## Security

_Wake Up!_ doesn't support certificats (yet?) so you need to protect it behind a reverse proxy wich have TLS active or find a way to encrypt the communications.

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
After that you will find a binary named `wake-up` under `./target/release`.

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
