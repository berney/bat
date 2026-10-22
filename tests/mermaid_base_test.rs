use bat::mermaid::canvas::{Canvas, DrawingCoord};
use bat::mermaid::text::{char_width, display_width};
use bat::mermaid::{detect_diagram_kind, DiagramKind};

#[test]
fn test_detect_diagram_kind() {
    assert_eq!(
        detect_diagram_kind("graph TD\n  A --> B"),
        DiagramKind::Flowchart
    );
    assert_eq!(
        detect_diagram_kind("flowchart LR\n  A --> B"),
        DiagramKind::Flowchart
    );
    assert_eq!(
        detect_diagram_kind("sequenceDiagram\n  Alice->>Bob: Hello"),
        DiagramKind::Sequence
    );
    assert_eq!(
        detect_diagram_kind("classDiagram\n  class Animal"),
        DiagramKind::Class
    );
    assert_eq!(
        detect_diagram_kind("erDiagram\n  CUSTOMER ||--o{ ORDER : places"),
        DiagramKind::Er
    );
    assert_eq!(
        detect_diagram_kind("xychart-beta\n  x-axis [A, B]"),
        DiagramKind::XyChart
    );
}

#[test]
fn test_display_width() {
    assert_eq!(display_width("hello"), 5);
    assert_eq!(display_width("你好"), 4);
    assert_eq!(display_width("┌───┐"), 5);
    assert_eq!(char_width('a'), 1);
    assert_eq!(char_width('你'), 2);
}

#[test]
fn test_canvas_draw_and_render() {
    let mut canvas = Canvas::new(5, 3);
    canvas.draw_text(DrawingCoord::new(0, 0), "hello", true);
    canvas.draw_text(DrawingCoord::new(0, 1), "world", true);
    let output = canvas.to_plain_string();
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0].trim_end(), "hello");
    assert_eq!(lines[1].trim_end(), "world");
}
