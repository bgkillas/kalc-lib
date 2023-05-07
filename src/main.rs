mod complex;
mod math;
mod parse;
use parse::get_func;
use complex::parse;
use math::do_math;
use std::env::{args, var};
use std::io::{BufRead, BufReader, stdout, Write};
use console::{Key, Term};
#[cfg(target_os = "linux")]
use std::io::stdin;
#[cfg(target_os = "linux")]
use libc::{isatty, STDIN_FILENO};
use std::fs::{File, OpenOptions};
use gnuplot::{AxesCommon, Caption, Figure, PointSymbol};
fn main()
{
    if args().len() > 1
    {
        if args().nth(1).unwrap() == "--help"
        {
            println!("Type in a function to evaluate it. Type \"exit\" to exit. Type \"clear\" to clear the screen. Type \"help\" to show this message.");
            println!("functions: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, sqrt, cbrt, ln, log(base,num), abs, dg(to_degrees),rd(to_radians)");
            return;
        }
        let func = get_func(&args().nth(1).unwrap(), true);
        if func.contains(&"x".to_string())
        {
            if func.contains(&"y".to_string())
            {
                let mut re3d = true;
                let mut im3d = true;
                for i in 2..args().len()
                {
                    if args().nth(i).unwrap() == "--nore"
                    {
                        re3d = false;
                    }
                    if args().nth(i).unwrap() == "--noim"
                    {
                        im3d = false;
                    }
                }
                graph(&func, true, im3d, re3d);
                return;
            }
            graph(&func, false, true, false);
            return;
        }
        print_answer(func);
        return;
    }
    let mut input = String::new();
    #[cfg(target_os = "linux")]
    if !unsafe { isatty(STDIN_FILENO) != 0 }
    {
        let line = stdin().lock().lines().next();
        if line.as_ref().is_none()
        {
            return;
        }
        input = line.unwrap().unwrap();
        if input.is_empty()
        {
            return;
        }
        print_answer(get_func(&input, true));
        return;
    }
    #[cfg(target_os = "linux")]
    let file_path = &(var("HOME").unwrap() + "/.config/calc.history");
    #[cfg(target_os = "windows")]
    let file_path = &format!("C:\\Users\\{}\\AppData\\Roaming\\calc.history", var("USERNAME").unwrap());
    if File::open(file_path).is_err()
    {
        File::create(file_path).unwrap();
    }
    let mut var:Vec<Vec<char>> = Vec::new();
    loop
    {
        input.clear();
        let fg = "\x1b[96m";
        stdout().flush().unwrap();
        let mut i = BufReader::new(File::open(file_path).unwrap()).lines().count() as i32;
        let max = i;
        let mut cursor = 0;
        loop
        {
            let c = read_single_char();
            match c
            {
                '\n' =>
                {
                    println!();
                    break;
                }
                '\x08' =>
                {
                    if input.is_empty()
                    {
                        continue;
                    }
                    cursor -= 1;
                    input.remove(cursor);
                    print!("\x1B[2K\x1B[1G{}", input);
                    if input.is_empty()
                    {
                        print_concurrent(&"0".to_string(), var.clone(), true);
                    }
                    else
                    {
                        print_concurrent(&input, var.clone(), false);
                    }
                    for _ in 0..(input.len() - cursor)
                    {
                        print!("\x08");
                    }
                }
                '\x1D' =>
                {
                    i -= 1;
                    input.clear();
                    if i == -1
                    {
                        i = 0;
                        continue;
                    }
                    input = BufReader::new(File::open(file_path).unwrap()).lines().nth(i as usize).unwrap().unwrap();
                    cursor = input.len();
                    print_concurrent(&input, var.clone(), false);
                    print!("\x1B[2K\x1B[1G{fg}{}\x1b[0m", input);
                }
                '\x1E' =>
                {
                    i += 1;
                    input.clear();
                    if i >= max
                    {
                        print!("\x1B[2K\x1B[1G{fg}\x1b[0m");
                        stdout().flush().unwrap();
                        i = max;
                        cursor = 0;
                        continue;
                    }
                    input = BufReader::new(File::open(file_path).unwrap()).lines().nth(i as usize).unwrap().unwrap();
                    cursor = input.len();
                    print_concurrent(&input, var.clone(), false);
                    print!("\x1B[2K\x1B[1G{fg}{}\x1b[0m", input);
                }
                '\x1B' =>
                {
                    if cursor == 0
                    {
                        continue;
                    }
                    cursor -= 1;
                    print!("\x08");
                }
                '\x1C' =>
                {
                    if cursor == input.len()
                    {
                        continue;
                    }
                    cursor += 1;
                    print!("\x1b[1C")
                }
                _ =>
                {
                    input.insert(cursor, c);
                    cursor += 1;
                    print_concurrent(&input, var.clone(), false);
                    for _ in 0..(input.len() - cursor)
                    {
                        print!("\x08");
                    }
                }
            }
            stdout().flush().unwrap();
        }
        if input == "exit"
        {
            break;
        }
        if input == "clear"
        {
            print!("\x1B[2J\x1B[1;1H");
            stdout().flush().unwrap();
            continue;
        }
        if input == "help"
        {
            println!("Type in a function to evaluate it. Type \"exit\" to exit. Type \"clear\" to clear the screen. Type \"help\" to show this message.");
            println!("functions: sin, cos, tan, asin, acos, atan, sinh, cosh, tanh, asinh, acosh, atanh, sqrt, cbrt, ln, log(base,num), abs, dg(to_degrees),rd(to_radians)");
            continue;
        }
        if input.is_empty()
        {
            continue;
        }
        if !input.contains('=') && input.contains('x') && var.iter().all(|i| i[0] != 'x')
        {
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
            write_history(&input, file_path);
            if input.contains('y')
            {
                graph(&get_func(&input, true), true, true, true);
                continue;
            }
            graph(&get_func(&input, true), false, false, false);
            continue;
        }
        if input.contains('=')
        {
            print!("\x1B[2K\x1B[1G");
            for i in 0..var.len()
            {
                if var[i][0] == input.chars().next().unwrap()
                {
                    var.remove(i);
                    break;
                }
            }
            var.push(input.chars().collect());
            write_history(&input, file_path);
            continue;
        }
        let unmodified = input.clone();
        for i in &var
        {
            input = input.replace(&i[0..i.iter().position(|&x| x == '=').unwrap()].iter().collect::<String>(),
                                  &i[i.iter().position(|&x| x == '=').unwrap() + 1..].iter().collect::<String>());
        }
        if input.contains('x') || input.contains('y')
        {
            println!("{}", input);
            write_history(&input, file_path);
            continue;
        }
        write_history(&unmodified, file_path);
        println!();
    }
}
fn print_answer(func:Vec<String>)
{
    let num = match do_math(func)
    {
        Ok(num) => num,
        Err(_) =>
        {
            println!("\x1b[91m0\x1b[0m");
            return;
        }
    };
    let (a, b) = parse(&num);
    let a = (a * 1e9).round() / 1e9;
    let b = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &((b * 1e9).round() / 1e9).to_string() + "\x1b[93mi";
    println!("{}{}\x1b[0",
             if a == 0.0 && !(b.ends_with("0\x1b[93mi")) { "".to_string() } else { a.to_string() },
             if b.ends_with("0\x1b[93mi") { "".to_string() } else { b });
}
fn print_concurrent(input:&String, var:Vec<Vec<char>>, del:bool)
{
    let mut modified = input.to_string();
    for i in &var
    {
        modified = input.replace(&i[0..i.iter().position(|&x| x == '=').unwrap()].iter().collect::<String>(),
                                 &i[i.iter().position(|&x| x == '=').unwrap() + 1..].iter().collect::<String>());
    }
    if let Ok(num) = do_math(get_func(&modified, false))
    {
        let (a, b) = parse(&num);
        let a = (a * 1e9).round() / 1e9;
        let b = if a != 0.0 && b.is_sign_positive() { "+" } else { "" }.to_owned() + &((b * 1e9).round() / 1e9).to_string() + "\x1b[93mi";
        print!("\x1b[0m\x1b[B\x1B[2K\x1B[1G{}{}\x1b[A",
               if a == 0.0 && !(b.ends_with("0\x1b[93mi")) { "".to_string() } else { a.to_string() },
               if b.ends_with("0\x1b[93mi") { "".to_string() } else { b });
    }
    if !del
    {
        print!("\x1b[96m\x1B[2K\x1B[1G{}\x1b[0m", input);
    }
    else
    {
        print!("\x1B[2K\x1B[1G");
    }
}
fn write_history(input:&str, file_path:&str)
{
    let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
    file.write_all(input.as_bytes()).unwrap();
    file.write_all(b"\n").unwrap();
}
fn read_single_char() -> char
{
    let term = Term::stdout();
    let key = term.read_key().unwrap();
    match key
    {
        Key::Char(c) =>
        {
            if c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '*' || c == '/' || c == '^' || c == '(' || c == ')' || c == '.' || c == '=' || c == ','
            {
                c
            }
            else
            {
                read_single_char()
            }
        }
        Key::Enter => '\n',
        Key::Backspace => '\x08',
        Key::ArrowLeft => '\x1B',
        Key::ArrowRight => '\x1C',
        Key::ArrowUp => '\x1D',
        Key::ArrowDown => '\x1E',
        _ => read_single_char(),
    }
}
fn graph(func:&[String], graph:bool, im3d:bool, re3d:bool)
{
    let mut fg = Figure::new();
    if graph
    {
        let mut re = Vec::new();
        let mut im = Vec::new();
        let amount = 200;
        for n in -amount..=amount
        {
            let modified:Vec<String> = func.iter().map(|i| i.replace('x', &(n as f64 / (amount as f64 / 10.0)).to_string())).collect();
            for g in -amount..=amount
            {
                let num = match do_math(modified.iter().map(|j| j.replace('y', &(g as f64 / (amount as f64 / 10.0)).to_string())).collect())
                {
                    Ok(n) => n,
                    Err(_) =>
                    {
                        println!("0");
                        continue;
                    }
                };
                let (a, b) = parse(&num);
                let a = (a * 1e9).round() / 1e9;
                let b = (b * 1e9).round() / 1e9;
                if a != 0.0
                {
                    re.push([n as f64 / (amount as f64 / 10.0), g as f64 / (amount as f64 / 10.0), a]);
                }
                if b != 0.0
                {
                    im.push([n as f64 / (amount as f64 / 10.0), g as f64 / (amount as f64 / 10.0), b]);
                }
            }
        }
        if re3d && im3d
        {
            fg.axes3d()
              .points(re.iter().map(|i| i[0]), re.iter().map(|i| i[1]), re.iter().map(|i| i[2]), &[PointSymbol('.')])
              .points(im.iter().map(|i| i[0]), im.iter().map(|i| i[1]), im.iter().map(|i| i[2]), &[PointSymbol('.')]);
        }
        else if re3d
        {
            fg.axes3d().points(re.iter().map(|i| i[0]), re.iter().map(|i| i[1]), re.iter().map(|i| i[2]), &[PointSymbol('.')]);
        }
        else if im3d
        {
            fg.axes3d().points(im.iter().map(|i| i[0]), im.iter().map(|i| i[1]), im.iter().map(|i| i[2]), &[PointSymbol('.')]);
        }
        fg.show().unwrap();
        return;
    }
    let mut re = Vec::new();
    let mut im = Vec::new();
    for n in -50000..=50000
    {
        let modified = func.iter().map(|i| i.replace('x', &(n as f64 / 5000.0).to_string())).collect();
        let num = match do_math(modified)
        {
            Ok(n) => n,
            Err(_) =>
            {
                println!("0");
                continue;
            }
        };
        let (a, b) = parse(&num);
        let a = (a * 1e9).round() / 1e9;
        let b = (b * 1e9).round() / 1e9;
        if a != 0.0
        {
            re.push([n as f64 / 5000.0, a]);
        }
        if b != 0.0
        {
            im.push([n as f64 / 5000.0, b]);
        }
    }
    fg.axes2d()
      .set_y_range(gnuplot::AutoOption::Fix(-10.0), gnuplot::AutoOption::Fix(10.0))
      .set_x_range(gnuplot::AutoOption::Fix(-10.0), gnuplot::AutoOption::Fix(10.0))
      .points(re.iter().map(|x| x[0]), re.iter().map(|x| x[1]), &[Caption("real"), PointSymbol('.')])
      .points(im.iter().map(|x| x[0]), im.iter().map(|x| x[1]), &[Caption("imag"), PointSymbol('.')]);
    fg.show().unwrap();
}