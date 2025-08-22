use docx_rs::*;
use std::fs::File;

type DynError = Box<dyn std::error::Error>;

fn main() -> Result<(), DynError> {
    println!("Generating test documents...");

    // Create output directory if it doesn't exist
    std::fs::create_dir_all("tests/fixtures")?;

    generate_minimal_doc()?;
    generate_tables_heavy_doc()?;
    generate_headings_hierarchy_doc()?;
    generate_formatting_showcase_doc()?;
    generate_lists_comprehensive_doc()?;
    generate_business_report_doc()?;
    generate_unicode_special_doc()?;
    generate_export_test_doc()?;
    generate_retro_gaming_guide()?;

    println!("All test documents generated successfully!");
    Ok(())
}

fn generate_minimal_doc() -> Result<(), DynError> {
    let doc = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Minimal Test").bold()))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(
            "This is the smallest possible test document with just a title and one paragraph.",
        )))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text(
            "This single paragraph tests the most basic document parsing functionality.",
        )));

    let path = "tests/fixtures/minimal.docx";
    let file = File::create(path)?;
    doc.build().pack(file)?;
    println!("Generated: {path}");
    Ok(())
}

fn generate_tables_heavy_doc() -> Result<(), DynError> {
    let doc = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Tables Heavy Test Document").bold().size(24)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This document contains multiple complex tables to test table parsing capabilities.")))

        // Simple Table
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Simple Table").bold().size(16)))
        .add_table(
            Table::new(vec![
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Name").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Age").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("City").bold())),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("John"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("25"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("New York"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Jane"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("30"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Los Angeles"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Bob"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("35"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Chicago"))),
                ]),
            ])
        )

        // Financial Data Table
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Financial Data Table").bold().size(16)))
        .add_table(
            Table::new(vec![
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Quarter").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Revenue").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Expenses").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Profit").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Margin %").bold())),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Q1 2024"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$150,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$120,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$30,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("20%"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Q2 2024"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$175,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$130,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$45,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("25.7%"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Q3 2024"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$200,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$140,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$60,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("30%"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Q4 2024"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$225,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$150,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$75,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("33.3%"))),
                ]),
            ])
        )

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This document should test table header detection, CSV export, and complex table rendering.")));

    let path = "tests/fixtures/tables-heavy.docx";
    let file = File::create(path)?;
    doc.build().pack(file)?;
    println!("Generated: {path}");
    Ok(())
}

fn generate_headings_hierarchy_doc() -> Result<(), DynError> {
    let doc = Docx::new()
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Document Structure Test").bold().size(24))
                .style("Title")
        )

        // Level 1 headings
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Level 1: Introduction").bold().size(20))
                .style("Heading1")
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This document tests the heading hierarchy detection and outline generation capabilities of doxx.")))

        // Level 2 headings
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Level 2: Getting Started").bold().size(18))
                .style("Heading2")
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This section covers the basic setup and configuration.")))

        // Level 3 headings
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Level 3: Prerequisites").bold().size(16))
                .style("Heading3")
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Before you begin, ensure you have the following installed.")))

        // Level 4 headings
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Level 4: System Requirements").bold().size(14))
                .style("Heading4")
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Your system must meet these minimum requirements.")))

        // Level 5 headings
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Level 5: Hardware Specifications").bold().size(13))
                .style("Heading5")
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("The following hardware specifications are recommended.")))

        // Level 6 headings
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Level 6: Memory Requirements").bold().size(12))
                .style("Heading6")
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("At least 8GB of RAM is recommended for optimal performance.")))

        // More level 1
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Level 1: Core Features").bold().size(20))
                .style("Heading1")
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This section describes the main features of the application.")))

        // More level 2
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Level 2: Document Processing").bold().size(18))
                .style("Heading2")
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("The application can process various document formats.")))

        // More level 3
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Level 3: Supported Formats").bold().size(16))
                .style("Heading3")
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Currently supported formats include DOCX, PDF, and TXT.")));

    let path = "tests/fixtures/headings-hierarchy.docx";
    let file = File::create(path)?;
    doc.build().pack(file)?;
    println!("Generated: {path}");
    Ok(())
}

