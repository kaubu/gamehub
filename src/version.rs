pub struct Version {
	pub major: i32,
	pub minor: i32,
	pub patch: i32,
}

impl Version {
	pub const fn new(maj: i32, min: i32, pat: i32) -> Version {
		Version {
			major: maj,
			minor: min,
			patch: pat,
		}
	}
	pub fn display(&self) -> String {
		format!("{}.{}.{}", self.major, self.minor, self.patch)
	}
}