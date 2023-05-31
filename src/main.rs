mod complex;
mod equal;
mod fraction;
mod graph;
mod math;
mod parse;
mod print;
#[cfg(test)]
mod tests;
use equal::equal;
use parse::input_var;
use parse::get_func;
use std::env::{args, var};
use std::io::{BufRead, BufReader, stdout, Write};
use std::fs::{File, OpenOptions};
use gnuplot::Figure;
use graph::graph;
use print::{print_answer, print_concurrent};
#[cfg(not(unix))]
use console::{Key, Term};
#[cfg(unix)]
use {
    libc::{tcgetattr, ECHO, ICANON, TCSANOW, VMIN, VTIME, tcsetattr, isatty, STDIN_FILENO, ioctl, winsize, STDOUT_FILENO, TIOCGWINSZ}, std::io::stdin, std::os::fd::AsRawFd, std::io::Read
};
use crate::parse::get_vars;
fn help()
{
    println!(
             "Usage: kalc [FLAGS] function_1 function_2 function_3...\n\
FLAGS: --help (this message)\n\
--tau fractions are shown in tau instead of pi\n\
--deg compute in degrees, gets rid of complex support for non hyperbolic trig functions\n\
--2d [num] number of points to graph in 2D\n\
--3d [num] number of points to graph in 3D\n\
--xr [min] [max] x range for graphing\n\
--yr [min] [max] y range for graphing\n\
--zr [min] [max] z range for graphing\n\
--point [char] point style for graphing\n\
--sci toggles scientific notation\n\
--base [num] sets the number base (2,8,16)\n\
--prompt toggles the prompt\n\
--color toggles color\n\
--vars toggles default variables\n\
--default ignores config file\n\
--debug displays computation time in nanoseconds\n\n\
- Type \"exit\" to exit the program\n\
- Type \"clear\" to clear the screen\n\
- Type \"help\" to see this message\n\
- Type \"history\" to see the history of calculations\n\
- Type \"deg\" to switch to degrees mode\n\
- Type \"rad\" to switch to radians mode\n\
- Type \"tau\" to show fractions in tau\n\
- Type \"pi\" to show fractions in pi\n\
- Type \"prompt\" to toggle the prompt\n\
- Type \"color\" to toggle color\n\
- Type \"2d=[num]\" to set the number of points in 2D graphs\n\
- Type \"3d=[num]\" to set the number of points in 3D graphs\n\
- Type \"xr=[min],[max]\" to set the x range for graphing\n\
- Type \"yr=[min],[max]\" to set the y range for graphing\n\
- Type \"zr=[min],[max]\" to set the z range for graphing\n\
- Type \"point=[char]\" to set the point style for graphing\n\
- Type \"sci\" to toggle scientific notation\n\
- Type \"base=[num]\" to set the number base (2,8,16)\n\
- Type \"_\" to use the previous answer\n\
- Type \"a=[num]\" to define a variable\n\
- Type \"f(x)=...\" to define a function\n\
- Type \"f(x,y)=...\" to define a 2 variable function\n\
- Type \"f(x,y,z...)=...\" to define a multi variable function\n\
- Type \"debug\" toggles displaying computation time in nanoseconds\n\n\
Trigonometric functions:\n\
- sin, cos, tan, asin, acos, atan\n\
- csc, sec, cot, acsc, asec, acot\n\
- sinh, cosh, tanh, asinh, acosh, atanh\n\
- csch, sech, coth, acsch, asech, acoth\n\n\
Other functions:\n\
- sqrt, cbrt, square, cube\n\
- ln, log(base,num), root(base,exp)\n\
- abs, sgn, arg\n\
- ceil, floor, round, int, frac\n\
- fact, subfact\n\
- sinc, cis, exp\n\
- deg(to_degrees), rad(to_radians)\n\
- re(real part), im(imaginary part)\n\n\
Constants:\n\
- g: gravity\n\
- c: speed of light\n\
- h: planck's constant\n\
- e: euler's number\n\
- pi: pi\n\
- tau: tau (2pi)\n\
- phi: golden ratio\n\
- G: gravitational constant\n\
- ec: elementary charge\n\
- mp: proton mass\n\
- mn: neutron mass\n\
- me: electron mass\n\
- ev: electron volt\n\
- kc: coulomb's constant"
    );
}
fn write(input:&str, file:&mut File, lines:&Vec<String>)
{
    if lines.is_empty() || lines[lines.len() - 1] != *input
    {
        file.write_all(input.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}
fn main()
{
    let mut graph_options = ([[-10.0, 10.0]; 3], 40000.0, 400.0, '.'); //[xr,yr,zr], 2d, 3d, point style
    let mut plot = Figure::new();
    let mut watch = None;
    let mut print_options = (false, false, 10); //[sci, deg, #base]
    let mut allow_vars = true;
    let mut debug = false;
    let mut tau = false;
    let mut prompt = true;
    let mut color = true;
    plot.set_enhanced_text(false);
    #[cfg(unix)]
    let file_path = &(var("HOME").unwrap() + "/.config/kalc.config");
    #[cfg(not(unix))]
    let file_path = &format!("C:\\Users\\{}\\AppData\\Roaming\\kalc.config", var("USERNAME").unwrap());
    if File::open(file_path).is_ok()
    {
        let file = File::open(file_path).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines()
        {
            let line = line.unwrap();
            let mut args = line.split('=');
            match args.next().unwrap()
            {
                "2d" => graph_options.1 = args.next().unwrap().parse::<f64>().unwrap_or(40000.0),
                "3d" => graph_options.2 = args.next().unwrap().parse::<f64>().unwrap_or(400.0),
                "xr" =>
                {
                    let mut xr = args.next().unwrap().split(',');
                    graph_options.0[0][0] = xr.next().unwrap().parse::<f64>().unwrap_or(-10.0);
                    graph_options.0[0][1] = xr.next().unwrap().parse::<f64>().unwrap_or(10.0);
                }
                "yr" =>
                {
                    let mut yr = args.next().unwrap().split(',');
                    graph_options.0[1][0] = yr.next().unwrap().parse::<f64>().unwrap_or(-10.0);
                    graph_options.0[1][1] = yr.next().unwrap().parse::<f64>().unwrap_or(10.0);
                }
                "zr" =>
                {
                    let mut zr = args.next().unwrap().split(',');
                    graph_options.0[2][0] = zr.next().unwrap().parse::<f64>().unwrap_or(-10.0);
                    graph_options.0[2][1] = zr.next().unwrap().parse::<f64>().unwrap_or(10.0);
                }
                "prompt" => prompt = args.next().unwrap().parse::<bool>().unwrap_or(true),
                "color" => color = args.next().unwrap().parse::<bool>().unwrap_or(true),
                "point" => graph_options.3 = args.next().unwrap().chars().next().unwrap_or('.'),
                "sci" => print_options.0 = args.next().unwrap().parse::<bool>().unwrap_or(false),
                "base" => print_options.2 = args.next().unwrap().parse::<usize>().unwrap_or(10),
                "debug" => debug = args.next().unwrap().parse::<bool>().unwrap_or(false),
                "deg" => print_options.1 = args.next().unwrap().parse::<bool>().unwrap_or(false),
                "tau" => tau = args.next().unwrap().parse::<bool>().unwrap_or(false),
                "vars" => allow_vars = args.next().unwrap().parse::<bool>().unwrap_or(true),
                _ =>
                {}
            }
        }
    }
    let mut args = args().collect::<Vec<String>>();
    args.remove(0);
    while !args.is_empty()
    {
        match args[0].as_str()
        {
            "--debug" => debug = !debug,
            "--tau" => tau = !tau,
            "--deg" => print_options.1 = !print_options.1,
            "--prompt" => prompt = !prompt,
            "--color" => color = !color,
            "--2d" =>
            {
                if args.len() > 1
                {
                    graph_options.1 = args[1].parse::<f64>().unwrap_or(40000.0);
                    args.remove(0);
                }
            }
            "--3d" =>
            {
                if args.len() > 1
                {
                    graph_options.2 = args[1].parse::<f64>().unwrap_or(400.0);
                    args.remove(0);
                }
            }
            "--yr" =>
            {
                if args.len() > 2
                {
                    graph_options.0[1][0] = args[1].parse::<f64>().unwrap_or(-10.0);
                    graph_options.0[1][1] = args[2].parse::<f64>().unwrap_or(10.0);
                    args.remove(0);
                    args.remove(0);
                }
            }
            "--xr" =>
            {
                if args.len() > 2
                {
                    graph_options.0[0][0] = args[1].parse::<f64>().unwrap_or(-10.0);
                    graph_options.0[0][1] = args[2].parse::<f64>().unwrap_or(10.0);
                    args.remove(0);
                    args.remove(0);
                }
            }
            "--zr" =>
            {
                if args.len() > 2
                {
                    graph_options.0[2][0] = args[1].parse::<f64>().unwrap_or(-10.0);
                    graph_options.0[2][1] = args[2].parse::<f64>().unwrap_or(10.0);
                    args.remove(0);
                    args.remove(0);
                }
            }
            "--base" =>
            {
                if args.len() > 1
                {
                    print_options.2 = args[1].parse::<usize>().unwrap_or(10);
                    args.remove(0);
                }
            }
            "--sci" => print_options.0 = !print_options.0,
            "--point" =>
            {
                graph_options.3 = args[1].chars().next().unwrap_or('.');
                args.remove(0);
            }
            "--help" =>
            {
                help();
                return;
            }
            "--version" =>
            {
                println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
                return;
            }
            "--vars" => allow_vars = !allow_vars,
            "--default" =>
            {
                print_options = (false, false, 10);
                allow_vars = true;
                debug = false;
                tau = false;
                prompt = true;
                color = true;
            }
            _ => break,
        }
        args.remove(0);
    }
    let mut vars:Vec<String> = get_vars(allow_vars);
    let mut input = String::new();
    #[cfg(unix)]
    if !unsafe { isatty(STDIN_FILENO) != 0 }
    {
        for line in stdin().lock().lines()
        {
            args.push(line.unwrap());
        }
    }
    #[cfg(unix)]
    let file_path = &(var("HOME").unwrap() + "/.config/kalc.history");
    #[cfg(not(unix))]
    let file_path = &format!("C:\\Users\\{}\\AppData\\Roaming\\kalc.history", var("USERNAME").unwrap());
    if File::open(file_path).is_err()
    {
        File::create(file_path).unwrap();
    }
    let mut file = OpenOptions::new().append(true).open(file_path).unwrap();
    let mut lines:Vec<String>;
    #[cfg(unix)]
    let mut width;
    let mut last = String::new();
    let (mut c, mut i, mut max, mut cursor, mut frac, mut len, mut l, mut r, mut funcl, mut funcm, mut funcr, mut split_str, mut split);
    let mut exit = false;
    loop
    {
        if exit
        {
            break;
        }
        #[cfg(unix)]
        {
            width = get_terminal_width();
        }
        input.clear();
        lines = BufReader::new(File::open(file_path).unwrap()).lines().map(|l| l.unwrap()).collect();
        i = lines.len() as i32;
        max = i;
        cursor = 0;
        frac = false;
        if !args.is_empty()
        {
            if debug
            {
                watch = Some(std::time::Instant::now());
            }
            input = args.first().unwrap().replace('z', "(x+y*i)").replace(' ', "").replace('_', &format!("({})", last));
            args.remove(0);
            print_answer(&input,
                         match get_func(&input_var(&input, &vars))
                         {
                             Ok(f) => f,
                             Err(()) =>
                             {
                                 println!("Invalid function.");
                                 return;
                             }
                         },
                         print_options,
                         color);
            if let Some(time) = watch
            {
                print!(" {}", time.elapsed().as_nanos());
            }
            println!();
            last = input.clone();
            if args.is_empty()
            {
                exit = true;
            }
        }
        else
        {
            last = lines.last().unwrap_or(&String::new()).clone();
            if prompt
            {
                print!("{}> \x1b[0m", if color { "\x1b[94m" } else { "" });
                stdout().flush().unwrap();
            }
            'outer: loop
            {
                c = read_single_char();
                if debug
                {
                    watch = Some(std::time::Instant::now());
                }
                match c
                {
                    '\n' =>
                    {
                        if frac
                        {
                            println!();
                        }
                        if !((input.contains('x') && !input.contains("exp")) || input.contains('y') || input.contains('z') || input.contains('='))
                        {
                            println!();
                        }
                        println!();
                        break;
                    }
                    '\x08' =>
                    {
                        if cursor == 0
                        {
                            if input.is_empty()
                            {
                                print!("\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A{}",
                                       if prompt
                                       {
                                           if color
                                           {
                                               "\x1b[94m> \x1b[0m"
                                           }
                                           else
                                           {
                                               "> "
                                           }
                                       }
                                       else
                                       {
                                           ""
                                       });
                                stdout().flush().unwrap();
                            }
                            continue;
                        }
                        cursor -= 1;
                        input.remove(cursor);
                        print!("\x1B[2K\x1B[1G{}", input);
                        #[cfg(unix)]
                        if width < (input.len() + 1) as u16
                        {
                            print!("\x1b[A");
                        }
                        frac = if input.is_empty()
                        {
                            false
                        }
                        else
                        {
                            print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), tau, print_options, prompt, color)
                        };
                        len = input.len();
                        if let Some(time) = watch
                        {
                            let time = time.elapsed().as_nanos();
                            len += time.to_string().len() + 1;
                            print!(" {}", time);
                        }
                        for _ in 0..(len - cursor)
                        {
                            print!("\x08");
                        }
                        if cursor == 0 && input.is_empty()
                        {
                            print!("\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A{}",
                                   if prompt
                                   {
                                       if color
                                       {
                                           "\x1b[94m> \x1b[0m"
                                       }
                                       else
                                       {
                                           "> "
                                       }
                                   }
                                   else
                                   {
                                       ""
                                   });
                        }
                    }
                    '\x1D' =>
                    {
                        i -= if i > 0 { 1 } else { 0 };
                        input = lines[i as usize].clone();
                        cursor = input.len();
                        #[cfg(unix)]
                        if width < (input.len() + 1) as u16
                        {
                            print!("\x1b[A");
                        }
                        frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), tau, print_options, prompt, color);
                        print!("\x1B[2K\x1B[1G{}{}\x1b[0m",
                               if prompt
                               {
                                   if color
                                   {
                                       "\x1b[94m> \x1b[96m"
                                   }
                                   else
                                   {
                                       "> "
                                   }
                               }
                               else if color
                               {
                                   "\x1b[96m"
                               }
                               else
                               {
                                   ""
                               },
                               input);
                    }
                    '\x1E' =>
                    {
                        i += 1;
                        if i >= max
                        {
                            input.clear();
                            print!("\n\x1B[2K\x1B[1G\n\x1B[2K\x1B[1G\x1b[A\x1b[A\x1B[2K\x1B[1G{}",
                                   if prompt
                                   {
                                       if color
                                       {
                                           "\x1b[94m> \x1b[0m"
                                       }
                                       else
                                       {
                                           "> "
                                       }
                                   }
                                   else
                                   {
                                       ""
                                   });
                            stdout().flush().unwrap();
                            i = max;
                            cursor = 0;
                            continue 'outer;
                        }
                        input = lines[i as usize].clone();
                        cursor = input.len();
                        #[cfg(unix)]
                        if width < (input.len() + 1) as u16
                        {
                            print!("\x1b[A");
                        }
                        frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), tau, print_options, prompt, color);
                        print!("\x1B[2K\x1B[1G{}{}\x1b[0m",
                               if prompt
                               {
                                   if color
                                   {
                                       "\x1b[94m> \x1b[96m"
                                   }
                                   else
                                   {
                                       "> "
                                   }
                               }
                               else if color
                               {
                                   "\x1b[96m"
                               }
                               else
                               {
                                   ""
                               },
                               input);
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
                        if c == 'π'
                        {
                            input.insert(cursor, 'p');
                            cursor += 1;
                            input.insert(cursor, 'i');
                            cursor += 1;
                        }
                        else if c == 'τ'
                        {
                            input.insert(cursor, 't');
                            cursor += 1;
                            input.insert(cursor, 'a');
                            cursor += 1;
                            input.insert(cursor, 'u');
                            cursor += 1;
                        }
                        else
                        {
                            input.insert(cursor, c);
                            cursor += 1;
                        }
                        #[cfg(unix)]
                        if width < (input.len() + 1) as u16
                        {
                            print!("\x1b[A");
                        }
                        frac = print_concurrent(&input, &input_var(&input.replace('_', &format!("({})", last)), &vars), tau, print_options, prompt, color);
                        len = input.len();
                        if let Some(time) = watch
                        {
                            let time = time.elapsed().as_nanos();
                            len += time.to_string().len() + 1;
                            print!(" {}", time);
                        }
                        for _ in 0..(len - cursor)
                        {
                            print!("\x08");
                        }
                    }
                }
                stdout().flush().unwrap();
            }
            if input.is_empty() || input.chars().filter(|c| *c == '=').count() > 1
            {
                continue;
            }
            match input.as_str()
            {
                "color" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    color = !color;
                    continue;
                }
                "prompt" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    prompt = !prompt;
                    continue;
                }
                "deg" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.1 = true;
                    continue;
                }
                "rad" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.1 = false;
                    continue;
                }
                "tau" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    tau = true;
                    println!();
                    write(&input, &mut file, &lines);
                    continue;
                }
                "pi" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    tau = false;
                    println!();
                    write(&input, &mut file, &lines);
                    continue;
                }
                "sci" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print_options.0 = !print_options.0;
                    continue;
                }
                "clear" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    print!("\x1B[2J\x1B[1;1H");
                    stdout().flush().unwrap();
                    continue;
                }
                "debug" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    debug = !debug;
                    watch = None;
                    continue;
                }
                "help" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    help();
                    continue;
                }
                "history" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    for l in lines
                    {
                        println!("{}", l);
                    }
                    continue;
                }
                "exit" =>
                {
                    print!("\x1b[A\x1B[2K\x1B[1G");
                    stdout().flush().unwrap();
                    break;
                }
                _ =>
                {}
            }
            if input.len() > 6 && &input[0..6] == "point="
            {
                graph_options.3 = input[6..].chars().next().unwrap();
                continue;
            }
            if input.len() > 5 && &input[0..5] == "base="
            {
                print_options.2 = input[5..].parse::<usize>().unwrap();
                continue;
            }
            write(&input, &mut file, &lines);
        }
        if input.contains("==")
        {
            split_str = input.split("==");
            l = split_str.next().unwrap();
            r = split_str.next().unwrap();
            equal(graph_options, &input, l, r);
            continue;
        }
        else if input.contains('=')
        {
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
            split = input.split('=');
            l = split.next().unwrap();
            r = split.next().unwrap();
            match l
            {
                "xr" =>
                {
                    graph_options.0[0][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                    graph_options.0[0][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                    continue;
                }
                "yr" =>
                {
                    graph_options.0[1][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                    graph_options.0[1][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                    continue;
                }
                "zr" =>
                {
                    graph_options.0[2][0] = r.split(',').next().unwrap().parse::<f64>().unwrap();
                    graph_options.0[2][1] = r.split(',').last().unwrap().parse::<f64>().unwrap();
                    continue;
                }
                "2d" =>
                {
                    graph_options.1 = r.parse::<f64>().unwrap();
                    continue;
                }
                "3d" =>
                {
                    graph_options.2 = r.parse::<f64>().unwrap();
                    continue;
                }
                _ => (),
            }
            for i in 0..vars.len()
            {
                if vars[i].split('=').next().unwrap() == l
                {
                    vars.remove(i);
                    break;
                }
            }
            vars.push(format!("{}={}", l, input_var(r, &vars)));
            continue;
        }
        else if (input.replace("exp", "").contains('x') && vars.iter().all(|i| i.split('=').next().unwrap() != "x"))
                  || (input.contains('z') && vars.iter().all(|i| i.split('=').next().unwrap() != "z"))
        {
            input = input.replace('z', "(x+y*i)");
            print!("\x1b[2K\x1b[1G");
            stdout().flush().unwrap();
            split = input.split('#');
            let l = input_var(split.next().unwrap_or("0"), &vars);
            let m = input_var(split.next().unwrap_or("0"), &vars);
            let r = input_var(split.next().unwrap_or("0"), &vars);
            funcl = match get_func(&l)
            {
                Ok(f) => f,
                _ => continue,
            };
            funcm = match get_func(&m)
            {
                Ok(f) => f,
                _ => continue,
            };
            funcr = match get_func(&r)
            {
                Ok(f) => f,
                _ => continue,
            };
            graph([&l, &m, &r], [&funcl, &funcm, &funcr], false, &mut plot, graph_options, print_options.1);
            if let Some(time) = watch
            {
                println!("{}", time.elapsed().as_nanos());
            }
            continue;
        }
    }
}
#[cfg(unix)]
fn get_terminal_width() -> u16
{
    unsafe {
        let mut size:winsize = std::mem::zeroed();
        if ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut size) == 0
        {
            size.ws_col
        }
        else
        {
            80
        }
    }
}
#[cfg(unix)]
fn read_single_char() -> char
{
    let stdin_fd = stdin().as_raw_fd();
    let orig_termios = unsafe {
        let mut termios = std::mem::zeroed();
        tcgetattr(stdin_fd, &mut termios);
        termios
    };
    let mut new_termios = orig_termios;
    new_termios.c_lflag &= !(ICANON | ECHO);
    new_termios.c_cc[VMIN] = 1;
    new_termios.c_cc[VTIME] = 0;
    unsafe {
        tcsetattr(stdin_fd, TCSANOW, &new_termios);
    }
    let mut input = [0u8; 1];
    stdin().read_exact(&mut input).unwrap();
    unsafe {
        tcsetattr(stdin_fd, TCSANOW, &orig_termios);
    }
    match input[0]
    {
        27 =>
        {
            let mut seq = [0u8; 1];
            stdin().read_exact(&mut seq).unwrap();
            if seq[0] != 91
            {
                return seq[0] as char;
            }
            stdin().read_exact(&mut seq).unwrap();
            match seq[0]
            {
                65 => '\x1D', // Up arrow key
                66 => '\x1E', // Down arrow key
                67 => '\x1C', // Right arrow key
                68 => '\x1B', // Left arrow key
                _ => read_single_char(),
            }
        }
        207 =>
        {
            let mut seq = [0u8; 1];
            stdin().read_exact(&mut seq).unwrap();
            match seq[0]
            {
                128 => 'π',
                132 => 'τ',
                _ => read_single_char(),
            }
        }
        127 => '\x08',
        b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'+' | b'-' | b'*' | b'/' | b'^' | b'(' | b')' | b'.' | b'=' | b',' | b'#' | b'|' | b'!' | b'%' | b'_' | b'\n' => input[0] as char,
        _ => read_single_char(),
    }
}
#[cfg(not(unix))]
fn read_single_char() -> char
{
    let term = Term::stdout();
    let key = term.read_key().unwrap();
    match key
    {
        Key::Char(c) =>
        {
            if c.is_ascii_alphanumeric()
               || c == '+'
               || c == '-'
               || c == '*'
               || c == '/'
               || c == '^'
               || c == '('
               || c == ')'
               || c == '.'
               || c == '='
               || c == ','
               || c == 'π'
               || c == 'τ'
               || c == '#'
               || c == '|'
               || c == '!'
               || c == '%'
               || c == '_'
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