fn generate_formatting_showcase_doc() -> Result<(), DynError> {
    let doc = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Text Formatting Showcase").bold().size(24)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This document demonstrates various text formatting options to test parsing capabilities.")))

        // Bold text examples
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Bold Text Examples").bold().size(16)))
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("This paragraph contains "))
                .add_run(Run::new().add_text("bold text").bold())
                .add_run(Run::new().add_text(" mixed with regular text."))
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This entire sentence is bold.").bold()))
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Here we have "))
                .add_run(Run::new().add_text("bold at the beginning").bold())
                .add_run(Run::new().add_text(" and regular at the end."))
        )

        // Italic text examples
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Italic Text Examples").bold().size(16)))
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("This paragraph contains "))
                .add_run(Run::new().add_text("italic text").italic())
                .add_run(Run::new().add_text(" mixed with regular text."))
        )
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This entire sentence is italic.").italic()))

        // Combined formatting
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Combined Formatting").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Bold and italic combined").bold().italic()))
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("Bold with ").bold())
                .add_run(Run::new().add_text("italic inside").bold().italic())
                .add_run(Run::new().add_text(" and back to bold").bold())
        )

        // Special characters
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Special Characters and Symbols").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This section tests special characters: © ® ™ § ¶ • → ← ↑ ↓")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Math symbols: ≤ ≥ ≠ ± ∞ ∑ ∏ ∆")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Currency: $ € £ ¥ ₹")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Fractions: ½ ⅓ ¼ ¾")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Accented characters: café résumé naïve")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Greek letters: α β γ δ ε θ λ π σ ω")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This document should test comprehensive formatting detection and preservation.")));

    let path = "tests/fixtures/formatting-showcase.docx";
    let file = File::create(path)?;
    doc.build().pack(file)?;
    println!("Generated: {path}");
    Ok(())
}

fn generate_lists_comprehensive_doc() -> Result<(), DynError> {
    // Note: docx-rs doesn't have direct list support, so we'll simulate with bullet characters
    let doc = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Lists Comprehensive Test").bold().size(24)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This document tests various list types and nesting levels.")))

        // Simple unordered list
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Simple Unordered List").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• First item")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Second item")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Third item")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Fourth item")))

        // Simple ordered list
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Simple Ordered List").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("1. First numbered item")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("2. Second numbered item")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("3. Third numbered item")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("4. Fourth numbered item")))

        // Nested lists
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Nested Lists").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Top level item one")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("  • Second level item A")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("  • Second level item B")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("    • Third level item i")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("    • Third level item ii")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Top level item two")))

        // Lists with formatting
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Lists with Formatting").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Bold list item").bold()))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Italic list item").italic()))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Bold and italic list item").bold().italic()))
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("• Regular list item with "))
                .add_run(Run::new().add_text("bold words").bold())
                .add_run(Run::new().add_text(" inside"))
        )

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This document should test list detection, nesting levels, and mixed formatting within lists.")));

    let path = "tests/fixtures/lists-comprehensive.docx";
    let file = File::create(path)?;
    doc.build().pack(file)?;
    println!("Generated: {path}");
    Ok(())
}

fn generate_business_report_doc() -> Result<(), DynError> {
    let doc = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Q4 2024 Business Performance Report").bold().size(24)))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Executive Summary").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("TechCorp Inc. delivered exceptional results in Q4 2024, achieving record revenue of $2.1M and expanding our customer base by 34%. Key highlights include successful product launches, strategic partnerships, and improved operational efficiency.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Key Performance Indicators").bold().size(16)))
        .add_table(
            Table::new(vec![
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Metric").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Q4 2024").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Q3 2024").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Change").bold())),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Revenue"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$2,100,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$1,750,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("+20%"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Net Profit"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$420,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$315,000"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("+33%"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Customer Acquisition"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("450"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("335"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("+34%"))),
                ]),
            ])
        )

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Financial Performance").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Revenue Analysis").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Our revenue growth of 20% quarter-over-quarter demonstrates strong market demand for our products. The primary growth drivers include:")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Enterprise Sales").bold()).add_run(Run::new().add_text(": $850,000 (+45% from Q3)")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Subscription Revenue").bold()).add_run(Run::new().add_text(": $720,000 (+15% from Q3)")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Professional Services").bold()).add_run(Run::new().add_text(": $380,000 (+8% from Q3)")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Product Sales").bold()).add_run(Run::new().add_text(": $150,000 (-5% from Q3)")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Risks and Challenges").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Market Risks").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Increased Competition").bold()).add_run(Run::new().add_text(": Three new competitors entered the market")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Economic Uncertainty").bold()).add_run(Run::new().add_text(": Potential recession could impact enterprise spending")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Regulatory Changes").bold()).add_run(Run::new().add_text(": New data privacy regulations may require compliance investments")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Strategic Initiatives").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("2025 Objectives").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Revenue Target").bold()).add_run(Run::new().add_text(": $12M (400% growth)")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Market Expansion").bold()).add_run(Run::new().add_text(": Enter European and Asian markets")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Product Portfolio").bold()).add_run(Run::new().add_text(": Launch 4 new products")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Team Growth").bold()).add_run(Run::new().add_text(": Scale to 150 employees")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Key Action Items").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Q1 2025").bold()).add_run(Run::new().add_text(": Complete Series A funding round ($5M target)")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Q2 2025").bold()).add_run(Run::new().add_text(": Open European office in London")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Q3 2025").bold()).add_run(Run::new().add_text(": Launch enterprise mobile application")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• ")).add_run(Run::new().add_text("Q4 2025").bold()).add_run(Run::new().add_text(": Achieve SOC 2 Type II compliance")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Conclusion").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Q4 2024 represents a transformative quarter for TechCorp. We've demonstrated strong execution across all business functions while positioning ourselves for accelerated growth in 2025.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Next Review").bold()).add_run(Run::new().add_text(": February 15, 2025")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Prepared by").bold()).add_run(Run::new().add_text(": Strategic Planning Team")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Date").bold()).add_run(Run::new().add_text(": January 10, 2025")));

    let path = "tests/fixtures/business-report.docx";
    let file = File::create(path)?;
    doc.build().pack(file)?;
    println!("Generated: {path}");
    Ok(())
}

