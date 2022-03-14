use std::env;
use std::io::{self, Write};
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{time, time::Duration, time::Instant};

use signal_hook::{iterator::Signals, SIGUSR1, SIGUSR2};

use MorseSignal::{Long, Short};

fn server() {
    println!("Listening for signals at PID {}", process::id());
    let signals = Signals::new(&[SIGUSR1, SIGUSR2]).expect("Couldn't register signal handlers");

    let mut signal_mutex: Arc<Mutex<Vec<MorseSignal>>> = Arc::new(Mutex::new(vec![]));
    let thread_queue_mut = signal_mutex.clone();

    let mut start_mutex: Arc<Mutex<Option<Instant>>> = Arc::new(Mutex::new(None));
    let thread_start_mut = start_mutex.clone();

    thread::spawn(move || {
        for signal in signals.forever() {
            let signal = MorseSignal::from_int(signal)
                .expect("Handled signal other than SIGUSR1 or SIGUSR2");

            let mut mut_start = thread_start_mut.lock().unwrap();
            let timer_hint = match *mut_start {
                None => Duration::from_millis(10),
                Some(last_signal_time) => last_signal_time.elapsed(),
            };

            let mut signal_queue = thread_queue_mut.lock().unwrap();
            match round_nearest_multiple(timer_hint.as_millis() as u64, 10) {
                // New signal
                10 => {
                    (*signal_queue).push(signal);
                }
                // New letter
                30 => {
                    let decoded = MorseSignal::to_char(&*signal_queue);
                    print!("{}", decoded);

                    (*signal_queue).clear();
                    (*signal_queue).push(signal);
                    *mut_start = Some(Instant::now());
                }
                // Space
                70 => {
                    let decoded = MorseSignal::to_char(&*signal_queue);
                    print!("{}", decoded);

                    (*signal_queue).clear();
                    (*signal_queue).push(signal);
                    *mut_start = Some(Instant::now());

                    print!(" ");
                }
                x => panic!("Other time {}", x),
            }

            *mut_start = Some(Instant::now());
        }
    });

    let loop_queue_mut = signal_mutex.clone();
    let loop_start_mut = start_mutex.clone();
    loop {
        thread::sleep(time::Duration::from_millis(10));
        let mut mut_start = loop_start_mut.lock().unwrap();
        if let Some(start) = *mut_start {
            let mut signal_queue = loop_queue_mut.lock().unwrap();

            // End of message
            let time = round_nearest_multiple(start.elapsed().as_millis() as u64, 10);
            if time > 70 && (*signal_queue).len() > 0 {
                let decoded = MorseSignal::to_char(&*signal_queue);
                println!("{}", decoded);

                (*signal_queue).clear();
                *mut_start = None;
            }
        }
    }
}

// Or rounds down if in the middle
fn round_nearest_multiple(number: u64, multiple: u64) -> u64 {
    let diff_to_lower = number % multiple;
    if diff_to_lower == 0 {
        number
    } else if diff_to_lower <= multiple / 2 {
        multiple * (number / multiple)
    } else {
        multiple * (1 + (number / multiple))
    }
}

fn client(server_pid: u32) {
    println!("Please type messages to send to PID {}:", server_pid);
    loop {
        print!("> ");
        io::stdout().flush().expect("Could not flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        send_morse(server_pid, &input.trim());
    }
}

#[derive(Debug, PartialEq, Eq)]
enum MorseSignal {
    Short,
    Long,
}

impl MorseSignal {
    fn from_int(signal: i32) -> Option<MorseSignal> {
        match signal {
            10 => Some(Short),
            12 => Some(Long),
            _ => None,
        }
    }

    fn to_int(&self) -> i32 {
        match self {
            Short => 10,
            Long => 12,
        }
    }

    fn from_char(c: char) -> Vec<MorseSignal> {
        match c {
            'a' => vec![Short, Long],
            'b' => vec![Long, Short, Short, Short],
            'c' => vec![Long, Short, Long, Short],
            'd' => vec![Long, Short, Short],
            'e' => vec![Short],
            'f' => vec![Short, Short, Long, Short],
            'g' => vec![Long, Long, Short],
            'h' => vec![Short, Short, Short, Short],
            'i' => vec![Short, Short],
            'j' => vec![Short, Long, Long, Long],
            'k' => vec![Long, Short, Long],
            'l' => vec![Short, Long, Short, Short],
            'm' => vec![Long, Long],
            'n' => vec![Long, Short],
            'o' => vec![Long, Long, Long],
            'p' => vec![Short, Long, Long, Short],
            'q' => vec![Long, Long, Short, Long],
            'r' => vec![Short, Long, Short],
            's' => vec![Short, Short, Short],
            't' => vec![Long],
            'u' => vec![Short, Short, Long],
            'v' => vec![Short, Short, Short, Long],
            'w' => vec![Short, Long, Long],
            'x' => vec![Long, Short, Short, Long],
            'y' => vec![Long, Short, Long, Long],
            'z' => vec![Long, Long, Short, Short],
            _ => vec![],
        }
    }

    #[rustfmt::skip]
    fn to_char(signals: &[MorseSignal]) -> char {
        match signals {
          [Long,  Short, Short, Short]  => 'b',
          [Long,  Short, Long,  Short]   => 'c',
          [Short, Short, Long,  Short]  => 'f',
          [Short, Short, Short, Short] => 'h',
          [Short, Long,  Long,  Long ]   => 'j',
          [Short, Long,  Short, Short]  => 'l',
          [Short, Long,  Long,  Short]   => 'p',
          [Long,  Long,  Short, Long ]   => 'q',
          [Short, Short, Short, Long ] => 'v',
          [Long,  Short, Short, Long ]  => 'x',
          [Long,  Short, Long,  Long ]   => 'y',
          [Long,  Long,  Short, Short]   => 'z',
          [Long,  Short, Short       ]  => 'd',
          [Long,  Long,  Short       ]   => 'g',
          [Long,  Short, Long        ]  => 'k',
          [Long,  Long,  Long        ]   => 'o',
          [Short, Long,  Short       ]  => 'r',
          [Short, Short, Short       ] => 's',
          [Short, Short, Long        ] => 'u',
          [Short, Long,  Long        ]  => 'w',
          [Short, Long               ] => 'a',
          [Long,  Long               ]  => 'm',
          [Short, Short              ] => 'i',
          [Long,  Short              ]  => 'n',
          [Short                     ] => 'e',
          [Long                      ] => 't',
          _                            => panic!("Invalid code: {:?}", signals),
        }
    }
}

fn send_morse(pid: u32, content: &str) {
    print!("Sending to {} as", pid);

    for c in content.chars() {
        if c.is_whitespace() {
            // 40 = 70 - 20 - 10
            thread::sleep(time::Duration::from_millis(40));
            continue;
        }

        for signal in MorseSignal::from_char(c) {
            print!(" {:?}", signal);
            unsafe {
                libc::kill(pid as i32, signal.to_int());
            }
            thread::sleep(time::Duration::from_millis(10));
        }

        // Sleep extra 20 so that we sleep 30ms in between characters
        thread::sleep(time::Duration::from_millis(20));
    }
    println!("");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "in" => server(),
        "out" => {
            if args.len() <= 2 || args[2].parse::<u32>().is_err() {
                println!("Must provide PID as second argument");
                return;
            }

            client(args[2].parse().unwrap())
        }
        _ => println!("Must provide either `in` or `out` as first argument"),
    }
}
