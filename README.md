## P2P Chat

This project aims to provide simple text chatting capabilities without relying on any host server. However, it is a toy project, is not secure, and is not intended for actual use.

#### Instructions

Build with:
```bash
cargo run
```

To use:

1. Run the software twice in parallel.
2. Provide each instance with a screen name and a port to listen on. (Make sure to provide a high enough number for the port, as the first thousand or so require elevated permissions.)
3. Connect one instance to the other by providing it with the other instance's port.
4. Type in each instance to send text to the other.

Typing "exit" in either instance will terminate the connection instead of sending the message "exit".

#### Development Environment

Everything is standard git/rust, except that the git hooks are stored in the `.githooks` directory. Run this command when setting up a new environment:
```bash
git config --local core.hooksPath .githooks
```

#### Ideas for further development

* [ ] Support more than two peers
* [ ] Use threads to wait for things instead of polling
* [ ] Disconnect/quit with the escape key instead of "exit". (The escape key can be detected by checking for its keycode, '\x1b', but detecting the key as soon as it's pressed requires external crates.)
* [ ] Improve robustness: Validate request line datatypes (eg chat message vs screen name), handle interrupted read calls, handle errors/unexpected timings better during session setup, possibly add more checks for session stability (eg heartbeats).