fn generate_unicode_special_doc() -> Result<(), DynError> {
    let doc = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Unicode and Special Characters Test").bold().size(24)))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("International Text").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("English: Hello, World!")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Spanish: ¡Hola, Mundo!")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("French: Bonjour, le Monde!")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("German: Hallo, Welt!")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Russian: Привет, мир!")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Chinese: 你好，世界！")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Japanese: こんにちは、世界！")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Korean: 안녕하세요, 세계!")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Arabic: مرحبا بالعالم!")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hindi: नमस्ते, दुनिया!")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hebrew: שלום, עולם!")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Greek: Γεια σου, κόσμε!")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Emoji and Symbols").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Basic emojis: 😀 😃 😄 😁 😆 😅 😂 🤣")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Hearts: ❤️ 💙 💚 💛 💜 🖤 🤍 🤎")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Animals: 🐶 🐱 🐭 🐹 🐰 🦊 🐻 🐼")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Food: 🍎 🍌 🍊 🍋 🍉 🍇 🍓 🥝")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Weather: ☀️ ⛅ ☁️ 🌧️ ⛈️ 🌩️ ❄️ ⭐")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Mathematical Symbols").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Basic operators: + - × ÷ = ≠ < > ≤ ≥")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Greek letters: α β γ δ ε ζ η θ ι κ λ μ ν ξ ο π ρ σ τ υ φ χ ψ ω")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Set theory: ∈ ∉ ⊂ ⊃ ⊆ ⊇ ∩ ∪ ∅")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Calculus: ∫ ∮ ∂ ∇ ∆ ∑ ∏ ∞")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Currency Symbols").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Major currencies: $ € £ ¥ ₹ ₽ ₩ ₪ ₫ ₡ ₦ ₨ ₱ ₲ ₴ ₵")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Fractions and Numbers").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Fractions: ½ ⅓ ¼ ¾ ⅕ ⅖ ⅗ ⅘ ⅙ ⅚ ⅛ ⅜ ⅝ ⅞")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Superscripts: x² x³ x⁴ x⁵ x⁶ x⁷ x⁸ x⁹ x¹⁰")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Subscripts: H₂O CO₂ H₂SO₄ CaCl₂")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Arrows and Shapes").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Arrows: ← → ↑ ↓ ↖ ↗ ↘ ↙ ⟵ ⟶ ⟷ ⤴ ⤵")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Shapes: ○ ● ◯ ◉ □ ■ ▢ ▣ △ ▲ ▼ ◆ ◇ ★ ☆")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This document tests Unicode handling, special character rendering, and international text support.")));

    let path = "tests/fixtures/unicode-special.docx";
    let file = File::create(path)?;
    doc.build().pack(file)?;
    println!("Generated: {path}");
    Ok(())
}

