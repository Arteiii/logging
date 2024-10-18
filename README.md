# Logging Service

Work in Progress: This project is currently under development and may not yet be fully stable or feature-complete.

AN extensible logging service designed to provide logging and client management functionalities.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [API Reference](#api-reference)
- [License](#license)
- [Contributing](#contributing)
- [Contact](#contact)

## Features

- **Application and Client Authentication**: Secure application and client registration with API keys.
- **Flexible Logging Levels**: Log messages with different severity levels (INFO, WARNING, ERROR).
- **Simple Integration**: gRPC-based service with well-defined protobuf schemas.

## Getting Started

To get started with the Logging Service, follow these instructions:

### Prerequisites

- [Rust](https://www.rust-lang.org/learn/get-started) (for service implementation)
- [Protocol Buffers Compiler (`protoc`)](https://grpc.io/docs/protoc-installation/)

### Installation

1. **Clone the Repository:**

    ```bash
    git clone https://github.com/Arteiii/logging.git
    cd logging
    ```

2. **Run the Service:**

    ```bash
    cargo run
    ```

### Using `nix-shell`

1. **Enter the Development Shell**:

   Navigate to your project directory and run:

   ```bash
   nix-shell
   ```

   This command will drop you into a shell with all the necessary tools for building and running the service,
   including Rust and the Protocol Buffers Compiler (`protoc`).

2. **Build and Run the Service**:

   Inside the `nix-shell`, you can build and run the service as usual:

   ```bash
   cargo run
   ```

## Usage

### gRPC Service

The Logging Service exposes the following gRPC methods:

- **Log**: Log a message from a client.
- **RegisterApplication**: Register a new application with the service.
- **RegisterClient**: Register a new client with the service.

### Example

#### Log Request

```json
{
  "auth": {
    "app_name": "MyApp",
    "api_key": "abc123xyz"
  },
  "client_auth": {
    "client_id": "client-456",
    "api_key": "def789ghi"
  },
  "level": "INFO",
  "message": "User successfully logged in"
}
```

#### Register Application

```json
{
  "name": "MyApp"
}
```

#### Register Client

```json
{
  "auth": {
    "app_name": "MyApp",
    "api_key": "abc123xyz"
  },
  "client_name": "ClientA",
  "client_secret": "supersecret"
}
```

## Running Tests

To run the tests using ghz, use the following command:

```shell
ghz --config=testing/test.json --insecure --concurrency=100 --rps=5000 --total=10000 --duration=2m --proto=proto/logging.proto -- 0.0.0.0:4444
```

## API Reference

### `LogRequest`

- **auth**: `ApplicationAuth` - Application credentials.
- **client_auth**: `ClientAuth` - Client credentials.
- **level**: `LogLevel` - Severity level of the log.
- **message**: `string` - The log message.

### `RegisterAppRequest`

- **name**: `string` - Name of the application.

### `RegisterAppResponse`

- **status**: `StatusResponse` - Status of the registration request.

### `RegisterClientRequest`

- **auth**: `ApplicationAuth` - Application credentials.
- **client_name**: `string` - Name of the client.
- **client_secret**: `string` - Client secret.

### `RegisterClientResponse`

- **status**: `StatusResponse` - Status of the registration request.
- **client_id**: `string` - Unique identifier for the client.


## License

This project is licensed under the 
[GNU Affero General Public License (AGPL) v3.0](https://www.gnu.org/licenses/agpl-3.0.html).
See the [LICENSE](LICENSE-AGPLv3) file for details.

## Contributing

To see the list of all planned features, fixes, and enhancements for Version 1 of the project, 
please refer to the [Version 1 Issue](https://github.com/Arteiii/logging/issues/1). 
This issue tracks all ongoing and upcoming work.

## Contact

For questions or support, please reach out to [ben.arteii@proton.me](mailto:ben.arteii@proton.me).
