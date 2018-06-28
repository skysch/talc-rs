// Copyright 2018 Skylor R. Schermer.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
////////////////////////////////////////////////////////////////////////////////
//! Text drawing primitives.
////////////////////////////////////////////////////////////////////////////////

// Local imports.
use brush::Brush;
use canvas::Canvas;
use geometry::Point;
use geometry::Scale;
use pattern::Pattern;
use super::line::segment_horizontal;

// External library imports.
use rusttype::GlyphId;
use rusttype::HMetrics;
use rusttype::PositionedGlyph;
use rusttype::ScaledGlyph;
use rusttype::VMetrics;
use rusttype;


pub type Font<'a> = rusttype::Font<'a>;

////////////////////////////////////////////////////////////////////////////////
// FontStyle
////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontStyle {
    scale: rusttype::Scale,
    underline: bool,
}

impl FontStyle {
    #[inline]
    pub fn new(
        scale: Scale,
        underline: bool)
        -> Self
    {
        // Convert talc Scale to rusttype Scale.
        let scale = rusttype::Scale {
            x: scale.horz,
            y: scale.vert,
        };

        FontStyle { scale, underline }
    }
}


////////////////////////////////////////////////////////////////////////////////
// glyph
////////////////////////////////////////////////////////////////////////////////
#[inline]
pub fn glyph<C, P, B, X>(
    canvas: &mut C,
    pattern: &P,
    underline: &B,
    font: &Font,
    font_style: FontStyle,
    pt: Point,
    character: char)
    where
        C: Canvas<Pixel=X>,
        P: Pattern<X>,
        B: Brush<X>,
{
    prepare_glyph(font, font_style, character)
        .draw(canvas, pattern, underline, pt)
}


////////////////////////////////////////////////////////////////////////////////
// text
////////////////////////////////////////////////////////////////////////////////
#[inline]
pub fn text<C, P, B, X>(
    canvas: &mut C,
    pattern: &P,
    underline: &B,
    font: &Font,
    font_style: FontStyle,
    pt: Point,
    text: &str)
    where
        C: Canvas<Pixel=X>,
        P: Pattern<X>,
        B: Brush<X>,
{
    prepare_text(font, font_style, text)
        .draw(canvas, pattern, underline, pt)
}


////////////////////////////////////////////////////////////////////////////////
// PreparedText
////////////////////////////////////////////////////////////////////////////////
pub struct PreparedText<'f> {
    glyphs: Vec<OffsetGlyph<'f>>,
    font_style: FontStyle,
    v_metrics: VMetrics,
}

impl<'f> PreparedText<'f> {
    #[inline]
    pub fn font_style(&self) -> FontStyle {
        self.font_style
    }

    #[inline]
    pub fn width(&self) -> f32 {
        debug_assert!(self.glyphs.len() > 0);

        let last = self.glyphs.last().unwrap();
        last.glyph.h_metrics().advance_width + last.offset
    }

    #[inline]
    pub fn height(&self) -> f32 {
        self.v_metrics.ascent - self.v_metrics.descent
    }
    
    #[inline]
    pub fn draw_clone<C, P, B, X>(
        &self,
        canvas: &mut C,
        pattern: &P,
        underline: &B,
        mut pt: Point)
        where
            C: Canvas<Pixel=X>,
            P: Pattern<X>,
            B: Brush<X>,
    {
        pt.y += self.v_metrics.ascent;
        
        let positioned = self.glyphs.iter().map(|g| g.clone().relative_to(pt));
        PreparedText::draw_positioned(canvas, pattern, pt, positioned);

        if self.font_style.underline {
            let u_left = Point { x: pt.x, y: pt.y };
            let u_right = u_left.x + self.width();
            segment_horizontal(canvas, underline, u_left, u_right);
        }
    }

    #[inline]
    pub fn draw<C, P, B, X>(
        self,
        canvas: &mut C,
        pattern: &P,
        underline: &B,
        mut pt: Point)
        where
            C: Canvas<Pixel=X>,
            P: Pattern<X>,
            B: Brush<X>,
    {
        // Shift point.
        pt.y += self.v_metrics.ascent;

        // Get underline info.
        let draw_underline = self.font_style.underline;
        let u_left = Point { x: pt.x, y: pt.y + 2.0 };
        let u_right = u_left.x + self.width();


        let positioned = self.glyphs.into_iter().map(|g| g.relative_to(pt));
        PreparedText::draw_positioned(canvas, pattern, pt, positioned);

        if draw_underline {
            segment_horizontal(canvas, underline, u_left, u_right);
        }
    }

