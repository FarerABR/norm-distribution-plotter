use plotters::prelude::*;
use std::{
    env,
    f32::{consts, INFINITY},
};

/**
## config for IO argumants

```
param: [String]

returns: miu:i32, vrc:i32, perc:i32, dom:i32
```
*/
fn config(args: &[String]) -> (i32, i32, i32, i32) {
    if args.len() < 4 {
        panic!("not enough argumants");
    }

    (
        args[1].parse::<i32>().unwrap(),
        args[2].parse::<i32>().unwrap(),
        args[3].parse::<i32>().unwrap(),
        args[4].parse::<i32>().unwrap(),
    )
}

fn draw(miu: i32, vrc: i32, perc: f32, dom: f32) {
    let mut y: Vec<(f32, f32)> = Vec::new();

    for i in (-1.0 * dom * perc) as i32..=(dom * perc) as i32 {
        let temp = norm(i as f32 / perc, miu as f32, vrc as f32);
        if temp != INFINITY {
            y.push((i as f32 / perc, temp));
        }
    }

    let drawing_area = BitMapBackend::new("./figure.png", (1920, 1080)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut max_n = 0.0;
    for i in &y {
        if i.1 > max_n {
            max_n = i.1;
        }
    }

    let mut chart = ChartBuilder::on(&drawing_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption(
            format!("norm distibution - N({}, {})", miu, vrc),
            ("mono", 40),
        )
        .build_cartesian_2d(-1.0 * dom..dom + 1.0, -0.001f32..max_n + 0.01)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    chart
        .draw_series(
            y.iter()
                .map(|&point| Circle::new(point, 2, Into::<ShapeStyle>::into(&RED).filled())),
        )
        .unwrap();
}

/**
# The norm distribution function

x, miu, variance
*/
fn norm(x: f32, m: f32, v: f32) -> f32 {
    let p = -1.0 * ((x - m).powi(2) / (2.0 * v));
    let sqrt = 1.0 / f32::sqrt(2.0 * consts::PI * v);

    sqrt * f32::powf(consts::E, p)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (miu, vrc, perc, dom) = config(&args);
    draw(miu, vrc, perc as f32, dom as f32);
}
