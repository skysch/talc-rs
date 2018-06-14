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


////////////////////////////////////////////////////////////////////////////////
// segment
////////////////////////////////////////////////////////////////////////////////
/// Draws a line segment.
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
/// `endpoints`: The [`Point`]s of the line segment's endpoints.
///
///
pub fn segment<C, B>(
    canvas: &mut C,
    brush: &mut B,
    endpoints: [Point; 2])
    where
        C: Canvas,
        B: Brush
{
    unimplemented!()
}

////////////////////////////////////////////////////////////////////////////////
// line_horizontal
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
///
pub fn segment_horizontal<C, B>(
    canvas: &mut C,
    brush: &mut B,
    pt: Point,
    x: f32)
    where
        C: Canvas,
        B: Brush
{
    unimplemented!()
}

////////////////////////////////////////////////////////////////////////////////
// line_vertical
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
/// [`Canvas`]: ../talc/trait.Canvas.html
/// [`Brush`]: ../talc/trait.Brush.html
/// [`Point`]: ../talc/struct.Point.html
pub fn segment_vertical<C, B>(
    canvas: &mut C,
    brush: &mut B,
    pt: Point,
    y: f32)
    where
        C: Canvas,
        B: Brush
{
    unimplemented!()
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
// ray_segment
////////////////////////////////////////////////////////////////////////////////
/// Draws a line segment as a lengh-delimitted ray.
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
/// `from`: The [`Point`] of the start of the ray.
///
/// `angle`: The slope angle of the ray in radians.
///
/// `len`: The length of the line segment.
///
/// [`Canvas`]: ../talc/trait.Canvas.html
/// [`Brush`]: ../talc/trait.Brush.html
/// [`Point`]: ../talc/struct.Point.html
pub fn ray_segment<C, B>(
    canvas: &mut C,
    brush: &mut B,
    from: Point,
    angle: f64,
    len: f64)
    where
        C: Canvas,
        B: Brush
{
    unimplemented!()
}

////////////////////////////////////////////////////////////////////////////////
// ray
////////////////////////////////////////////////////////////////////////////////
/// Draws a ray.
///
/// The resulting ray will be cropped within the rect of the canvas.
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
/// [`Canvas`]: ../talc/trait.Canvas.html
/// [`Brush`]: ../talc/trait.Brush.html
/// [`Point`]: ../talc/struct.Point.html
pub fn ray<C, B>(
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
