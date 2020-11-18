# Von Neumann Defense Force ![Build, Test, Release, Deploy](https://github.com/hannobraun/vndf/workflows/Build,%20Test,%20Release,%20Deploy/badge.svg)

## About

Von Neumann Defense Force is a game about spaceships. It is the continuation of [an earlier project](https://github.com/hannobraun/vndf-2016) of the same name.

![a screenshot from the game](screenshots/2020-07-04.png)

This project is, as far as I'm concerned, done. I did some of the things I wanted to do, others I either don't want anymore, or I've figured out that this is the wrong approach of doing them. This never turned into a coherent game (see *Status*), but it's time to move on to other things.

## Status

This was a hobby project with no clear goal in mind. As such, I only worked on whatever interested me, while leaving other things in half-done or broken states.

From a gameplay perspective, there's not a lot there. We have working multiplayer, and there are game mechanics, but they don't form a coherent whole.

From a technical perspective, the code was in a constant state of transition. Most of the gameplay code is basically an experiment in how to structure code such as this, only partially with satisfactory results. Little of that has been documented.

## Instructions

### Quickly testing the game

```
cargo run
```

This starts a test client with a built-in server. This is useful for quickly testing stuff, but it doesn't provide any multiplayer features.

### Run the game with a local server

Start the local server:
```
cd vndf-server
cargo run
```

In another terminal, start a client to connect to that server:
```
cd vndf-launcher
cargo run -- --local
```

Using this technique, you can connect with any number of clients, until you run into scalability issues. As of this writing, games have been run with up to three clients.

### Play the game on the official test server

```
cd vndf-launcher
cargo run
```

Please note that, as per above, this game is no longer developed. I deployed the latest server version when I updated this text, so it should work for the time being. However, I don't intend to go into any effort to keep the server up and running, should that turn into an inconvenience.

## License

All code in this repository is available under the [Zero-Clause BSD License](https://opensource.org/licenses/0BSD) (0BSD). This basically means you can do anything with it, without any restrictions.

Here's the full license text:

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby granted.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