fn generate_export_test_doc() -> Result<(), DynError> {
    let doc = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Export Test Document").bold().size(24)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This document is designed specifically to test all export formats and features.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Text Formatting Test").bold().size(18)))
        .add_paragraph(
            Paragraph::new()
                .add_run(Run::new().add_text("This paragraph contains "))
                .add_run(Run::new().add_text("bold").bold())
                .add_run(Run::new().add_text(", "))
                .add_run(Run::new().add_text("italic").italic())
                .add_run(Run::new().add_text(", and "))
                .add_run(Run::new().add_text("bold italic").bold().italic())
                .add_run(Run::new().add_text(" text."))
        )

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Simple Table for CSV Export").bold().size(18)))
        .add_table(
            Table::new(vec![
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Product").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Price").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Stock").bold())),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Widget A"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$19.99"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("150"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Widget B"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("$29.99"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("75"))),
                ]),
            ])
        )

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("List Test for Markdown").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• First bullet point")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Second bullet point with ")).add_run(Run::new().add_text("bold text").bold()))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Third bullet point")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Numbered List").bold().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("1. First numbered item")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("2. Second numbered item")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("3. Third numbered item")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Special Characters for JSON").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Testing quotes: \"double quotes\" and 'single quotes'")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Testing backslashes: \\ and forward slashes: /")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Testing newlines and tabs in export")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("This document validates that all export formats (markdown, text, CSV, JSON) work correctly with various content types.")));

    let path = "tests/fixtures/export-test.docx";
    let file = File::create(path)?;
    doc.build().pack(file)?;
    println!("Generated: {path}");
    Ok(())
}

