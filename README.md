<h2>Backstory</h2>

After constructing my home, I found myself facing a common dilemma that every tech enthusiast encounters: How do I make my home smart? How do I simplify my life? As I began researching, experimenting, and acquiring various devices, it became apparent that there was no single solution to meet all my needs.

I yearned for a modern, reliable, secure, and high-performance system. I needed something easy to set up, capable of self-adaptation, and, at the same time, offering a high degree of customization. Despite the numerous existing solutions, none personally resonated with me. Hence, I decided to create my own system and make it accessible to those who found themselves in a similar situation or were eager to explore something new and innovative.

The core motivation behind this project is to eliminate the hours spent creating a functional setup, especially when adding a new device. Imagine having a system that automatically recognizes patterns and assists you during the configuration process—how cool would that be? This open-source project aims to provide a comprehensive solution for individuals seeking a smart home experience that seamlessly combines modern technology, reliability, security, and personalization. Feel free to join the community and contribute to building the future of smart home automation.

<h2>So basically what is Sync?</h2>

Sync is a pioneering project that aims to revolutionize your home experience. Imagine a world where your home effortlessly caters to your needs, prioritizing top-notch performance, security, and reliability.

A comprehensive solution for modern home management, Sync empowers you to take charge of your living space by combining advanced technology, extensive customization options, and an intuitive interface.

Whether you want to manage lighting, fine-tune the temperature, enhance security, or optimize entertainment, we transcend conventional home assistance. We are your steadfast ally in shaping a smarter, more comfortable living environment.

<h2>How are we building it?</h2>

Creating a project of this magnitude requires extensive time dedicated to developing the architectural framework and meticulous planning. It's essential that everything functions as intended, ensuring optimal performance and stability.

For this project, we've selected Rust as our programming language of choice, given its reputation as one of the finest languages currently available.

Feel free to check out our [wiki]() for a clear explanation of what we're building, how we're doing it, and to get familiar with our development process. It's a great way to make yourself comfortable with our project.

<h2>Developing</h2>

<h3>Requirements</h3>

- Rust Toolchain (>1.70.0)

After cloning the repo, you can either use nix to enter a custom shell or use cargo make commands

```sh
$ cd sync
$ cargo install --force cargo-make
$ cargo make setup
$ cargo run
```

with nix

```sh
$ cd sync
$ nix develop
$ setup && run
```

You'll need to set a password during the setup, which will be used to encrypt all secrets. This password can also be utilized with [ssclient](https://crates.io/crates/ssclient/).

<b>Important</b>: The setup process might take a few minutes as Cargo is building all the dependencies.

---

Keep in mind that the project is in its early development stage, so there's currently no user interface available. To view results, you can inspect the database instance, with [Surrealist][surrealist] being the recommended tool for this purpose.

<h2>Contributing</h2>

We are actively seeking support to develop this ambitious project, currently spearheaded by a solo enthusiast – humorously questioning whether one person constitutes a team. 😊

For guidance on how to begin contributing, please refer to our [Contributing Guide][contributing-guide].

<h2>License</h2>

Our material is available under various licenses:

- All libraries and Software Development Kits (SDKs) are distributed under the Apache License 2.0 or MIT.
- All final interfaces or services are distributed under the Apache License 2.0 or MIT.
- The primary code found in this repository within the core folder is distributed under the Business Source License 1.1.

[surrealist]: https://surrealist.app
[contributing-guide]: CONTRIBUTING.md

<h2>Statistics</h2>

![Alt](https://repobeats.axiom.co/api/embed/8ae8b959850dd11b7d3a6279c76c094bdf8fa339.svg "Repobeats analytics image")