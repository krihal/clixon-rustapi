use xmltree::Element;

fn parse_string(xmlstr: &str) -> Result<Element, String> {
    let root = match Element::parse(xmlstr.as_bytes()) {
        Ok(root) => root,
        Err(e) => return Err(e.to_string()),
    };

    Ok(root)
}
