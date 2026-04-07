//! Create a PDF of a Receipt

use crate::sale::Sale;
use anyhow::{Context, Result};
use log::{debug, warn};
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
        // Save the graphics state to allow for position resets later
        Op::SaveGraphicsState,
        // Start a text section (required for text operations)
        Op::StartTextSection,
        Op::SetTextCursor {
            pos: Point::new(Mm(padding_right), Mm(header_y)),
        },
        Op::SetLineHeight { lh: Pt(font_height) },
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
        Op::SetLineHeight { lh: Pt(font_height) },
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
        Op::SetLineHeight { lh: Pt(font_height) },
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
        Op::SetLineHeight { lh: Pt(font_height) },
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
        Op::SetLineHeight { lh: Pt(font_height) },
        Op::SetFont {
            font: PdfFontHandle::Builtin(BuiltinFont::Helvetica),
            size: Pt(font_height),
        },
        // Set the text colour
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
        // A4 Portrait in Points is 595.0 wide (x) and 842.0 high (y)
        // A rectangle
        Op::DrawLine {
            line: Line {
                points: vec![
                    LinePoint {
                        p: Point {
                            x: Pt(padding_right),
                            y: Pt(160.0),
                        },
                        bezier: false,
                    },
                    LinePoint {
                        p: Point {
                            x: Pt(585.0),
                            y: Pt(160.0),
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
                            y: Pt(160.0),
                        },
                        bezier: false,
                    },
                ],
                is_closed: false,
            },
        },
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
