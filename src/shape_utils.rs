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

    if border_only {
        // TODO: fix weird holes at the cardinals of the oval...
        // this border only formula seems fundimentally flawed honestly, it doesnt match with the filled oval

        let rx_i = rx as isize;
        let ry_i = ry as isize;

        let mut x = 0;
        let mut y = ry_i;

        let rx2 = rx_i * rx_i;
        let ry2 = ry_i * ry_i;

        let mut p = ry2 - rx2 * ry_i + rx2 / 4;

        let mut plot_four = |x: isize, y: isize| {
            if let (Some(x1), Some(y1)) = (cx.checked_add_signed(x), cy.checked_add_signed(y)) {
                f(x1, y1);
            }
            if let (Some(x2), Some(y1)) = (cx.checked_add_signed(-x), cy.checked_add_signed(y)) {
                f(x2, y1);
            }
            if let (Some(x1), Some(y2)) = (cx.checked_add_signed(x), cy.checked_add_signed(-y)) {
                f(x1, y2);
            }
            if let (Some(x2), Some(y2)) = (cx.checked_add_signed(-x), cy.checked_add_signed(-y)) {
                f(x2, y2);
            }
        };

        while 2 * ry2 * x <= 2 * rx2 * y {
            plot_four(x, y);
            x += 1;
            if p < 0 {
                p += 2 * ry2 * x + ry2;
            } else {
                y -= 1;
                p += 2 * ry2 * x - 2 * rx2 * y + ry2;
            }
        }

        p = (ry2 as f64 * (x as f64 + 0.5).powi(2) + rx2 as f64 * (y - 1) as f64 * (y - 1) as f64
            - (rx2 * ry2) as f64) as isize;
        while y >= 0 {
            plot_four(x, y);
            y -= 1;
            if p > 0 {
                p += rx2 - 2 * rx2 * y;
            } else {
                x += 1;
                p += 2 * ry2 * x - 2 * rx2 * y + rx2;
            }
        }
    } else {
        let rx_i = rx as isize;
        let ry_i = ry as isize;
        let rx2 = rx_i * rx_i;
        let ry2 = ry_i * ry_i;

        let start_y = cy.saturating_sub(ry);
        let start_x = cx.saturating_sub(rx);

        for y in start_y..=(cy + ry) {
            let dy = y as isize - cy as isize;
            for x in start_x..=(cx + rx) {
                let dx = x as isize - cx as isize;
                if dx * dx * ry2 + dy * dy * rx2 <= rx2 * ry2 {
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
    if border_only {
        line(p0.0, p0.1, p1.0, p1.1, &mut f);
        line(p1.0, p1.1, p2.0, p2.1, &mut f);
        line(p2.0, p2.1, p0.0, p0.1, &mut f);
    } else {
        let min_x = p0.0.min(p1.0).min(p2.0);
        let max_x = p0.0.max(p1.0).max(p2.0);
        let min_y = p0.1.min(p1.1).min(p2.1);
        let max_y = p0.1.max(p1.1).max(p2.1);

        // Signed math helper to calculate edge weights
        let edge = |a: (usize, usize), b: (usize, usize), c: (usize, usize)| -> isize {
            (c.0 as isize - a.0 as isize) * (b.1 as isize - a.1 as isize)
                - (c.1 as isize - a.1 as isize) * (b.0 as isize - a.0 as isize)
        };

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = (x, y);
                let w0 = edge(p1, p2, p);
                let w1 = edge(p2, p0, p);
                let w2 = edge(p0, p1, p);

                if (w0 >= 0 && w1 >= 0 && w2 >= 0) || (w0 <= 0 && w1 <= 0 && w2 <= 0) {
                    f(x, y);
                }
            }
        }
    }
}
