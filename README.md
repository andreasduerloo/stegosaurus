# stegosaurus

Stegosaurus is a steganography tool written in Rust. It allows hiding plaintext messages in .bmp files, as well as decoding those messages from .bmp files. It only works on little-endian machines.

Usage:

-d/--decode imagefile

-e/--encode imagefile textfile outputfile

Use linux-style forward slashes when passing paths.
