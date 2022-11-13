use std::slice::from_raw_parts_mut;
use std::fs;

const MODEL_SIG: u32 = 0x4D4F444C;
const MODEL_EXT: &str = "mdl";

#[derive(Debug)]
enum DERR
{
	WrongSignature
}

pub struct ModelDescriptor<'a>
{
	sig: &'a u32,

	vert_n: &'a u32,
	indx_n: &'a u32,

	verts: &'a [f32],
	indcs: &'a [u32],
}
impl ModelDescriptor<'_>
{
	fn build(file: &mut [u8]) -> Result<ModelDescriptor<'static>, DERR>
	{
		unsafe
		{
		// Setup some stuff
		let len = file.len() as isize;
		let buf: *mut u8 = &mut file[0];

		// Validate signature
		if *(buf as *const u32) == MODEL_SIG
		{
			return Err(DERR::WrongSignature);
		}

		// Read offsets
		let o_vert_n		= *(buf.offset(len - (4 * (1))) as *mut u32) as isize;
		let o_indx_n		= *(buf.offset(len - (4 * (2))) as *mut u32) as isize;
		let o_verts		= *(buf.offset(len - (4 * (3))) as *mut u32) as isize;
		let o_indcs		= *(buf.offset(len - (4 * (4))) as *mut u32) as isize;

		// Make arrays
		let vertices = from_raw_parts_mut(
			buf.offset(o_verts) as *mut f32,
			o_vert_n as usize);
		let indices = from_raw_parts_mut(
			buf.offset(o_indcs) as *mut u32,
			o_indx_n as usize);

		Ok(ModelDescriptor
		{
			sig: 	&*(buf as *mut u32),

			vert_n: &*(buf.offset(o_vert_n) as *mut u32),
			indx_n: &*(buf.offset(o_indx_n) as *mut u32),

			verts: 	vertices,
			indcs: 	indices,
		})
		}
	}
}

#[cfg(test)]
mod tests {
	use std::env;

use super::*;

	#[test]
	fn it_works()
	{
		let mut vc = fs::read("./exampleFile.dvl.mdl").
			expect("Could not read file.");
			
		let arr: &mut [u8];
		unsafe
		{
			let vecptr: *mut u8 = &mut vc[0];
			arr = from_raw_parts_mut(vecptr, vc.len());
		}

		let mdl = ModelDescriptor::build(arr).
			expect("File signature was wrong");
		
		println!("{0}", mdl.sig);
		println!("(");
		println!("	Vertex Count: {0}", mdl.vert_n);
		println!("	Index Count: {0}", mdl.indx_n);
		println!("	(");
		println!("		Vertices:");
		println!("		[");
		for i in 0usize..*mdl.vert_n as usize
		{
			println!("			{0}", mdl.verts[i]);
		}
		println!("		]");
		println!("		Indices:");
		println!("		[");
		for i in 0usize..*mdl.indx_n as usize
		{
			println!("			{0}", mdl.indcs[i]);
		}
		println!("		]");
		println!("	)");
		println!(")");

		panic!();	// Force the test to fail to look at the output
	}
}
