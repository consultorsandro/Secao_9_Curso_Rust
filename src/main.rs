fn main() {
	//Class 139
	let mut my_array = [10,15,20,25,30];
	let my_slyce = &mut my_array[2..4];
	
	my_slyce[0] = 100;
	println!("My Slice: {:?}", my_slyce);
	println!("My Array: {:?}", my_array);
	
}
