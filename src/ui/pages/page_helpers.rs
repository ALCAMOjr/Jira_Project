use ellipse::Ellipse;

/// Truncates the given text to fit within the specified width, adding an ellipsis (`...`) if necessary.
/// If the text is shorter than the specified width, it will be padded with spaces to meet the width.
pub fn get_column_string(text: &str, width: usize) -> String {
    if width == 0 {
        return ".".to_owned(); 
    }
    let truncated_text = Ellipse::truncate_ellipse(&text, width);
    if truncated_text.len() < width {
        format!("{:<width$}", truncated_text, width = width)
    } else {
        truncated_text.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    }
}
