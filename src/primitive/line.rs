// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! Line drawing primitives.
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use brush::Brush;
use canvas::Canvas;
use geometry::Point;
use geometry::clip_segment_to_rect;
use utilities::clipped;
use utilities::ordered;

////////////////////////////////////////////////////////////////////////////////
// segment
////////////////////////////////////////////////////////////////////////////////
/// Draws a line segment.
///
/// The resulting line segment will be cropped within the boundaries of the 
/// canvas.
///
/// # Arguments
///
/// `canvas`: The [`Canvas`] to draw to.
///
/// `brush`: The [`Brush`] to draw with.
///
/// `endpoints`: The [`Point`]s of the line segment's endpoints.
///
/// [`Canvas`]: ../canvas/trait.Canvas.html
/// [`Brush`]: ../brush/trait.Brush.html
/// [`Point`]: ../geometry/struct.Point.html
pub fn segment<C, B>(
    canvas: &mut C,
    brush: &mut B,
    endpoints: [Point; 2])
    where
        C: Canvas,
        B: Brush
{
    let rect = canvas.virtual_bounding_rect(brush);
    if let Some(segment) = clip_segment_to_rect(endpoints, rect) {
        let [Point { x: xa, y: ya }, Point { x: xb, y: yb }] = segment;
        
        if (yb - ya).abs() < (xb - xa).abs() {
            let [Point { x: xa, y: ya }, Point { x: xb, y: yb }]
                 = Point::x_ordered(segment);

            // Horizontally-oriented line.
            let dx = xb - xa;
            let dy = yb - ya;
            let mut x = xa;
            while x < xb {
                // Solve parametric line equation for stroke and y-coordinate.
                let t = (x - xa) / dx;
                let y = ya + dy * t;
                brush.apply(canvas, Point { x, y });
                x += 1.0;
            }

        } else {
            let [Point { x: xa, y: ya }, Point { x: xb, y: yb }]
                 = Point::y_ordered(segment);

            // Horizontally-oriented line.
            let dx = xb - xa;
            let dy = yb - ya;
            let mut y = ya;
            while y < yb {
                // Solve parametric line equation for stroke an x-coordinate.
                let t = (y - ya) / dy;
                let x = xa + dx * t;
                brush.apply(canvas, Point { x, y });
                y += 1.0;
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// segment_horizontal
////////////////////////////////////////////////////////////////////////////////
/// Draws a horizontal line segment.
///
/// The resulting line segment will be cropped within the rect of the 
/// canvas.
///
/// # Arguments
///
/// `canvas`: The [`Canvas`] to draw to.
///
/// `brush`: The [`Brush`] to draw with.
///
/// `pt`: The [`Point`] of one of the line segment's endpoints.
///
/// `x`: The x-coordinate of the opptite endpoint.
///
/// [`Canvas`]: ../canvas/trait.Canvas.html
/// [`Brush`]: ../brush/trait.Brush.html
/// [`Point`]: ../geometry/struct.Point.html
#[inline]
pub fn segment_horizontal<C, B>(
    canvas: &mut C,
    brush: &mut B,
    pt: Point,
    x: f32)
    where
        C: Canvas,
        B: Brush
{
    let rect = canvas.virtual_bounding_rect(brush);
    if rect.contains_y(pt.y) {
        let clip_order = clipped((pt.x, x), rect.left, rect.right)
            .map(|(a, b)| ordered(a, b));
        if let Some((xa, xb)) = clip_order {
            // let dx = xb - xa;
            let mut x = xa;
            while x <= xb {
                // Solve parametric line equation for stroke.
                // let t = (x - xa) / dx;
                brush.apply(canvas, Point { x, y: pt.y });
                x += 1.0;
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// segment_vertical
////////////////////////////////////////////////////////////////////////////////
/// Draws a vertical line segment.
///
/// The resulting line segment will be cropped within the rect of the 
/// canvas.
///
/// # Arguments
///
/// `canvas`: The [`Canvas`] to draw to.
///
/// `brush`: The [`Brush`] to draw with.
///
/// `pt`: The [`Point`] of one of the line segment's endpoints.
///
/// `y`: The y-coordinate of the opptite endpoint.
///
/// [`Canvas`]: ../canvas/trait.Canvas.html
/// [`Brush`]: ../brush/trait.Brush.html
/// [`Point`]: ../geometry/struct.Point.html
#[inline]
pub fn segment_vertical<C, B>(
    canvas: &mut C,
    brush: &mut B,
    pt: Point,
    y: f32)
    where
        C: Canvas,
        B: Brush
{
    let rect = canvas.virtual_bounding_rect(brush);
    if rect.contains_x(pt.x) {
        let clip_order = clipped((pt.y, y), rect.top, rect.bottom)
            .map(|(a, b)| ordered(a, b));
        if let Some((ya, yb)) = clip_order {
            // let dy = yb - ya;
            let mut y = ya;
            while y <= yb {
                // Solve parametric line equation for stroke.
                // let t = (y - ya) / dy;
                brush.apply(canvas, Point { x: pt.x, y });
                y += 1.0;
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// segment_extended
////////////////////////////////////////////////////////////////////////////////
/// Draws a line overlaying a line segment.
///
/// The resulting line will be cropped within the rect of the canvas.
///
/// # Arguments
///
/// `canvas`: The [`Canvas`] to draw to.
///
/// `brush`: The [`Brush`] to draw with.
///
/// `segment_endpoints`: The [`Point`] of one of the line segment's
///
/// endpoints.
///
/// [`Canvas`]: ../talc/trait.Canvas.html
/// [`Brush`]: ../talc/trait.Brush.html
/// [`Point`]: ../talc/struct.Point.html
pub fn segment_extended<C, B>(
    canvas: &mut C,
    brush: &mut B,
    segment_endpoints: [Point; 2])
    where
        C: Canvas,
        B: Brush
{
    unimplemented!()
    // let [mut a, mut b] = segment_endpoints;

    // // Calculate line slope.
    // let rise = b.y - a.y;
    // let run = b.x - a.x;
    // if rise == 0.0 {
    //     line_horizontal(canvas, brush, a.y)

    // } if run == 0.0 {
    //     line_vertical(canvas, brush, a.x)

    // } else {
    //     let slope = rise / run;
    //     line(canvas, brush, a, slope as f64);
    // }
}

////////////////////////////////////////////////////////////////////////////////
// line
////////////////////////////////////////////////////////////////////////////////
/// Draws a line.
///
/// The resulting line will be cropped within the rect of the canvas.
///
/// # Arguments
///
/// `canvas`: The [`Canvas`] to draw to.
///
/// `brush`: The [`Brush`] to draw with.
///
/// `pt`: The [`Point`] of a point on the line.
///
/// `angle`: The slope angle of the line in radians.
///
/// [`Canvas`]: ../talc/trait.Canvas.html
/// [`Brush`]: ../talc/trait.Brush.html
/// [`Point`]: ../talc/struct.Point.html
pub fn line<C, B>(
    canvas: &mut C,
    brush: &mut B,
    pt: Point,
    angle: f64)
    where
        C: Canvas,
        B: Brush
{
    unimplemented!()
}

////////////////////////////////////////////////////////////////////////////////
// line_horizontal
////////////////////////////////////////////////////////////////////////////////
/// Draws an horizontal line.
///
/// The resulting line will be cropped within the rect of the canvas.
///
/// # Arguments
///
/// `canvas`: The [`Canvas`] to draw to.
///
/// `brush`: The [`Brush`] to draw with.
///
/// `y`: The y-coordinate of the line.
///
/// [`Canvas`]: ../talc/trait.Canvas.html
/// [`Brush`]: ../talc/trait.Brush.html
pub fn line_horizontal<C, B>(
    canvas: &mut C,
    brush: &mut B,
    y: f32)
    where
        C: Canvas,
        B: Brush
{
    unimplemented!()
    // for x in canvas.left() .. canvas.right() {
    //     brush.apply(canvas, Point { x, y })
    // }
}

////////////////////////////////////////////////////////////////////////////////
// line_vertical
////////////////////////////////////////////////////////////////////////////////
/// Draws an vertical line.
///
/// The resulting line will be cropped within the rect of the canvas.
///
/// # Arguments
///
/// `canvas`: The [`Canvas`] to draw to.
///
/// `brush`: The [`Brush`] to draw with.
///
/// `x`: The x-coordinate of the line.
///
/// [`Canvas`]: ../talc/trait.Canvas.html
/// [`Brush`]: ../talc/trait.Brush.html
pub fn line_vertical<C, B>(
    canvas: &mut C,
    brush: &mut B,
    x: f32)
    where
        C: Canvas,
        B: Brush
{
    unimplemented!()
    // for y in canvas.top() .. canvas.bottom() {
    //     brush.apply(canvas, Point { x, y })
    // }
}


////////////////////////////////////////////////////////////////////////////////
// normal_segment
////////////////////////////////////////////////////////////////////////////////
/// Draws a line segment as a normal to a ray.
///
/// The resulting line segment will centered on the ray and cropped within the
/// rect of the canvas.
///
/// # Arguments
///
/// `canvas`: The [`Canvas`] to draw to.
///
/// `brush`: The [`Brush`] to draw with.
///
/// `from`: The [`Point`] of the start of the ray.
///
/// `angle`: The slope angle of the ray in radians.
///
/// `dist`: The line segment's distance from the ray's start ptition.
///
/// `len`: The length of the line segment.
///
/// [`Canvas`]: ../talc/trait.Canvas.html
/// [`Brush`]: ../talc/trait.Brush.html
/// [`Point`]: ../talc/struct.Point.html
pub fn normal_segment<C, B>(
    canvas: &mut C,
    brush: &mut B,
    from: Point,
    angle: f64,
    dist: f64,
    len: f64)
    where
        C: Canvas,
        B: Brush
{
    unimplemented!()
}
