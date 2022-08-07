use clap::{App, Arg};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader, Read},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("FacesPass <https://github.com/FacesPass>")
        .about("Rust head. View the specified number of lines in front of the file")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines")
                .help("Number of bytes"),
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .get_matches();

    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();
    for (file_num, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(e) => eprintln!("{}, {}", filename, e),
            Ok(mut file) => {
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        filename
                    );
                }
                if let Some(num_bytes) = config.bytes {
                    // BufRead::take 去读取指定数量的字节
                    let mut handle = file.take(num_bytes as u64);
                    // 创建一个指定长度的 vec 并填充 0 去接收从文件中读取到的字节
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    // 使用 Iterator::take 选择指定的行数去读取，但是这样去读会丢失每一行最后一个换行符
                    // 所以得换一种方式去读
                    // for line in file.lines().take(config.lines) {
                    //     println!("{}", line?);
                    // }

                    // 创建一个空的 String buffer 去存取每行读取到的数据
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        // 这里的 read_line 用的是 BufRead 上的方法
                        // 直到遇到一个换行符的时候就会读取结束，并且换行符会被拼到 Buffer 上
                        let bytes = file.read_line(&mut line)?;

                        // 遇到0的时候就表明读取结束
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

/**
 * 命令行读取到的参数都是字符串类型的
 * 将字符串类型的数字转成数字类型
*/
fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        // if n > 0 是一个守卫
        Ok(n) if n > 0 => Ok(n),
        // From::from 方法将 Error 类型转化成 Box<dyn Error> 类型
        // 或者使用 Into / into()
        // _ => Err(From::from(val)),
        // _ => Err(Into::into(val)),
        _ => Err(val.into()),
    }
}

#[test]
fn test_parse_positive_int() {
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // 不是数字字符串将会转化出错
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // 0 不是正数，也要报错
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
