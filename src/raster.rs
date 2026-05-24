// TODO: handle or scrap rotation

pub fn rect<F>(width: usize, height: usize, border_only: bool, mut f: F)
where
    F: FnMut(usize, usize),
{
    if border_only {
        for y in 0..height {
            for x in 0..width {
                if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    f(x, y);
                }
            }
        }
    } else {
        for y in 0..height {
            for x in 0..width {
                f(x, y);
            }
        }
    }
}

pub fn line<F>(x0: usize, y0: usize, x1: usize, y1: usize, mut f: F)
where
    F: FnMut(usize, usize),
{
    let mut x = x0 as isize;
    let mut y = y0 as isize;
    let x1 = x1 as isize;
    let y1 = y1 as isize;

    let dx = (x1 - x).abs();
    let dy = -(y1 - y).abs();
    let sx = if x < x1 { 1 } else { -1 };
    let sy = if y < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        f(x as usize, y as usize);
        if x == x1 && y == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
}

pub fn oval<F>(cx: usize, cy: usize, rx: usize, ry: usize, border_only: bool, mut f: F)
where
    F: FnMut(usize, usize),
{
    if rx == 0 || ry == 0 {
        f(cx, cy);
        return;
    }

    let w = (2 * rx + 1) as isize;
    let h = (2 * ry + 1) as isize;
    let w2 = w * w;
    let h2 = h * h;
    let limit = w2 * h2;

    let start_y = cy.saturating_sub(ry);
    let start_x = cx.saturating_sub(rx);
    let end_y = cy + ry;
    let end_x = cx + rx;

    let is_inside = |x: isize, y: isize| -> bool {
        let dx = x - cx as isize;
        let dy = y - cy as isize;
        4 * dx * dx * h2 + 4 * dy * dy * w2 <= limit
    };

    for y in start_y..=end_y {
        for x in start_x..=end_x {
            let x_i = x as isize;
            let y_i = y as isize;

            if is_inside(x_i, y_i) {
                if border_only {
                    let is_border = !is_inside(x_i + 1, y_i)
                        || !is_inside(x_i - 1, y_i)
                        || !is_inside(x_i, y_i + 1)
                        || !is_inside(x_i, y_i - 1);

                    if is_border {
                        f(x, y);
                    }
                } else {
                    f(x, y);
                }
            }
        }
    }
}

pub fn triangle<F>(
    p0: (usize, usize),
    p1: (usize, usize),
    p2: (usize, usize),
    border_only: bool,
    mut f: F,
) where
    F: FnMut(usize, usize),
{
    let min_x = p0.0.min(p1.0).min(p2.0);
    let max_x = p0.0.max(p1.0).max(p2.0);
    let min_y = p0.1.min(p1.1).min(p2.1);
    let max_y = p0.1.max(p1.1).max(p2.1);

    let edge = |a: (usize, usize), b: (usize, usize), cx: isize, cy: isize| -> isize {
        (cx - a.0 as isize) * (b.1 as isize - a.1 as isize)
            - (cy - a.1 as isize) * (b.0 as isize - a.0 as isize)
    };

    let is_inside = |x: isize, y: isize| -> bool {
        let w0 = edge(p1, p2, x, y);
        let w1 = edge(p2, p0, x, y);
        let w2 = edge(p0, p1, x, y);
        (w0 >= 0 && w1 >= 0 && w2 >= 0) || (w0 <= 0 && w1 <= 0 && w2 <= 0)
    };

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let x_i = x as isize;
            let y_i = y as isize;

            if is_inside(x_i, y_i) {
                if border_only {
                    let is_border = !is_inside(x_i + 1, y_i)
                        || !is_inside(x_i - 1, y_i)
                        || !is_inside(x_i, y_i + 1)
                        || !is_inside(x_i, y_i - 1);

                    if is_border {
                        f(x, y);
                    }
                } else {
                    f(x, y);
                }
            }
        }
    }
}
