pub fn index_html() -> &'static str {
    include_str!("../static/index.html")
}

#[cfg(test)]
mod tests {
  use super::index_html;

  #[test]
  fn ui_html_contains_key_elements() {
    let html = index_html();

    assert!(html.contains("id=\"shareUrl\""));
    assert!(html.contains("id=\"joinBtn\""));
    assert!(html.contains("id=\"nameInput\""));
    assert!(html.contains("id=\"appPanel\""));
    assert!(html.contains("id=\"themeSelect\""));
    assert!(html.contains("id=\"copyShareBtn\""));
    assert!(html.contains("id=\"summaryStatus\""));
    assert!(html.contains("/ws?name=${encodeURIComponent(name)}"));
  }

  #[test]
  fn ui_html_does_not_contain_escaped_quotes() {
    let html = index_html();

    assert!(
      !html.contains("\\\""),
      "ui template contains escaped quotes that can break rendered HTML"
    );
  }
}
