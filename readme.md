# PE Randomizer

This is a simple utility that can randomize some properties of a PE file.

It is mostly a Rust-lang learning project for me, so bear with me if some constructs are not ideal.

Why would you modify properties of a PE file?
Well you might be a pentester trying to lay low and make each of your EXEs slightly different.
Or you just want to play around with some PE file.

# PE Timestamp

The timestamp of a PE file is a basic property that is automatically set by the linker.
There are many articles about PE timestamps, their meaning, the attacker's timezone, the attacker's campaigns etc.
Long story short - PE timestamps can shine a bit of light on the malware and we might try to avoid that (for whatever reason).

There are multiple timestamps, this utility can read/modify the File Header, import, export and resource timestamps.

# TODO

* finish interactive CLI
* set more types of timestamps
* set/get/explore other properties of the PE file (compiler, linker, section names, etc.)
* add tests