fn generate_retro_gaming_guide() -> Result<(), DynError> {
    let doc = Docx::new()
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("🚀 RETRO GAMING GUIDE 🎮").bold().size(28)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("The Ultimate Journey Through Gaming History").italic().size(16)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("A comprehensive guide to the golden age of video games").size(12)))

        // Table of Contents with numbering
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Table of Contents").bold().size(20)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("1. The Dawn of Gaming (1970s)").size(14)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("2. The Arcade Revolution (1980s)").size(14)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("3. Console Wars Begin (1985-1995)").size(14)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("4. Legendary Games That Changed Everything").size(14)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("5. Modern Retro: The Revival").size(14)))

        // Chapter 1
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("1. The Dawn of Gaming (1970s)").bold().size(22)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("In the beginning, there was Pong. The year was 1972, and Atari had just launched what would become the foundation of an entire industry. This simple game of electronic tennis would spark a revolution that continues to this day.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("1.1 Pong: The Genesis").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Created by Allan Alcorn at Atari, Pong was deceptively simple: two paddles, one ball, and endless hours of entertainment. The game's success was immediate and profound.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Key Features:").bold()))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Two-player competitive gameplay")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Simple controls: up and down")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Increasing ball speed for difficulty")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Distinctive 'beep' sound effects")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("1.2 Space Invaders: The Phenomenon").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Released in 1978 by Taito, Space Invaders created the template for the shoot-'em-up genre. Players controlled a cannon at the bottom of the screen, defending Earth from descending alien invaders.")))

        // Chapter 2
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("2. The Arcade Revolution (1980s)").bold().size(22)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("The 1980s brought us into the golden age of arcade gaming. Dimly lit rooms filled with the sounds of electronic music, coin drops, and joystick clicks became cultural epicenters.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("2.1 Pac-Man: The Cultural Icon").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Released by Namco in 1980, Pac-Man wasn't just a game—it was a cultural phenomenon. The yellow circle with a voracious appetite became one of the most recognizable characters in gaming history.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Game Mechanics:").bold()))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Navigate mazes while eating dots")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Avoid four uniquely behaved ghosts:")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("  • Blinky (red): Aggressive pursuer")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("  • Pinky (pink): Ambush predator")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("  • Inky (cyan): Unpredictable behavior")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("  • Clyde (orange): Shy and random")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Power pellets temporarily reverse the chase")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Bonus fruits for extra points")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("2.2 Donkey Kong: Nintendo's Breakthrough").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("In 1981, a young designer named Shigeru Miyamoto created Donkey Kong for Nintendo. This platformer introduced the world to Mario (originally called Jumpman) and established Nintendo as a major player in the arcade scene.")))

        // Chapter 3
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("3. Console Wars Begin (1985-1995)").bold().size(22)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("The mid-1980s marked the transition from arcade dominance to home console supremacy. The Nintendo Entertainment System (NES) launched in North America in 1985, revitalizing the video game industry after the crash of 1983.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("3.1 Nintendo vs. Sega: The Epic Battle").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("The console wars of the late 80s and early 90s were defined by two titans: Nintendo and Sega. Each company had its mascot, its philosophy, and its passionate fanbase.")))

        // Comparison Table
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Console Comparison").bold().size(16)))
        .add_table(
            Table::new(vec![
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Feature").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Nintendo NES").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Sega Genesis").bold())),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("CPU"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("6502 @ 1.79 MHz"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("68000 @ 7.6 MHz"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Graphics"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("2C02 PPU"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("VDP @ 256x224"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Colors"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("54 total, 25 on screen"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("512 total, 61 on screen"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Mascot"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Mario 🍄"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Sonic 💨"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Launch Year"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("1985 (US)"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("1989 (US)"))),
                ]),
            ])
        )

        // Chapter 4
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("4. Legendary Games That Changed Everything").bold().size(22)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Certain games transcended entertainment to become cultural touchstones. These titles didn't just entertain—they redefined what video games could be.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("4.1 Super Mario Bros. (1985)").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Shigeru Miyamoto's masterpiece established the platformer genre and saved the video game industry in North America. Its precise controls, memorable music, and imaginative level design remain influential today.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Revolutionary Features:").bold()))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Precise character physics and momentum")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Iconic power-up system (mushroom, fire flower)")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Memorable soundtrack by Koji Kondo")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Hidden secrets and warp zones")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Perfect difficulty curve")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("4.2 The Legend of Zelda (1986)").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Nintendo's adventure epic introduced open-world exploration, battery-backed save files, and a level of freedom unprecedented in console gaming.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("4.3 Sonic the Hedgehog (1991)").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Sega's answer to Mario brought speed, attitude, and technical prowess to the platformer genre. Sonic's fluid animation and breakneck pace showcased the Genesis hardware.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Speed Mechanics:").bold()))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Momentum-based physics system")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Loop-de-loops and corkscrews")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Ring collection system")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Special Stage bonus rounds")))

        // Chapter 5
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("5. Modern Retro: The Revival").bold().size(22)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("The 2010s brought an unexpected renaissance of retro gaming. Indie developers embraced pixel art aesthetics, major publishers released mini consoles, and speedrunning communities celebrated the technical mastery of classic games.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("5.1 The Indie Revolution").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Modern indie games like Shovel Knight, Celeste, and Hollow Knight proved that retro aesthetics could house cutting-edge game design. These titles combined nostalgic visuals with contemporary mechanical sophistication.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("5.2 Speedrunning Culture").bold().size(18)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("The pursuit of perfect game completion times has elevated classic games to new heights. Communities have discovered glitches, optimizations, and techniques that push 30-year-old games to their absolute limits.")))

        // Famous Speedrun Times Table
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Famous Speedrun Records").bold().size(16)))
        .add_table(
            Table::new(vec![
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Game").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Category").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("World Record").bold())),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Technique").bold())),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Super Mario Bros."))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Any%"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("4:54.798"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Frame-perfect inputs"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("The Legend of Zelda"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Any%"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("29:36"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Screen wrapping glitches"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Sonic the Hedgehog"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Any%"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("14:28"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Zip glitches"))),
                ]),
                TableRow::new(vec![
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Pac-Man"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Perfect Game"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("3,333,360 pts"))),
                    TableCell::new().add_paragraph(Paragraph::new().add_run(Run::new().add_text("Pattern memorization"))),
                ]),
            ])
        )

        // Conclusion
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Conclusion: The Eternal Appeal").bold().size(20)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Retro games endure because they represent pure game design distilled to its essence. Without the crutch of photorealistic graphics or complex narratives, these games succeed through tight controls, clever mechanics, and that indefinable quality we call 'fun'.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("The influence of these pioneering titles can be seen in every modern game. From the precise platforming of Mario to the pattern-based challenges of Space Invaders, the DNA of retro gaming lives on in today's most innovative titles.")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Game Preservation Resources:").bold()))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Internet Archive's Console Living Room")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Video Game History Foundation")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• RetroGaming subreddit community")))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("• Local retro gaming meetups and conventions")))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("---").italic()))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("\"The best games are timeless. They transcend the technology that created them.\"").italic()))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("— Shigeru Miyamoto").italic().size(10)))

        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("Document created with doxx 📄✨").size(10)))
        .add_paragraph(Paragraph::new().add_run(Run::new().add_text("For testing terminal document viewing with style 🎮🚀").size(10)));

    let path = "tests/fixtures/retro-gaming-guide.docx";
    let file = File::create(path)?;
    doc.build().pack(file)?;
    println!("Generated: {path}");
    Ok(())
}
