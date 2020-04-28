# IPC via SIGUSR1 and SIGUSR2 using Morse code

@Profpatsch on Twitter
[tweeted](https://twitter.com/Profpatsch/status/1255072730921099265) this idea
without a further comment. I recently learned morse code and thought that was
an interesting idea, so I decided to implement it.

## Running
To run it, you need two terminals, one to start a program that waits for IPC
and the other, which takes input and signals it to the other.

First start the receiver

```
$ cargo run -- in
Listening for signals at PID 14032
```

and then the sender with that PID as argument. When it's started, you can
enter text. On every newline this text is sent to the other process and should
appear in its stdout.

```
$ cargo run -- out 7036
> Hello World
Sending to 7036 as Short Short Long Short Short Short Long Short Short Long Long Long Long Long Long Short Long Short Short Long Short Short Long Short Short
>
```
