## Overview

**Project Title**: P2P Chat

**Project Description**: A P2P chat application.

**Project Goals**: This project aims to provide simple text chatting capabilities without relying on any host server. However, it is a toy project, is not secure, and is not intended for actual use.

## Instructions for Build and Use

Steps to build and/or run the software:

```bash
cargo run
```

Instructions for using the software:

1. Run the software twice in parallel.
2. Provide each instance with a screen name and a port to listen on. (Make sure to provide a high enough number for the port, as the first thousand or so require elevated permissions.)
3. Connect one instance to the other by providing it with the other instance's port.
4. Type in each instance to send text to the other.

## Development Environment

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* Install rust: https://www.rust-lang.org/tools/install
* Set up the pre-commit hook:
```bash
git config --local core.hooksPath .githooks
```

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [The Rust book](https://doc.rust-lang.org/book/)
* [The Rust std documentation](https://doc.rust-lang.org/std/)

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [ ] Support more than two peers
* [ ] Use threads to wait for things instead of polling
* [ ] Disconnect/quit with the escape key instead of "exit". (The escape key can be detected by checking for its keycode, '\x1b', but detecting the key as soon as it's pressed requires external crates.)
* [ ] Improve robustness: Validate request line datatypes (eg chat message vs screen name), handle interrupted read calls, handle errors/unexpected timings better during session setup, possibly add more checks for session stability (eg heartbeats).
