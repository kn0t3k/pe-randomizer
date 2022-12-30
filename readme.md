# PE Randomizer

This is a simple utility that can randomize some properties of a PE file.

It is also a Rust-lang learning project for me, so bear with me if some constructs are not ideal.

I will describe some properties of a PE file and show how this utility can modify them.
Why would you modify them?
Well you might be a pentester trying to lay low and make each of your EXE slightly different.
Or you just want to play around with some EXEs.

# PE Timestamp

The timestamp of a PE file is a basic property that is automatically set by the linker.
There are many articles about PE timestamps, their meaning, the attacker's timezone, the attacker's campaigns etc.
Long story short - PE timestamps can shine a bit of light on the malware and we might try to avoid that (for whatever reason).

There are multiple timestamps, this utility reads/modifes the TimeDateStamp of the COFF File Header.
Use `--help` to show how the timestamp can be modified.