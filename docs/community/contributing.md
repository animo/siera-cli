---
description: >-
  Feel free to contribute to the repository by forking and submitting a pull
  request!
---

# Contributing

We welcome contributions that to make the CLI better. If you do not have a specific contribution in mind, we encourage you to look for issues labelled as [`good first issue`](https://github.com/animo/siera/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) or look at our [roadmap](roadmap.md).

For significant changes, please open an issue first to discuss the proposed changes with the community to avoid re-work.

(If you are new to GitHub, you might start with a [basic tutorial](https://docs.github.com/en/get-started/quickstart/set-up-git) and check out a more detailed guide to [pull requests](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/proposing-changes-to-your-work-with-pull-requests/about-pull-requests).)

Pull requests will be evaluated by the repository guardians on a schedule and if deemed beneficial will be committed to the main branch. Pull requests should have a descriptive name and include a summary of all changes made in the pull request description.

### Fixes

If you have a fix, go straight to [Getting set up](contributing.md#getting-set-up).

### Features

If you have something bigger in mind (structural changes, new features) we strongly encourage you to create a new GitHub issue that documents:

- motivate the changes you want to make
- how you want to make them

This gives your new idea the best chance to get accepted by the repository maintainers.

### Getting set up

```
git clone https://github.com/animo/siera.git
```

Once you have the code locally, you should be able to test out the current source code by running:

```
cargo run -q -- <cmd>
```

#### Tests

We love tests, but recognize that there is a shortage of them at the moment. We encourage you to take a look at [Rust's guide](https://doc.rust-lang.org/book/ch11-01-writing-tests.html) on how to create an automated test. We are happy to provide support for writing tests on the PR.

Currently a simple suite of tests can be executed by running `./tests/run.sh`.
