# rust-junk
This is where I put the simple projects I wrote along the way of learning Rust.
Enjoy!

## sdencrypt
This little thing is supposed to become a basic file encryption program. As of
now, it's capable of reading a password and cleartext on stdin and showing the
different forms it takes when fed through `rust-crypto`'s AES 256-bit
implementation. It also includes my own PKCS#7 padding implementation for
block-aligning the specified cleartext.

## ff_stats
`ff_stats` will become a tool for dumping Firefox usage statistics from its
`places.sqlite` database. As of now, it dumps the 10 most recent visits in the
browser history.
