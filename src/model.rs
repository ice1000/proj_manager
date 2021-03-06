use std::collections::BTreeMap;

type FieldIndex = (usize, usize);

#[allow(dead_code)]
pub struct Config {
	str_fields: Vec<String>,
	vac_fields: Vec<Vec<String>>,
	ui8_fields: Vec<u8>,

	field_map: BTreeMap<String, FieldIndex>,
}

#[allow(dead_code)]
impl Config {
	pub fn new(
		proj_name: String,
		path: String,
		ignored: Vec<String>,
		ignored_suffix: Vec<String>,
		build: Vec<String>,
		indent_line_1: u8,
		indent_line_2: u8,
		indent_line_3: u8,
		indent_ls_1: u8
	) -> Config {
		Config {
			str_fields: vec![proj_name, path],
			vac_fields: vec![ignored, ignored_suffix, build],
			ui8_fields: vec![indent_ls_1, indent_line_1, indent_line_2, indent_line_3],
			field_map: Config::field_map(),
		}
	}

	pub fn default() -> Config {
		Config::new(
			String::from("Unknown project name"),
			String::new(),
			Vec::new(),
			Vec::new(),
			Vec::new(),
			32,
			32,
			4,
			4
		)
	}

	pub fn set_fields(&mut self, name: &str, item: &str) -> bool {
		if let Some(field_index) = self.field_map.get(name) {
			let &(main_index, sub_index) = field_index;
			if main_index == 0 {
				self.str_fields[sub_index] = item.to_string();
			} else if main_index == 1 {
				self.vac_fields[sub_index].push(item.to_string());
			} else if main_index == 2 {
				if let Ok(value) = item.parse() {
					self.ui8_fields[sub_index] = value;
				}
			} else {
				panic!("unexpected error")
			}
			true
		} else {
			false
		}
	}

	pub fn field_map() -> BTreeMap<String, FieldIndex> {
		let mut result: BTreeMap<String, FieldIndex> = BTreeMap::new();
		result.insert("name".to_string(), (0, 0));
		result.insert("path".to_string(), (0, 1));

		result.insert("ign".to_string(), (1, 0));
		result.insert("ign-sfx".to_string(), (1, 1));
		result.insert("build".to_string(), (1, 2));

		result.insert("idt-ls-1".to_string(), (2, 0));
		result.insert("idt-line-1".to_string(), (2, 1));
		result.insert("idt-line-2".to_string(), (2, 2));
		result.insert("idt-line-3".to_string(), (2, 3));

		result
	}

	pub fn proj_name(&self) -> String {
		self.str_fields[0].clone()
	}

	pub fn path(&self) -> String {
		self.str_fields[1].clone()
	}

	pub fn ignored(&self) -> Vec<String> {
		self.vac_fields[0].clone()
	}

	pub fn ignored_suffix(&self) -> Vec<String> {
		self.vac_fields[1].clone()
	}

	pub fn build(&self) -> Vec<String> {
		self.vac_fields[2].clone()
	}

	pub fn indent_ls_1(&self) -> u8 {
		self.ui8_fields[0].clone()
	}

	pub fn indent_line_1(&self) -> u8 {
		self.ui8_fields[1].clone()
	}

	pub fn indent_line_2(&self) -> u8 {
		self.ui8_fields[2].clone()
	}

	pub fn indent_line_3(&self) -> u8 {
		self.ui8_fields[3].clone()
	}

	pub fn is_ignored(&self, name: &String) -> bool {
		self.ignored().contains(name)
	}

	pub fn is_ignored_suffix(&self, name: &String) -> bool {
		self.ignored_suffix().iter().any(|x| name.ends_with(x))
	}
}
