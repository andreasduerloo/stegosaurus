# stegosaurus

Stegosaurus is a steganography tool written in Rust. It allows hiding plaintext messages in .bmp files, as well as decoding those messages from .bmp files. The program assumes the image file is little-endian.

Usage:

-d/--decode imagefile

-e/--encode imagefile textfile outputfile

You can use either UNIX-style forward slashes or Windows-style backslashes when passing paths.
