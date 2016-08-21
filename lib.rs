pub fn hello(name: Option<&str>) -> String {
	let n = match name{
		Some(name) => name,
		None => "World",
	};
	
	format!("Hello, {}", n)

}