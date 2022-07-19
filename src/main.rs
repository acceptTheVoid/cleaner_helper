use std::{fs::{read_dir, DirEntry}, path::PathBuf, env};
use cleaner_helper::{Size, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    
    let res = search_for_files(config);
    for (p, s) in res {
        if p.is_dir() {
            println!("{p:?}: {s}");
        }
    }
}

fn search_for_files(config: Config) -> Vec<(PathBuf, Size)> {
    let dir = read_dir(config.dir)
        .expect("При прочтении директории произошла ошибка");
    let mut suitable = vec![];
    
    for entry in dir {
        let entry = entry.unwrap();
        let size = rec_search(&entry, &mut suitable, &config);
        let last_accessed = entry.metadata().unwrap().accessed().unwrap();
        if size > config.min_size && last_accessed > config.time {
            suitable.push((entry.path(), size));
        }
    }
    
    suitable
}

fn rec_search(
    entry: &DirEntry, 
    suitable: &mut Vec<(PathBuf, Size)>,
    config: &Config
) -> Size {
    let mut cur_len = Size::new(0);
    let last_accessed;
    match entry.metadata() {
        Ok(data) => {
            if data.is_dir() {
                let path = entry.path();
                match read_dir(&path) {
                    Ok(dir) => {
                        for entry in dir {
                            let entry = entry.unwrap();
                            cur_len += rec_search(&entry, suitable, config);
                        }
                    },
                    Err(_) => return Size::new(0),
                }
            } else {
                cur_len += Size::new(data.len());
            }
            last_accessed = data.accessed().unwrap();
        }, 
        Err(_) => return Size::new(0),
    }

    if cur_len > config.min_size && last_accessed > config.time {
        suitable.push((entry.path(), cur_len));
    }
    
    cur_len
}
