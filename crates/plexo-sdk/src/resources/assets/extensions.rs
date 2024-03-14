trait AssetImplementation {
    fn alt_text(&self) -> String;
    fn icon_base64(&self) -> String;
    fn minimal_icon_base64(&self) -> String;

    fn render(&self) -> String;
}
