// Copyright 2024 Sebastian Wiesner <sebastian@swsnr.de>

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use mdcat::stats::DocumentStats;

#[test]
fn test_document_stats() {
    let content = "# Test Document\n\nThis is a **test** document with [a link](http://example.com).\n\n```rust\nfn main() {\n    println!(\"Hello, world!\");\n}\n```\n\n## Another heading\n\n- List item 1\n- List item 2\n\n1. Ordered item 1\n2. Ordered item 2";
    
    let stats = DocumentStats::from_markdown(content);
    
    // Test basic counts
    assert!(stats.character_count > 0);
    assert!(stats.word_count > 0);
    assert!(stats.line_count > 0);
    
    // Test structural elements
    assert_eq!(stats.heading_count, 2); // # Test Document and ## Another heading
    assert_eq!(stats.code_block_count, 1); // The rust code block
    assert_eq!(stats.link_count, 1); // The link
    assert_eq!(stats.image_count, 0); // No images
    assert_eq!(stats.list_count, 2); // One unordered list, one ordered list
    assert_eq!(stats.table_count, 0); // No tables
    
    // Test reading time calculation
    let reading_time = stats.reading_time_minutes();
    assert!(reading_time > 0);
    
    // Test formatting
    let formatted = stats.format();
    assert!(formatted.contains("Document Statistics:"));
    assert!(formatted.contains("Characters:"));
    assert!(formatted.contains("Words:"));
    assert!(formatted.contains("Lines:"));
    assert!(formatted.contains("Headings:"));
    assert!(formatted.contains("Code blocks:"));
    assert!(formatted.contains("Links:"));
    assert!(formatted.contains("Images:"));
    assert!(formatted.contains("Lists:"));
    assert!(formatted.contains("Tables:"));
    assert!(formatted.contains("Estimated reading time:"));
}

#[test]
fn test_line_number_formatter() {
    use mdcat::stats::LineNumberFormatter;
    use std::io::Write;
    
    let mut formatter = LineNumberFormatter::new(true, 100);
    let mut output = Vec::new();
    
    // Test line number writing
    formatter.write_line_number(&mut output).unwrap();
    write!(&mut output, "test line").unwrap();
    formatter.write_newline(&mut output).unwrap();
    
    formatter.write_line_number(&mut output).unwrap();
    write!(&mut output, "another line").unwrap();
    formatter.write_newline(&mut output).unwrap();
    
    let result = String::from_utf8_lossy(&output);
    assert!(result.contains("1 │ test line"));
    assert!(result.contains("2 │ another line"));
    
    // Test current line tracking
    assert_eq!(formatter.current_line(), 2);
}

#[test]
fn test_line_number_formatter_disabled() {
    use mdcat::stats::LineNumberFormatter;
    use std::io::Write;
    
    let mut formatter = LineNumberFormatter::new(false, 100);
    let mut output = Vec::new();
    
    // Test that no line numbers are added when disabled
    formatter.write_line_number(&mut output).unwrap();
    write!(&mut output, "test line").unwrap();
    formatter.write_newline(&mut output).unwrap();
    
    let result = String::from_utf8_lossy(&output);
    assert!(!result.contains("│"));
    assert!(result.contains("test line"));
} 