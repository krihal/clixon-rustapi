use xmltree::Element;

pub fn parse_string(xmlstr: &str) -> Result<Element, String> {
	let root = match Element::parse(xmlstr.as_bytes()) {
		Ok(root) => root,
		Err(e) => return Err(e.to_string())
	};

	return Ok(root);
}

pub fn parse_to_string(root: &Element) -> String {
	return root.to_string();
}