    fn draw_positioned<C, P, X, I,>(
        canvas: &mut C,
        pattern: &P,
        pt: Point,
        positioned: I)
        where
            C: Canvas<Pixel=X>,
            P: Pattern<X>,
            I: Iterator<Item=PositionedGlyph<'f>>
    {
        // Loop through the glyphs in the text, positioning each one on a line.
        for glyph in positioned {
            if let Some(bounding_box) = glyph.pixel_bounding_box() {
                // Draw the glyph into the image per-pixel by using the draw
                // closure, which scans the bounding box and 
                glyph.draw(|x, y, v| {
                    // Draw text glyph.
                    pattern.apply(canvas, Point {
                        // Offset the position by the glyph bounding box
                        x: (x as i32 + bounding_box.min.x) as f32,
                        y: (y as i32 + bounding_box.min.y) as f32,
                    }, v);
                });
            }
        }
    }
}


////////////////////////////////////////////////////////////////////////////////
// prepare_glyph
////////////////////////////////////////////////////////////////////////////////
#[derive(Clone)]
pub struct OffsetGlyph<'f> {
    pub(in primitive::text) glyph: ScaledGlyph<'f>,
    pub(in primitive::text) offset: f32,
}

impl<'f> OffsetGlyph<'f> {
    #[inline]
    pub fn new(glyph: ScaledGlyph<'f>) -> Self {
        OffsetGlyph { glyph, offset: 0.0 }
    }

    #[inline]
    pub fn new_after(
        glyph: ScaledGlyph<'f>,
        prev: OffsetGlyph<'f>,
        font_style: FontStyle)
        -> Self
    {
        let offset = glyph.h_metrics().advance_width + glyph
            .font()
            .unwrap() // OffsetGlyph cannot be standalone.
            .pair_kerning(font_style.scale, prev.glyph.id(), glyph.id());

        OffsetGlyph { glyph, offset }
    }

    #[inline]
    pub fn relative_to(self, pt: Point) -> PositionedGlyph<'f> {
        self.glyph.positioned(rusttype::point(pt.x + self.offset, pt.y))
    }
}



////////////////////////////////////////////////////////////////////////////////
// prepare_glyph
////////////////////////////////////////////////////////////////////////////////
#[inline]
pub fn prepare_glyph<'f>(
    font: &'f Font,
    font_style: FontStyle,
    character: char)
    -> PreparedText<'f>
{
    // Layout the glyph geometry.
    let glyph = OffsetGlyph::new(font
        .glyph(character)
        .scaled(font_style.scale));

    PreparedText {
        glyphs: vec![glyph],
        font_style,
        v_metrics: font.v_metrics(font_style.scale),
    }
}


////////////////////////////////////////////////////////////////////////////////
// prepare_text
////////////////////////////////////////////////////////////////////////////////
#[inline]
pub fn prepare_text<'f>(
    font: &'f Font,
    font_style: FontStyle,
    text: &str)
    -> PreparedText<'f>
{
    // Layout the glyph geometry.
    let layout = OffsetLayoutIter {
        font, 
        chars: text.chars(),
        caret: 0.0,
        font_style,
        last_glyph: None
    };

    PreparedText {
        glyphs: layout.collect(),
        font_style,
        v_metrics: font.v_metrics(font_style.scale),
    }
}


struct OffsetLayoutIter<'a, 'b> {
    font: &'a Font<'a>,
    chars: ::std::str::Chars<'b>,
    caret: f32,
    font_style: FontStyle,
    last_glyph: Option<GlyphId>,
}

impl<'a, 'b> Iterator for OffsetLayoutIter<'a, 'b> {
    type Item = OffsetGlyph<'a>;

    fn next(&mut self) -> Option<OffsetGlyph<'a>> {
        self.chars.next().map(|c| {
            let scale = self.font_style.scale;
            let glyph = self.font.glyph(c).scaled(scale);

            if let Some(last) = self.last_glyph {
                self.caret += self.font.pair_kerning(scale, last, glyph.id());
            }
            self.last_glyph = Some(glyph.id());
            let offset_glyph = OffsetGlyph { glyph, offset: self.caret };

            self.caret += offset_glyph.glyph.h_metrics().advance_width;
            offset_glyph
        })
    }
}