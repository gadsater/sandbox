use pulldown_cmark::{html, Parser};

/// Render a markdown string to an HTML fragment.
pub fn render_markdown(input: &str) -> String {
    let parser = Parser::new(input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_heading() {
        let html = render_markdown("# Hello");
        assert!(html.contains("<h1>"));
        assert!(html.contains("Hello"));
    }

    #[test]
    fn renders_bold() {
        let html = render_markdown("**bold**");
        assert!(html.contains("<strong>bold</strong>"));
    }
}
