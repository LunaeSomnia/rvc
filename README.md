# rvc

## **R**ust **V**ersion **C**ontrol

A version control software written in Rust.
This repository is created as a fun side project and it is not intended for general public use.

It is based on a challenge from the `Pro/g/ramming Challenges, v4.0` problem set, number 24, described as:

> *Simple Version Control supporting checkout, commit (with commit messages), unlocking, and per-file configuration of number of revisions kept.*

If you feel like I could have improved something in this project, please let me know!

## To-do:

- [x] Save state locally (under user's default home directory as `.rvc` using the `ron` crate)
- [x] Save which repositories are being tracked with commands `create` and `delete`
- [x] Review state with the `state` command
- [ ] Identify file changes under a given repository from a previous state in time
- [ ] Save states under commits
- [ ] Add the *number of revisions* functionality
