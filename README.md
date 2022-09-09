# Passwordo (provisional name) 🦀🦀🦀

## What is this?

A personal project that aims to generate good-enough passwords in a variety of ways.
I have made some tests like: measure the byte entropy of the passwords, guess the password with algorithms using the most common characters and digits and KeePassXC says _"good enough kiddo"_.

If you want an ultra secure "unbreakable" password this project might not be what you're looking for 😔 (we're sorry).

## About the Project

- Made with Rust and Typescript 🤘
- Generates passwords from random ascii characters (surprising I know)
- Generates passwords from a list of words + random ascii characters (ultra random ascii characters)
- Use random bytes (printed as hexadecimal) for your passwords (because why not?)

## Things to Do

- Maybe add a list of phrases from a text file as a replacement of the hard-coded words
- Add a byte entropy meter
- Add a [xkcd generation method](https://xkcd.com/936/)

![screenshot](./assets/showcase_01.png)
![screenshot](./assets/showcase_02.png)