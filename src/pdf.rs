//! Create a PDF of a Receipt

use crate::sale::Sale;
use anyhow::{Context, Result};
use log::{debug, warn};
use printpdf::PaintMode::{Fill, FillStroke};
use printpdf::*;
use std::path::Path;

pub fn print_pdf(sale: &Sale, output_path: &Path) -> Result<()> {
    /// A4 portrait dimensions in mm
    const PAGE_H: f32 = 297.0;
    const PAGE_W: f32 = 210.0;

    let padding_right = 10.0;
    let header_y = 277.0;
    let font_height = 15.0;

    let mut doc = PdfDocument::new("Iced Receipt");
    let ops = vec![
        // A4 Portrait in Points is 595.0 wide (x) and 842.0 high (y)
        // Draw the rectangle with our background colour
        Op::SetFillColor {
            col: Color::Rgb(Rgb {
                r: 0.929,
                g: 0.929,
                b: 0.929,
                icc_profile: None,
            }),
        },
        Op::DrawPolygon {
            polygon: Polygon {
                rings: vec![PolygonRing {
                    points: vec![
                        LinePoint {
                            p: Point {
                                x: Pt(padding_right),
                                y: Pt(812.0),
                            },
                            bezier: false,
                        },
                        LinePoint {
                            p: Point {
                                x: Pt(585.0),
                                y: Pt(812.0),
                            },
                            bezier: false,
                        },
                        LinePoint {
                            p: Point {
                                x: Pt(585.0),
                                y: Pt(190.0),
                            },
                            bezier: false,
                        },
                        LinePoint {
                            p: Point {
                                x: Pt(padding_right),
                                y: Pt(190.0),
                            },
                            bezier: false,
                        },
                        LinePoint {
                            p: Point {
                                x: Pt(padding_right),
                                y: Pt(812.0),
                            },
                            bezier: false,
                        },
                    ],
                }],
                mode: Fill,
                winding_order: WindingOrder::NonZero,
            },
        },
        // Save the graphics state to allow for position resets later
        Op::SaveGraphicsState,
        // Start a text section (required for text operations)
        Op::StartTextSection,
        Op::SetTextCursor {
            pos: Point::new(Mm(padding_right), Mm(header_y)),
        },
        Op::SetLineHeight {
            lh: Pt(font_height),
        },
        Op::SetFont {
            font: PdfFontHandle::Builtin(BuiltinFont::Helvetica),
            size: Pt(font_height),
        },
        // Set the text colour
        Op::SetFillColor {
            col: Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)),
        },
        Op::ShowText {
            items: vec![TextItem::Text(String::from("Item Name"))],
        },
        Op::EndTextSection,
        Op::StartTextSection,
        Op::SetTextCursor {
            pos: Point::new(Mm(100.0), Mm(header_y)),
        },
        Op::SetLineHeight {
            lh: Pt(font_height),
        },
        Op::SetFont {
            font: PdfFontHandle::Builtin(BuiltinFont::Helvetica),
            size: Pt(font_height),
        },
        // Set the text colour
        Op::SetFillColor {
            col: Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)),
        },
        Op::ShowText {
            items: vec![TextItem::Text(String::from("Qty"))],
        },
        Op::EndTextSection,
        Op::StartTextSection,
        Op::SetTextCursor {
            pos: Point::new(Mm(125.0), Mm(header_y)),
        },
        Op::SetLineHeight {
            lh: Pt(font_height),
        },
        Op::SetFont {
            font: PdfFontHandle::Builtin(BuiltinFont::Helvetica),
            size: Pt(font_height),
        },
        // Set the text colour
        Op::SetFillColor {
            col: Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)),
        },
        Op::ShowText {
            items: vec![TextItem::Text(String::from("Price Tax Group"))],
        },
        Op::EndTextSection,
        Op::StartTextSection,
        Op::SetTextCursor {
            pos: Point::new(Mm(180.0), Mm(header_y)),
        },
        Op::SetLineHeight {
            lh: Pt(font_height),
        },
        Op::SetFont {
            font: PdfFontHandle::Builtin(BuiltinFont::Helvetica),
            size: Pt(font_height),
        },
        // Set the text colour
        Op::SetFillColor {
            col: Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)),
        },
        Op::ShowText {
            items: vec![TextItem::Text(String::from("Total"))],
        },
        Op::EndTextSection,
        Op::StartTextSection,
        Op::SetTextCursor {
            pos: Point::new(Mm(padding_right), Mm(50.0)),
        },
        Op::SetLineHeight {
            lh: Pt(font_height),
        },
        Op::SetFont {
            font: PdfFontHandle::Builtin(BuiltinFont::Helvetica),
            size: Pt(font_height),
        },
        // A4 Portrait in Points is 595.0 wide (x) and 842.0 high (y)
        // Draw the rectangle with our background colour
        Op::SetFillColor {
            col: Color::Rgb(Rgb {
                r: 0.929,
                g: 0.929,
                b: 0.929,
                icc_profile: None,
            }),
        },
        Op::DrawPolygon {
            polygon: Polygon {
                rings: vec![PolygonRing {
                    points: vec![
                        LinePoint {
                            p: Point {
                                x: Pt(padding_right),
                                y: Pt(170.0),
                            },
                            bezier: false,
                        },
                        LinePoint {
                            p: Point {
                                x: Pt(585.0),
                                y: Pt(170.0),
                            },
                            bezier: false,
                        },
                        LinePoint {
                            p: Point {
                                x: Pt(585.0),
                                y: Pt(padding_right + 5.0),
                            },
                            bezier: false,
                        },
                        LinePoint {
                            p: Point {
                                x: Pt(padding_right),
                                y: Pt(padding_right + 5.0),
                            },
                            bezier: false,
                        },
                        LinePoint {
                            p: Point {
                                x: Pt(padding_right),
                                y: Pt(170.0),
                            },
                            bezier: false,
                        },
                    ],
                }],
                mode: Fill,
                winding_order: WindingOrder::NonZero,
            },
        },
        // Set the text colour and write over the top of the rectangle
        Op::SetFillColor {
            col: Color::Rgb(Rgb::new(0.0, 0.0, 0.0, None)),
        },
        Op::ShowText {
            items: vec![TextItem::Text(String::from("Subtotal"))],
        },
        Op::AddLineBreak,
        Op::AddLineBreak,
        Op::ShowText {
            items: vec![TextItem::Text(String::from("Service Charge"))],
        },
        Op::AddLineBreak,
        Op::AddLineBreak,
        Op::ShowText {
            items: vec![TextItem::Text(String::from("Tax"))],
        },
        Op::AddLineBreak,
        Op::AddLineBreak,
        Op::ShowText {
            items: vec![TextItem::Text(String::from("Gratuity"))],
        },
        Op::AddLineBreak,
        Op::AddLineBreak,
        Op::ShowText {
            items: vec![TextItem::Text(String::from("Total"))],
        },
        Op::EndTextSection,
        Op::RestoreGraphicsState,
    ];

    let page = PdfPage::new(Mm(PAGE_W), Mm(PAGE_H), ops);
    doc.with_pages(vec![page]);

    let mut save_warnings = Vec::new();
    let pdf_bytes = doc.save(&PdfSaveOptions::default(), &mut save_warnings);

    for warning in save_warnings {
        warn!("PDF save warning: {warning:?}");
    }

    std::fs::write(output_path, pdf_bytes)
        .with_context(|| format!("Cannot write PDF to {output_path:?}"))?;

    debug!("PDF saved to {output_path:?}");
    Ok(())
}
