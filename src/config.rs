
// cleaner_helper C:/ -T=4 -t=3m15d 
// запускает программу из директории C:/ с четырьмя потоками
// и маркирующую только программы со временем доступа больше 3 месяцев и 15 дней

use std::{time::{Duration, SystemTime}, path::Path};

use crate::Size;

const DEFAULT_THREAD_COUNT: usize = 1; // Один поток
const DEFAULT_DURATION: Duration = Duration::from_secs(60 * 60 * 24 * 30); // Прошел месяц
const DEFAULT_SIZE: Size = Size::new(1024 * 1024 * 100);

#[derive(Debug)]
pub struct Config<'a> {
    pub dir: &'a str,
    pub threads: usize,
    pub time: SystemTime,
    pub min_size: Size,
}

impl<'a> Config<'a> {
    pub fn new(args: &'a [String]) -> Self {
        match args.len() {
            1 => Self { 
                dir: &args[0], 
                threads: DEFAULT_THREAD_COUNT, 
                time: SystemTime::now() - DEFAULT_DURATION,
                min_size: DEFAULT_SIZE,
            },
            _ => {
                let mut dir: Option<&str> = None;
                let mut threads = DEFAULT_THREAD_COUNT;
                let mut time = DEFAULT_DURATION;

                for arg in args {
                    if let Some(t) = is_thread_arg(arg) {
                        threads = t;
                    } else if let Some(t) = is_time_arg(arg) {
                        time = t;
                    } else if Path::new(arg).exists() {
                        dir = Some(arg);
                    } else {
                        todo!("Добавить ошибку для неправильных аргументов")
                    }
                }
                
                if dir.is_none() {
                    dir = Some(&args[0]);
                }
                let dir = dir.unwrap();
                let time = SystemTime::now() - time;
                
                Self { dir, threads, time, min_size: DEFAULT_SIZE, }
            },
        }
    }
}

fn is_thread_arg(arg: &str) -> Option<usize> {
    if arg.to_lowercase().contains("threads=") || arg.contains("T=") || arg.contains("-T=") {
        let num = match arg.split("=").nth(1).unwrap().parse() {
            Ok(n) => n,
            Err(_) => todo!("Добавить кастомный выход при ошибке парсинга"),
        };
        Some(num)
    } else {
        None
    }
}  

fn is_time_arg(arg: &str) -> Option<Duration> {
    if arg.to_lowercase().contains("time=") || arg.contains("t=") || arg.contains("-t=") {
        let time = match parse_time(arg.split("=").nth(1).unwrap()) {
            Ok(n) => n,
            Err(_) => todo!("Добавить кастомный выход при ошибке парсинга"),
        };
        Some(time)
    } else {
        None
    }
}  

struct ParseTimeError;

fn parse_time(time: &str) -> Result<Duration, ParseTimeError> {
    let mut res = Duration::from_secs(0);
    let mut cur_time = 0;
    for c in time.chars() {
        if c.is_digit(10) {
            cur_time *= 10;
            cur_time += c.to_digit(10).unwrap() as u64;
        } else {
            match c {
                'd' => res += Duration::from_secs(cur_time * 3600 * 24),
                'm' => res += Duration::from_secs(cur_time * 3600 * 24 * 30),
                'y' => res += Duration::from_secs(cur_time * 3600 * 24 * 365),
                _ => todo!("Добавить обработку ошибок символов не поддерживаемых программой"),
            }
            cur_time = 0;
        }
    }

    if cur_time != 0 {
        todo!("Добавить обработку ошибок нетерминированной строки")
    }

    Ok(res)
}
