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

[//]: # (TODO)
1. First step here
2.
3.

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
* [ ] Support external connections, ideally [without port forwarding](https://stackoverflow.com/questions/16908714/how-do-you-create-a-peer-to-peer-connection-without-port-forwarding-or-a-centera)
* [ ] Use threads to wait for things instead of polling
