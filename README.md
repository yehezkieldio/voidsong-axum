![Banner](.github/assets/banner.png)

## Voidsong

Voidsong is a self-hostedable API aggregator that provides random images, trivia facts, and jokes. It acts a intermediary between multiple APIs, providing a single endpoint for all your needs, and is designed to be easily extensible. The project is written in Rust, using the [Axum](https://github.com/tokio-rs/axum) web framework.

### Usage

You can start the service by building the project and running the binary, with the following command:

> **Note:** Make sure you have Rust installed on your system. You can install it from [rust-lang.org](https://www.rust-lang.org/).

```bash
cargo run --release
```

By default, the service will be available at `http://localhost:8080`, you can change the port by setting an environment variable file named `.env` with the following content:

```env
SERVER_HOST=127.0.0.1
SERVER_PORT=8080
```

Or you can use a pre-built Docker image, available on [GitHub Container Registry](https://github.com/orgs/i9ntheory/packages/container/package/voidsong).

```bash
docker pull ghcr.io/i9ntheory/voidsong:latest

docker run -d -p 8080:8080 -e SERVER_HOST=8080 ghcr.io/i9ntheory/voidsong:latest
```

### API Routes

#### Media

These routes return random images of animals.

- `GET /random/media/cat` - Returns a random cat image.
- `GET /random/media/dog` - Returns a random dog image.
- `GET /random/media/fox` - Returns a random fox image.
- `GET /random/media/bunny` - Returns a random bunny image.
- `GET /random/media/duck` - Returns a random duck image.

#### Trivia

These routes return random trivia facts.

- `GET /random/trivia/fact` - Returns a random trivia fact.
- `GET /random/trivia/catfact` - Returns a random cat fact.
- `GET /random/trivia/dogfact` - Returns a random dog fact.

#### Humor

These routes return random jokes.

- `GET /random/humor/chucknorris` - Returns a random Chuck Norris joke.
- `GET /random/humor/dadjoke` - Returns a random dad joke.

### License

This project is proudly provided under the [MIT License](LICENSE).