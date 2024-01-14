use crate::{
    complex::{
        NumStr,
        NumStr::{Matrix, Num, Str, Vector},
    },
    math::do_math,
    misc::prompt,
    Colors, Options,
};
use gnuplot::{AxesCommon, Caption, Color, Figure, Fix, PointSymbol};
use rug::Complex;
use std::{
    io::{stdout, Write},
    thread,
    thread::JoinHandle,
    time::Instant,
};
pub fn graph(
    input: Vec<String>,
    unmod: Vec<String>,
    func: Vec<Vec<NumStr>>,
    mut options: Options,
    watch: Option<Instant>,
    colors: Colors,
) -> JoinHandle<()>
{
    thread::spawn(move || {
        if input.iter().all(|i| i.is_empty())
        {
            return;
        }
        options.prec = options.graph_prec;
        let mut fg = Figure::new();
        fg.set_enhanced_text(false);
        let mut re_cap: [String; 6] = Default::default();
        let mut im_cap: [String; 6] = Default::default();
        let mut points2d: [[[Vec<f64>; 2]; 2]; 6] = Default::default();
        let mut points3d: [[[Vec<f64>; 3]; 2]; 6] = Default::default();
        let mut d2_or_d3 = (false, false);
        let mut lines = false;
        let mut handles = Vec::new();
        for func in func
        {
            handles.push(get_data(options, colors.clone(), func));
        }
        let mut i = 0;
        #[allow(clippy::explicit_counter_loop)]
        for handle in handles
        {
            let re_or_im;
            let failed;
            let dimen;
            let line;
            (dimen, re_or_im, line, failed, points2d[i], points3d[i]) = handle.join().unwrap();
            if failed
            {
                return;
            }
            if re_or_im.0
            {
                re_cap[i] = unmod[i].clone() + if re_or_im.1 { ":re" } else { "" }
            }
            if re_or_im.1
            {
                im_cap[i] = unmod[i].clone() + ":im"
            }
            if dimen.0
            {
                d2_or_d3.0 = true;
            }
            if dimen.1
            {
                d2_or_d3.1 = true;
            }
            if line
            {
                lines = true
            }
            i += 1;
        }
        if d2_or_d3.0 == d2_or_d3.1
        {
            print!(
                "\x1b[G\x1b[Kcant graph 2d and 3d\n\x1b[G{}",
                prompt(options, &colors)
            );
            stdout().flush().unwrap();
            return;
        }
        if d2_or_d3.0
        {
            if lines
            {
                if Options::default().yr == options.yr
                {
                    options.xr = (
                        points2d.iter().fold(f64::MAX, |min, x| {
                            min.min(
                                x[0][0]
                                    .iter()
                                    .chain(&x[1][0])
                                    .fold(f64::MAX, |min, x| min.min(*x)),
                            )
                        }),
                        points2d.iter().fold(f64::MIN, |max, x| {
                            max.max(
                                x[0][0]
                                    .iter()
                                    .chain(&x[1][0])
                                    .fold(f64::MIN, |max, x| max.max(*x)),
                            )
                        }),
                    )
                }
                if Options::default().yr == options.yr
                {
                    options.yr = (
                        points2d.iter().fold(f64::MAX, |min, x| {
                            min.min(
                                x[0][1]
                                    .iter()
                                    .chain(&x[1][1])
                                    .fold(f64::MAX, |min, x| min.min(*x)),
                            )
                        }),
                        points2d.iter().fold(f64::MIN, |max, x| {
                            max.max(
                                x[0][1]
                                    .iter()
                                    .chain(&x[1][1])
                                    .fold(f64::MIN, |max, x| max.max(*x)),
                            )
                        }),
                    )
                }
            }
            let (xticks, yticks) = if options.ticks == 0.0
            {
                (Some((Fix(1.0), 1)), Some((Fix(1.0), 1)))
            }
            else
            {
                (
                    Some((Fix((options.xr.1 - options.xr.0) / options.ticks), 1)),
                    Some((Fix((options.yr.1 - options.yr.0) / options.ticks), 1)),
                )
            };
            if options.lines || lines
            {
                fg.axes2d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_x_label("x", &[])
                    .set_y_label("y", &[])
                    .lines([0], [0], &[Caption(&re_cap[0]), Color(&colors.re1col)])
                    .lines([0], [0], &[Caption(&im_cap[0]), Color(&colors.im1col)])
                    .lines([0], [0], &[Caption(&re_cap[1]), Color(&colors.re2col)])
                    .lines([0], [0], &[Caption(&im_cap[1]), Color(&colors.im2col)])
                    .lines([0], [0], &[Caption(&re_cap[2]), Color(&colors.re3col)])
                    .lines([0], [0], &[Caption(&im_cap[2]), Color(&colors.im3col)])
                    .lines([0], [0], &[Caption(&re_cap[3]), Color(&colors.re4col)])
                    .lines([0], [0], &[Caption(&im_cap[3]), Color(&colors.im4col)])
                    .lines([0], [0], &[Caption(&re_cap[4]), Color(&colors.re5col)])
                    .lines([0], [0], &[Caption(&im_cap[4]), Color(&colors.im5col)])
                    .lines([0], [0], &[Caption(&re_cap[5]), Color(&colors.re6col)])
                    .lines([0], [0], &[Caption(&im_cap[5]), Color(&colors.im6col)])
                    .lines(
                        &points2d[0][0][0],
                        &points2d[0][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re1col)],
                    )
                    .lines(
                        if points2d[0][1][0].is_empty()
                        {
                            &points2d[0][0][0]
                        }
                        else
                        {
                            &points2d[0][1][0]
                        },
                        &points2d[0][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im1col)],
                    )
                    .lines(
                        &points2d[1][0][0],
                        &points2d[1][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re2col)],
                    )
                    .lines(
                        if points2d[1][1][0].is_empty()
                        {
                            &points2d[2][0][0]
                        }
                        else
                        {
                            &points2d[1][1][0]
                        },
                        &points2d[1][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im2col)],
                    )
                    .lines(
                        &points2d[2][0][0],
                        &points2d[2][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re3col)],
                    )
                    .lines(
                        if points2d[2][1][0].is_empty()
                        {
                            &points2d[2][0][0]
                        }
                        else
                        {
                            &points2d[2][1][0]
                        },
                        &points2d[2][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im3col)],
                    )
                    .lines(
                        &points2d[3][0][0],
                        &points2d[3][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re4col)],
                    )
                    .lines(
                        if points2d[3][1][0].is_empty()
                        {
                            &points2d[3][0][0]
                        }
                        else
                        {
                            &points2d[3][1][0]
                        },
                        &points2d[3][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im4col)],
                    )
                    .lines(
                        &points2d[4][0][0],
                        &points2d[4][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re5col)],
                    )
                    .lines(
                        if points2d[4][1][0].is_empty()
                        {
                            &points2d[4][0][0]
                        }
                        else
                        {
                            &points2d[4][1][0]
                        },
                        &points2d[4][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im5col)],
                    )
                    .lines(
                        &points2d[5][0][0],
                        &points2d[5][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re6col)],
                    )
                    .lines(
                        if points2d[5][1][0].is_empty()
                        {
                            &points2d[5][0][0]
                        }
                        else
                        {
                            &points2d[5][1][0]
                        },
                        &points2d[5][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im6col)],
                    );
            }
            else
            {
                fg.axes2d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_x_label("x", &[])
                    .set_y_label("y", &[])
                    .lines([0], [0], &[Caption(&re_cap[0]), Color(&colors.re1col)])
                    .lines([0], [0], &[Caption(&im_cap[0]), Color(&colors.im1col)])
                    .lines([0], [0], &[Caption(&re_cap[1]), Color(&colors.re2col)])
                    .lines([0], [0], &[Caption(&im_cap[1]), Color(&colors.im2col)])
                    .lines([0], [0], &[Caption(&re_cap[2]), Color(&colors.re3col)])
                    .lines([0], [0], &[Caption(&im_cap[2]), Color(&colors.im3col)])
                    .lines([0], [0], &[Caption(&re_cap[3]), Color(&colors.re4col)])
                    .lines([0], [0], &[Caption(&im_cap[3]), Color(&colors.im4col)])
                    .lines([0], [0], &[Caption(&re_cap[4]), Color(&colors.re5col)])
                    .lines([0], [0], &[Caption(&im_cap[4]), Color(&colors.im5col)])
                    .lines([0], [0], &[Caption(&re_cap[5]), Color(&colors.re6col)])
                    .lines([0], [0], &[Caption(&im_cap[5]), Color(&colors.im6col)])
                    .points(
                        &points2d[0][0][0],
                        &points2d[0][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re1col)],
                    )
                    .points(
                        if points2d[0][1][0].is_empty()
                        {
                            &points2d[0][0][0]
                        }
                        else
                        {
                            &points2d[0][1][0]
                        },
                        &points2d[0][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im1col)],
                    )
                    .points(
                        &points2d[1][0][0],
                        &points2d[1][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re2col)],
                    )
                    .points(
                        if points2d[1][1][0].is_empty()
                        {
                            &points2d[2][0][0]
                        }
                        else
                        {
                            &points2d[1][1][0]
                        },
                        &points2d[1][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im2col)],
                    )
                    .points(
                        &points2d[2][0][0],
                        &points2d[2][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re3col)],
                    )
                    .points(
                        if points2d[2][1][0].is_empty()
                        {
                            &points2d[2][0][0]
                        }
                        else
                        {
                            &points2d[2][1][0]
                        },
                        &points2d[2][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im3col)],
                    )
                    .points(
                        &points2d[3][0][0],
                        &points2d[3][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re4col)],
                    )
                    .points(
                        if points2d[3][1][0].is_empty()
                        {
                            &points2d[3][0][0]
                        }
                        else
                        {
                            &points2d[3][1][0]
                        },
                        &points2d[3][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im4col)],
                    )
                    .points(
                        &points2d[4][0][0],
                        &points2d[4][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re5col)],
                    )
                    .points(
                        if points2d[4][1][0].is_empty()
                        {
                            &points2d[4][0][0]
                        }
                        else
                        {
                            &points2d[4][1][0]
                        },
                        &points2d[4][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im5col)],
                    )
                    .points(
                        &points2d[5][0][0],
                        &points2d[5][0][1],
                        &[PointSymbol(options.point_style), Color(&colors.re6col)],
                    )
                    .points(
                        if points2d[5][1][0].is_empty()
                        {
                            &points2d[5][0][0]
                        }
                        else
                        {
                            &points2d[5][1][0]
                        },
                        &points2d[5][1][1],
                        &[PointSymbol(options.point_style), Color(&colors.im6col)],
                    );
            }
        }
        if d2_or_d3.1
        {
            if lines
            {
                if Options::default().yr == options.yr
                {
                    options.xr = (
                        points3d.iter().fold(f64::MAX, |min, x| {
                            min.min(
                                x[0][0]
                                    .iter()
                                    .chain(&x[1][0])
                                    .fold(f64::MAX, |min, x| min.min(*x)),
                            )
                        }),
                        points3d.iter().fold(f64::MIN, |max, x| {
                            max.max(
                                x[0][0]
                                    .iter()
                                    .chain(&x[1][0])
                                    .fold(f64::MIN, |max, x| max.max(*x)),
                            )
                        }),
                    )
                }
                if Options::default().yr == options.yr
                {
                    options.yr = (
                        points3d.iter().fold(f64::MAX, |min, x| {
                            min.min(
                                x[0][1]
                                    .iter()
                                    .chain(&x[1][1])
                                    .fold(f64::MAX, |min, x| min.min(*x)),
                            )
                        }),
                        points3d.iter().fold(f64::MIN, |max, x| {
                            max.max(
                                x[0][1]
                                    .iter()
                                    .chain(&x[1][1])
                                    .fold(f64::MIN, |max, x| max.max(*x)),
                            )
                        }),
                    )
                }
                if Options::default().zr == options.zr
                {
                    options.zr = (
                        points3d.iter().fold(f64::MAX, |min, x| {
                            min.min(
                                x[0][2]
                                    .iter()
                                    .chain(&x[1][2])
                                    .fold(f64::MAX, |min, x| min.min(*x)),
                            )
                        }),
                        points3d.iter().fold(f64::MIN, |max, x| {
                            max.max(
                                x[0][2]
                                    .iter()
                                    .chain(&x[1][2])
                                    .fold(f64::MIN, |max, x| max.max(*x)),
                            )
                        }),
                    )
                }
            }
            let (xticks, yticks, zticks) = if options.ticks == 0.0
            {
                (
                    Some((Fix(1.0), 1)),
                    Some((Fix(1.0), 1)),
                    Some((Fix(1.0), 1)),
                )
            }
            else
            {
                (
                    Some((Fix((options.xr.1 - options.xr.0) / options.ticks), 1)),
                    Some((Fix((options.yr.1 - options.yr.0) / options.ticks), 1)),
                    Some((Fix((options.zr.1 - options.zr.0) / options.ticks), 1)),
                )
            };
            if options.lines || lines
            {
                fg.axes3d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_z_ticks(zticks, &[], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_z_range(Fix(options.zr.0), Fix(options.zr.1))
                    .set_x_label("x", &[])
                    .set_y_label("y", &[])
                    .set_z_label("z", &[])
                    .lines([0], [0], [0], &[Caption(&re_cap[0]), Color(&colors.re1col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[0]), Color(&colors.im1col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[1]), Color(&colors.re2col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[1]), Color(&colors.im2col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[2]), Color(&colors.re3col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[2]), Color(&colors.im3col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[3]), Color(&colors.re4col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[3]), Color(&colors.im4col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[4]), Color(&colors.re5col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[4]), Color(&colors.im5col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[5]), Color(&colors.re6col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[5]), Color(&colors.im6col)])
                    .lines(
                        &points3d[0][0][0],
                        &points3d[0][0][1],
                        &points3d[0][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re1col)],
                    )
                    .lines(
                        if points3d[0][1][0].is_empty()
                        {
                            &points3d[0][0][0]
                        }
                        else
                        {
                            &points3d[0][1][0]
                        },
                        if points3d[0][1][1].is_empty()
                        {
                            &points3d[0][0][1]
                        }
                        else
                        {
                            &points3d[0][1][1]
                        },
                        &points3d[0][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im1col)],
                    )
                    .lines(
                        &points3d[1][0][0],
                        &points3d[1][0][1],
                        &points3d[1][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re2col)],
                    )
                    .lines(
                        if points3d[1][1][0].is_empty()
                        {
                            &points3d[1][0][0]
                        }
                        else
                        {
                            &points3d[1][1][0]
                        },
                        if points3d[1][1][1].is_empty()
                        {
                            &points3d[1][0][1]
                        }
                        else
                        {
                            &points3d[1][1][1]
                        },
                        &points3d[1][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im2col)],
                    )
                    .lines(
                        &points3d[2][0][0],
                        &points3d[2][0][1],
                        &points3d[2][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re3col)],
                    )
                    .lines(
                        if points3d[2][1][0].is_empty()
                        {
                            &points3d[2][0][0]
                        }
                        else
                        {
                            &points3d[2][1][0]
                        },
                        if points3d[2][1][1].is_empty()
                        {
                            &points3d[2][0][1]
                        }
                        else
                        {
                            &points3d[2][1][1]
                        },
                        &points3d[2][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im3col)],
                    )
                    .lines(
                        &points3d[3][0][0],
                        &points3d[3][0][1],
                        &points3d[3][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re4col)],
                    )
                    .lines(
                        if points3d[3][1][0].is_empty()
                        {
                            &points3d[3][0][0]
                        }
                        else
                        {
                            &points3d[3][1][0]
                        },
                        if points3d[3][1][1].is_empty()
                        {
                            &points3d[3][0][1]
                        }
                        else
                        {
                            &points3d[3][1][1]
                        },
                        &points3d[3][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im4col)],
                    )
                    .lines(
                        &points3d[4][0][0],
                        &points3d[4][0][1],
                        &points3d[4][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re5col)],
                    )
                    .lines(
                        if points3d[4][1][0].is_empty()
                        {
                            &points3d[4][0][0]
                        }
                        else
                        {
                            &points3d[4][1][0]
                        },
                        if points3d[4][1][1].is_empty()
                        {
                            &points3d[4][0][1]
                        }
                        else
                        {
                            &points3d[4][1][1]
                        },
                        &points3d[4][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im5col)],
                    )
                    .lines(
                        &points3d[5][0][0],
                        &points3d[5][0][1],
                        &points3d[5][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re6col)],
                    )
                    .lines(
                        if points3d[5][1][0].is_empty()
                        {
                            &points3d[5][0][0]
                        }
                        else
                        {
                            &points3d[5][1][0]
                        },
                        if points3d[5][1][1].is_empty()
                        {
                            &points3d[5][0][1]
                        }
                        else
                        {
                            &points3d[5][1][1]
                        },
                        &points3d[5][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im6col)],
                    );
            }
            else
            {
                fg.axes3d()
                    .set_x_ticks(xticks, &[], &[])
                    .set_y_ticks(yticks, &[], &[])
                    .set_z_ticks(zticks, &[], &[])
                    .set_y_range(Fix(options.yr.0), Fix(options.yr.1))
                    .set_x_range(Fix(options.xr.0), Fix(options.xr.1))
                    .set_z_range(Fix(options.zr.0), Fix(options.zr.1))
                    .set_x_label("x", &[])
                    .set_y_label("y", &[])
                    .set_z_label("z", &[])
                    .lines([0], [0], [0], &[Caption(&re_cap[0]), Color(&colors.re1col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[0]), Color(&colors.im1col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[1]), Color(&colors.re2col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[1]), Color(&colors.im2col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[2]), Color(&colors.re3col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[2]), Color(&colors.im3col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[3]), Color(&colors.re4col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[3]), Color(&colors.im4col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[4]), Color(&colors.re5col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[4]), Color(&colors.im5col)])
                    .lines([0], [0], [0], &[Caption(&re_cap[5]), Color(&colors.re6col)])
                    .lines([0], [0], [0], &[Caption(&im_cap[5]), Color(&colors.im6col)])
                    .points(
                        &points3d[0][0][0],
                        &points3d[0][0][1],
                        &points3d[0][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re1col)],
                    )
                    .points(
                        if points3d[0][1][0].is_empty()
                        {
                            &points3d[0][0][0]
                        }
                        else
                        {
                            &points3d[0][1][0]
                        },
                        if points3d[0][1][1].is_empty()
                        {
                            &points3d[0][0][1]
                        }
                        else
                        {
                            &points3d[0][1][1]
                        },
                        &points3d[0][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im1col)],
                    )
                    .points(
                        &points3d[1][0][0],
                        &points3d[1][0][1],
                        &points3d[1][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re2col)],
                    )
                    .points(
                        if points3d[1][1][0].is_empty()
                        {
                            &points3d[1][0][0]
                        }
                        else
                        {
                            &points3d[1][1][0]
                        },
                        if points3d[1][1][1].is_empty()
                        {
                            &points3d[1][0][1]
                        }
                        else
                        {
                            &points3d[1][1][1]
                        },
                        &points3d[1][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im2col)],
                    )
                    .points(
                        &points3d[2][0][0],
                        &points3d[2][0][1],
                        &points3d[2][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re3col)],
                    )
                    .points(
                        if points3d[2][1][0].is_empty()
                        {
                            &points3d[2][0][0]
                        }
                        else
                        {
                            &points3d[2][1][0]
                        },
                        if points3d[2][1][1].is_empty()
                        {
                            &points3d[2][0][1]
                        }
                        else
                        {
                            &points3d[2][1][1]
                        },
                        &points3d[2][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im3col)],
                    )
                    .points(
                        &points3d[3][0][0],
                        &points3d[3][0][1],
                        &points3d[3][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re4col)],
                    )
                    .points(
                        if points3d[3][1][0].is_empty()
                        {
                            &points3d[3][0][0]
                        }
                        else
                        {
                            &points3d[3][1][0]
                        },
                        if points3d[3][1][1].is_empty()
                        {
                            &points3d[3][0][1]
                        }
                        else
                        {
                            &points3d[3][1][1]
                        },
                        &points3d[3][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im4col)],
                    )
                    .points(
                        &points3d[4][0][0],
                        &points3d[4][0][1],
                        &points3d[4][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re5col)],
                    )
                    .points(
                        if points3d[4][1][0].is_empty()
                        {
                            &points3d[4][0][0]
                        }
                        else
                        {
                            &points3d[4][1][0]
                        },
                        if points3d[4][1][1].is_empty()
                        {
                            &points3d[4][0][1]
                        }
                        else
                        {
                            &points3d[4][1][1]
                        },
                        &points3d[4][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im5col)],
                    )
                    .points(
                        &points3d[5][0][0],
                        &points3d[5][0][1],
                        &points3d[5][0][2],
                        &[PointSymbol(options.point_style), Color(&colors.re6col)],
                    )
                    .points(
                        if points3d[5][1][0].is_empty()
                        {
                            &points3d[5][0][0]
                        }
                        else
                        {
                            &points3d[5][1][0]
                        },
                        if points3d[5][1][1].is_empty()
                        {
                            &points3d[5][0][1]
                        }
                        else
                        {
                            &points3d[5][1][1]
                        },
                        &points3d[5][1][2],
                        &[PointSymbol(options.point_style), Color(&colors.im6col)],
                    );
            }
        }
        if let Some(time) = watch
        {
            print!("\x1b[G\x1b[K{}ms\n\x1b[G", time.elapsed().as_millis(),);
        }
        if fg.show().is_err()
        {
            print!("\x1b[G\x1b[Kno gnuplot\n\x1b[G{}", prompt(options, &colors));
        }
        stdout().flush().unwrap();
    })
}
#[allow(clippy::type_complexity)]
fn get_data(
    options: Options,
    colors: Colors,
    func: Vec<NumStr>,
) -> JoinHandle<(
    (bool, bool),
    (bool, bool),
    bool,
    bool,
    [[Vec<f64>; 2]; 2],
    [[Vec<f64>; 3]; 2],
)>
{
    thread::spawn(move || {
        let mut lines = false;
        let mut points2d: [[Vec<f64>; 2]; 2] = Default::default();
        let mut points3d: [[Vec<f64>; 3]; 2] = Default::default();
        let mut d2_or_d3: (bool, bool) = (false, false);
        let mut re_or_im = (false, false);
        let (has_x, has_y) = (
            func.iter().any(|i| i.str_is("x")),
            func.iter().any(|i| i.str_is("y")),
        );
        if !has_y && !has_x
        {
            match match do_math(func.clone(), options)
            {
                Ok(n) => n,
                _ =>
                {
                    fail(options, &colors);
                    return (
                        (false, false),
                        (false, false),
                        false,
                        true,
                        Default::default(),
                        Default::default(),
                    );
                }
            }
            {
                Num(n) =>
                {
                    d2_or_d3.0 = true;
                    (points2d, re_or_im) = get_list_2d(&[Num(n)], options);
                    if points2d[0][1].is_empty() && points2d[1][1].is_empty()
                    {
                        fail(options, &colors);
                        return (
                            (false, false),
                            (false, false),
                            false,
                            true,
                            Default::default(),
                            Default::default(),
                        );
                    }
                }
                Vector(v) =>
                {
                    lines = true;
                    match v.len()
                    {
                        3 =>
                        {
                            d2_or_d3.1 = true;
                            points3d = [
                                [
                                    vec![0.0, v[0].real().to_f64()],
                                    vec![0.0, v[1].real().to_f64()],
                                    vec![0.0, v[2].real().to_f64()],
                                ],
                                [
                                    vec![0.0, v[0].imag().to_f64()],
                                    vec![0.0, v[1].imag().to_f64()],
                                    vec![0.0, v[2].imag().to_f64()],
                                ],
                            ];
                            re_or_im = (
                                points3d[0].iter().flatten().any(|a| *a != 0.0),
                                points3d[1].iter().flatten().any(|a| *a != 0.0),
                            );
                        }
                        2 =>
                        {
                            d2_or_d3.0 = true;
                            points2d = [
                                [
                                    vec![0.0, v[0].real().to_f64()],
                                    vec![0.0, v[1].real().to_f64()],
                                ],
                                [
                                    vec![0.0, v[0].imag().to_f64()],
                                    vec![0.0, v[1].imag().to_f64()],
                                ],
                            ];
                            re_or_im = (
                                points2d[0].iter().flatten().any(|a| *a != 0.0),
                                points2d[1].iter().flatten().any(|a| *a != 0.0),
                            );
                        }
                        _ =>
                        {
                            d2_or_d3.0 = true;
                            let mut vec = Vec::with_capacity(v.len());
                            for i in 0..v.len()
                            {
                                vec.push(i as f64);
                            }
                            points2d = [
                                [
                                    vec,
                                    v.iter().map(|c| c.real().to_f64()).collect::<Vec<f64>>(),
                                ],
                                [
                                    Vec::new(),
                                    if v.iter().any(|c| !c.imag().is_zero())
                                    {
                                        v.iter().map(|c| c.imag().to_f64()).collect::<Vec<f64>>()
                                    }
                                    else
                                    {
                                        Vec::new()
                                    },
                                ],
                            ];
                            re_or_im = (
                                points2d[0].iter().flatten().any(|a| *a != 0.0),
                                points2d[1].iter().flatten().any(|a| *a != 0.0),
                            );
                        }
                    }
                }
                Matrix(m) =>
                {
                    lines = true;
                    match m[0].len()
                    {
                        3 =>
                        {
                            d2_or_d3.1 = true;
                            points3d = [
                                [
                                    m.iter().map(|c| c[0].real().to_f64()).collect::<Vec<f64>>(),
                                    m.iter().map(|c| c[1].real().to_f64()).collect::<Vec<f64>>(),
                                    m.iter().map(|c| c[2].real().to_f64()).collect::<Vec<f64>>(),
                                ],
                                [
                                    if m.iter().any(|c| !c[0].imag().is_zero())
                                    {
                                        m.iter().map(|c| c[0].imag().to_f64()).collect::<Vec<f64>>()
                                    }
                                    else
                                    {
                                        Vec::new()
                                    },
                                    if m.iter().any(|c| !c[1].imag().is_zero())
                                    {
                                        m.iter().map(|c| c[1].imag().to_f64()).collect::<Vec<f64>>()
                                    }
                                    else
                                    {
                                        Vec::new()
                                    },
                                    if m.iter().any(|c| !c[2].imag().is_zero())
                                    {
                                        m.iter().map(|c| c[2].imag().to_f64()).collect::<Vec<f64>>()
                                    }
                                    else
                                    {
                                        Vec::new()
                                    },
                                ],
                            ];
                            re_or_im = (
                                points3d[0].iter().flatten().any(|a| *a != 0.0),
                                points3d[1].iter().flatten().any(|a| *a != 0.0),
                            );
                        }
                        2 =>
                        {
                            d2_or_d3.0 = true;
                            points2d = [
                                [
                                    m.iter().map(|c| c[0].real().to_f64()).collect::<Vec<f64>>(),
                                    m.iter().map(|c| c[1].real().to_f64()).collect::<Vec<f64>>(),
                                ],
                                [
                                    if m.iter().any(|c| !c[0].imag().is_zero())
                                    {
                                        m.iter().map(|c| c[0].imag().to_f64()).collect::<Vec<f64>>()
                                    }
                                    else
                                    {
                                        Vec::new()
                                    },
                                    if m.iter().any(|c| !c[1].imag().is_zero())
                                    {
                                        m.iter().map(|c| c[1].imag().to_f64()).collect::<Vec<f64>>()
                                    }
                                    else
                                    {
                                        Vec::new()
                                    },
                                ],
                            ];
                            re_or_im = (
                                points2d[0].iter().flatten().any(|a| *a != 0.0),
                                points2d[1].iter().flatten().any(|a| *a != 0.0),
                            );
                        }
                        _ =>
                        {}
                    }
                }
                _ =>
                {}
            }
        }
        else if !has_y || !has_x
        {
            d2_or_d3.0 = true;
            (points2d, re_or_im) = get_list_2d(&func, options);
            if points2d[0][1].is_empty() && points2d[1][1].is_empty()
            {
                fail(options, &colors);
                return (
                    (false, false),
                    (false, false),
                    false,
                    true,
                    Default::default(),
                    Default::default(),
                );
            }
            if !has_x
            {
                points2d[1][0] = points2d[0][0].clone();
                points2d[0].swap(0, 1);
                points2d[1].swap(0, 1);
            }
            if options.flat
            {
                re_or_im.1 = false;
                points2d[0].swap(0, 1);
                points2d[0][1] = points2d[1][1].clone();
                points2d[1] = Default::default();
            }
            else if options.depth
            {
                re_or_im.1 = false;
                d2_or_d3 = (false, true);
                points3d[0][0] = points2d[0][0].clone();
                points3d[0][1] = points2d[0][1].clone();
                points3d[0][2] = points2d[1][1].clone();
                points2d = Default::default();
            }
        }
        else
        {
            d2_or_d3.1 = true;
            (points3d, re_or_im) = get_list_3d(&func, options);
            if points3d[0][2].is_empty() && points3d[1][2].is_empty()
            {
                fail(options, &colors);
                return (
                    (false, false),
                    (false, false),
                    false,
                    true,
                    Default::default(),
                    Default::default(),
                );
            }
            if options.depth
            {
                re_or_im.1 = false;
                d2_or_d3 = (false, true);
                points3d[0][0] = points2d[0][0].clone();
                points3d[0][1] = points2d[0][1].clone();
                points3d[0][2] = points2d[1][1].clone();
                points2d = Default::default();
            }
        }
        (d2_or_d3, re_or_im, lines, false, points2d, points3d)
    })
}
pub fn get_list_2d(func: &[NumStr], options: Options) -> ([[Vec<f64>; 2]; 2], (bool, bool))
{
    if let Num(n) = &func[0]
    {
        if func.len() == 1 && n.is_zero()
        {
            return Default::default();
        }
    }
    let mut data: [[Vec<f64>; 2]; 2] = [
        [
            Vec::with_capacity(options.samples_2d + 1),
            Vec::with_capacity(options.samples_2d + 1),
        ],
        [Vec::new(), Vec::with_capacity(options.samples_2d + 1)],
    ];
    let den_range = (options.xr.1 - options.xr.0) / options.samples_2d as f64;
    let mut zero = (false, false);
    for i in 0..=options.samples_2d
    {
        let n = options.xr.0 + i as f64 * den_range;
        let num = Num(Complex::with_val(options.prec, n));
        match do_math(
            func.iter()
                .map(|i| match i
                {
                    Str(s) if s == "x" || s == "y" => num.clone(),
                    _ => i.clone(),
                })
                .collect(),
            options,
        )
        {
            Ok(Num(num)) =>
            {
                let complex = num.real().is_finite();
                if complex
                {
                    let f = num.real().to_f64();
                    if (f * 1e8).round() / 1e8 != 0.0
                    {
                        zero.0 = true
                    }
                    data[0][0].push(n);
                    data[0][1].push(f);
                }
                if num.imag().is_finite()
                {
                    let f = num.imag().to_f64();
                    if (f * 1e8).round() / 1e8 != 0.0
                    {
                        zero.1 = true
                    }
                    if !complex
                    {
                        data[0][0].push(n);
                        data[0][1].push(f64::NAN);
                    }
                    data[1][1].push(f);
                }
            }
            Ok(Vector(v)) =>
            {
                for num in v
                {
                    let complex = num.real().is_finite();
                    if complex
                    {
                        let f = num.real().to_f64();
                        if (f * 1e8).round() / 1e8 != 0.0
                        {
                            zero.0 = true
                        }
                        data[0][0].push(n);
                        data[0][1].push(f);
                    }
                    if num.imag().is_finite()
                    {
                        let f = num.imag().to_f64();
                        if (f * 1e8).round() / 1e8 != 0.0
                        {
                            zero.1 = true
                        }
                        if !complex
                        {
                            data[0][0].push(n);
                            data[0][1].push(f64::NAN);
                        }
                        data[1][1].push(f);
                    }
                }
            }
            Ok(Matrix(m)) =>
            {
                for v in m
                {
                    for num in v
                    {
                        let complex = num.real().is_finite();
                        if complex
                        {
                            let f = num.real().to_f64();
                            if (f * 1e8).round() / 1e8 != 0.0
                            {
                                zero.0 = true
                            }
                            data[0][0].push(n);
                            data[0][1].push(f);
                        }
                        if num.imag().is_finite()
                        {
                            let f = num.imag().to_f64();
                            if (f * 1e8).round() / 1e8 != 0.0
                            {
                                zero.1 = true
                            }
                            if !complex
                            {
                                data[0][0].push(n);
                                data[0][1].push(f64::NAN);
                            }
                            data[1][1].push(f);
                        }
                    }
                }
            }
            _ =>
            {}
        }
    }
    if !zero.0
    {
        data[0][1] = Vec::new();
    }
    if !zero.1
    {
        data[1][1] = Vec::new();
    }
    (data, zero)
}
pub fn get_list_3d(func: &[NumStr], options: Options) -> ([[Vec<f64>; 3]; 2], (bool, bool))
{
    if let Num(n) = &func[0]
    {
        if func.len() == 1 && n.is_zero()
        {
            return Default::default();
        }
    }
    let mut data: [[Vec<f64>; 3]; 2] = [
        [
            Vec::with_capacity(options.samples_3d.0 + 1),
            Vec::with_capacity(options.samples_3d.1 + 1),
            Vec::with_capacity((options.samples_3d.0 + 1) * (options.samples_3d.1 + 1)),
        ],
        [
            Vec::new(),
            Vec::new(),
            Vec::with_capacity((options.samples_3d.0 + 1) * (options.samples_3d.1 + 1)),
        ],
    ];
    let den_x_range = (options.xr.1 - options.xr.0) / options.samples_3d.0 as f64;
    let den_y_range = (options.yr.1 - options.yr.0) / options.samples_3d.1 as f64;
    let mut modified: Vec<NumStr>;
    let mut zero = (false, false);
    for i in 0..=options.samples_3d.0
    {
        let n = options.xr.0 + i as f64 * den_x_range;
        let num = Num(Complex::with_val(options.prec, n));
        modified = func
            .iter()
            .map(|i| match i
            {
                Str(s) if s == "x" => num.clone(),
                _ => i.clone(),
            })
            .collect();
        for g in 0..=options.samples_3d.1
        {
            let f = options.yr.0 + g as f64 * den_y_range;
            let num = Num(Complex::with_val(options.prec, f));
            match do_math(
                modified
                    .iter()
                    .map(|j| match j
                    {
                        Str(s) if s == "y" => num.clone(),
                        _ => j.clone(),
                    })
                    .collect(),
                options,
            )
            {
                Ok(Num(num)) =>
                {
                    let complex = num.real().is_finite();
                    if complex
                    {
                        if (num.real().to_f64() * 1e8).round() / 1e8 != 0.0
                        {
                            zero.0 = true
                        }
                        data[0][0].push(n);
                        data[0][1].push(f);
                        data[0][2].push(num.real().to_f64());
                    }
                    if num.imag().is_finite()
                    {
                        if (num.imag().to_f64() * 1e8).round() / 1e8 != 0.0
                        {
                            zero.1 = true
                        }
                        if !complex
                        {
                            data[0][0].push(n);
                            data[0][1].push(f);
                            data[0][2].push(f64::NAN);
                        }
                        data[1][2].push(num.imag().to_f64());
                    }
                }
                Ok(Vector(v)) =>
                {
                    for num in v
                    {
                        let complex = num.real().is_finite();
                        if complex
                        {
                            if (num.real().to_f64() * 1e8).round() / 1e8 != 0.0
                            {
                                zero.0 = true
                            }
                            data[0][0].push(n);
                            data[0][1].push(f);
                            data[0][2].push(num.real().to_f64());
                        }
                        if num.imag().is_finite()
                        {
                            if (num.imag().to_f64() * 1e8).round() / 1e8 != 0.0
                            {
                                zero.1 = true
                            }
                            if !complex
                            {
                                data[0][0].push(n);
                                data[0][1].push(f);
                                data[0][2].push(f64::NAN);
                            }
                            data[1][2].push(num.imag().to_f64());
                        }
                    }
                    continue;
                }
                Ok(Matrix(m)) =>
                {
                    for v in m
                    {
                        for num in v
                        {
                            let complex = num.real().is_finite();
                            if complex
                            {
                                if (num.real().to_f64() * 1e8).round() / 1e8 != 0.0
                                {
                                    zero.0 = true
                                }
                                data[0][0].push(n);
                                data[0][1].push(f);
                                data[0][2].push(num.real().to_f64());
                            }
                            if num.imag().is_finite()
                            {
                                if (num.imag().to_f64() * 1e8).round() / 1e8 != 0.0
                                {
                                    zero.1 = true
                                }
                                if !complex
                                {
                                    data[0][0].push(n);
                                    data[0][1].push(f);
                                    data[0][2].push(f64::NAN);
                                }
                                data[1][2].push(num.imag().to_f64());
                            }
                        }
                    }
                }
                _ =>
                {}
            }
        }
    }
    if !zero.0
    {
        data[0][2] = Vec::new();
    }
    if !zero.1
    {
        data[1][2] = Vec::new();
    }
    (data, zero)
}
fn fail(options: Options, colors: &Colors)
{
    print!(
        "\x1b[G\x1b[KNo data to plot\n\x1b[G{}",
        prompt(options, colors)
    );
    stdout().flush().unwrap();
}