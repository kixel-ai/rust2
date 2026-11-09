//! The programs `#[wasm_bindgen]` encodes, decoded as the wasm-bindgen CLI decodes them.
//!
//! Vendored from wasm-bindgen's `crates/cli-support/src/decode.rs` at 0.2.130
//! (c93b8ae848d4d9e6beaeaf9516e18a7cadd818a2), MIT OR Apache-2.0, whose decoder is
//! private. Its field list is `shared_api!`'s from `crates/shared/src/lib.rs` at that
//! revision, with each field another supported schema lacks gated by the schema that
//! added it. Decoding is positional, so a schema is supported only once its gates are
//! here. A malformed program is an error, not a panic.

use std::str;

/// A schema whose programs decode here, by its `SCHEMA_VERSION`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Schema {
	/// wasm-bindgen 0.2.128 and 0.2.129.
	V0_2_128,
	/// wasm-bindgen 0.2.130.
	V0_2_130,
}

impl Schema {
	/// Every supported schema, oldest first.
	pub const ALL: [Schema; 2] = [Schema::V0_2_128, Schema::V0_2_130];

	pub fn of(version: &str) -> Option<Self> {
		Self::ALL.into_iter().find(|s| s.version() == version)
	}

	pub fn version(self) -> &'static str {
		match self {
			Schema::V0_2_128 => "0.2.128",
			Schema::V0_2_130 => "0.2.130",
		}
	}
}

/// Why a program didn't decode.
pub type Error = String;

/// The bytes of a program left to decode, in its schema.
pub struct Data<'src> {
	bytes: &'src [u8],
	schema: Schema,
}

impl<'src> Data<'src> {
	fn byte(&mut self) -> Result<u8, Error> {
		let (&b, rest) = self.bytes.split_first().ok_or("a program ends early")?;
		self.bytes = rest;
		Ok(b)
	}
}

pub trait Decode<'src>: Sized {
	fn decode(data: &mut Data<'src>) -> Result<Self, Error>;
}

/// Decode `bytes`, a whole program of `schema`.
pub fn program(bytes: &[u8], schema: Schema) -> Result<Program<'_>, Error> {
	let mut data = Data { bytes, schema };
	let program = Program::decode(&mut data)?;
	match data.bytes.is_empty() {
		true => Ok(program),
		false => Err("a program has bytes past its end".into()),
	}
}

/// A string the macro wrote, or that an expression it wrote evaluated to.
pub struct LitOrExpr<'src>(pub &'src str);

impl<'src> Decode<'src> for LitOrExpr<'src> {
	fn decode(data: &mut Data<'src>) -> Result<Self, Error> {
		Ok(Self(<&'src str>::decode(data)?))
	}
}

impl<'src> Decode<'src> for bool {
	fn decode(data: &mut Data<'src>) -> Result<Self, Error> {
		Ok(data.byte()? != 0)
	}
}

impl<'src> Decode<'src> for u32 {
	fn decode(data: &mut Data<'src>) -> Result<Self, Error> {
		let mut cur = 0u32;
		let mut offset = 0;
		loop {
			let byte = data.byte()?;
			let bits = u32::from(byte & 0x7f);
			cur |= bits
				.checked_shl(offset)
				.filter(|_| offset < 32)
				.ok_or("a number past 32 bits")?;
			if byte & 0x80 == 0 {
				break Ok(cur);
			}
			offset += 7;
		}
	}
}

impl<'src> Decode<'src> for &'src str {
	fn decode(data: &mut Data<'src>) -> Result<Self, Error> {
		let n = u32::decode(data)? as usize;
		if n > data.bytes.len() {
			return Err("a string past the program's end".into());
		}
		let (a, b) = data.bytes.split_at(n);
		data.bytes = b;
		str::from_utf8(a).map_err(|_| "a string that is not UTF-8".into())
	}
}

impl<'src> Decode<'src> for String {
	fn decode(data: &mut Data<'src>) -> Result<Self, Error> {
		Ok(<&'src str>::decode(data)?.to_string())
	}
}

impl<'src, T: Decode<'src>> Decode<'src> for Vec<T> {
	fn decode(data: &mut Data<'src>) -> Result<Self, Error> {
		let n = u32::decode(data)?;
		// Each item takes at least a byte, which bounds what a length can make us hold.
		let mut v = Vec::with_capacity((n as usize).min(data.bytes.len()));
		for _ in 0..n {
			v.push(T::decode(data)?);
		}
		Ok(v)
	}
}

impl<'src, T: Decode<'src>> Decode<'src> for Option<T> {
	fn decode(data: &mut Data<'src>) -> Result<Self, Error> {
		match data.byte()? {
			0 => Ok(None),
			1 => Ok(Some(T::decode(data)?)),
			_ => Err("an option tag past 1".into()),
		}
	}
}

/// A struct whose fields decode in order. A field marked `since` is in programs of that
/// schema and later, and `None` in earlier ones.
macro_rules! decode_struct {
	($name:ident ($($lt:tt)*) { $($(#[since($since:ident)])? $field:ident: $ty:ty,)* }) => {
		#[allow(dead_code, reason = "decoded to read what follows, whether or not translation reads it")]
		pub struct $name <$($lt)*> {
			$(pub $field: decode_struct!(@ty $($since)? $ty),)*
		}

		impl<'a> Decode<'a> for $name <$($lt)*> {
			fn decode(data: &mut Data<'a>) -> Result<Self, Error> {
				Ok($name {
					$($field: decode_struct!(@field data $($since)? $ty),)*
				})
			}
		}
	};
	(@ty $since:ident $ty:ty) => (Option<$ty>);
	(@ty $ty:ty) => ($ty);
	(@field $data:ident $since:ident $ty:ty) => (
		match $data.schema >= Schema::$since {
			true => Some(<$ty>::decode($data)?),
			false => None,
		}
	);
	(@field $data:ident $ty:ty) => (<$ty>::decode($data)?);
}

/// An enum whose variants are numbered in order.
macro_rules! decode_enum {
	($name:ident ($($lt:tt)*) { $($variant:ident $(($ty:ty))?,)* }) => {
		#[allow(dead_code, reason = "decoded to read what follows, whether or not translation reads it")]
		pub enum $name <$($lt)*> {
			$($variant $(($ty))?,)*
		}

		impl<'a> Decode<'a> for $name <$($lt)*> {
			fn decode(data: &mut Data<'a>) -> Result<Self, Error> {
				let tag = data.byte()?;
				let mut n = 0u8;
				$(
					if tag == n {
						return Ok($name::$variant $((<$ty>::decode(data)?))?);
					}
					n += 1;
				)*
				let _ = n;
				Err(format!("variant {tag} of `{}`, which has fewer", stringify!($name)))
			}
		}
	};
}

decode_struct!(Program('a) {
	exports: Vec<Export<'a>>,
	enums: Vec<Enum<'a>>,
	imports: Vec<Import<'a>>,
	structs: Vec<Struct<'a>>,
	typescript_custom_sections: Vec<LitOrExpr<'a>>,
	local_modules: Vec<LocalModule<'a>>,
	inline_js: Vec<&'a str>,
	unique_crate_identifier: &'a str,
	#[since(V0_2_130)]
	module_path: LitOrExpr<'a>,
	package_json: Option<&'a str>,
	linked_modules: Vec<LinkedModule<'a>>,
});

decode_struct!(Import('a) {
	module: Option<ImportModule<'a>>,
	js_namespace: Option<Vec<String>>,
	reexport: Option<String>,
	generate_typescript: bool,
	kind: ImportKind<'a>,
});

decode_struct!(LinkedModule('a) {
	module: ImportModule<'a>,
	link_function_name: &'a str,
});

decode_enum!(ImportModule('a) {
	Named(&'a str),
	RawNamed(&'a str),
	Inline(u32),
});

decode_enum!(ImportKind('a) {
	Function(ImportFunction<'a>),
	Static(ImportStatic<'a>),
	String(ImportString<'a>),
	Type(ImportType<'a>),
	Enum(StringEnum<'a>),
	DynamicUnion(DynamicUnion<'a>),
});

decode_struct!(ImportFunction('a) {
	shim: &'a str,
	catch: bool,
	variadic: bool,
	assert_no_shim: bool,
	suspending: bool,
	method: Option<MethodData<'a>>,
	structural: bool,
	function: Function<'a>,
	generic_per_mono: bool,
});

decode_struct!(MethodData('a) {
	class: &'a str,
	kind: MethodKind<'a>,
});

decode_enum!(MethodKind('a) {
	Constructor,
	Operation(Operation<'a>),
});

decode_struct!(Operation('a) {
	is_static: bool,
	kind: OperationKind<'a>,
});

decode_enum!(OperationKind('a) {
	Regular,
	RegularThis,
	Getter(&'a str),
	Setter(&'a str),
	IndexingGetter,
	IndexingSetter,
	IndexingDeleter,
});

decode_struct!(ImportStatic('a) {
	name: &'a str,
	shim: &'a str,
});

decode_struct!(ImportString('a) {
	shim: &'a str,
	string: &'a str,
});

decode_struct!(ImportType('a) {
	name: &'a str,
	instanceof_shim: &'a str,
	vendor_prefixes: Vec<&'a str>,
});

decode_struct!(StringEnum('a) {
	name: &'a str,
	variant_values: Vec<&'a str>,
	comments: Vec<&'a str>,
	generate_typescript: bool,
	private: bool,
	js_namespace: Option<Vec<&'a str>>,
});

decode_enum!(StartKind() {
	None,
	Public,
	Private,
});

decode_struct!(DynamicUnion('a) {
	name: &'a str,
	variant_strings: Vec<&'a str>,
	variant_type_cnt: u32,
	comments: Vec<&'a str>,
	generate_typescript: bool,
	private: bool,
	fallback: bool,
});

decode_struct!(Export('a) {
	class: Option<&'a str>,
	comments: Vec<&'a str>,
	consumed: bool,
	function: Function<'a>,
	js_namespace: Option<Vec<&'a str>>,
	method_kind: MethodKind<'a>,
	start: StartKind,
});

decode_struct!(Enum('a) {
	name: &'a str,
	signed: bool,
	variants: Vec<EnumVariant<'a>>,
	comments: Vec<&'a str>,
	generate_typescript: bool,
	js_namespace: Option<Vec<&'a str>>,
	private: bool,
});

decode_struct!(EnumVariant('a) {
	name: &'a str,
	value: u32,
	comments: Vec<&'a str>,
});

decode_struct!(Function('a) {
	args: Vec<FunctionArgumentData<'a>>,
	asyncness: bool,
	jspi: bool,
	name: &'a str,
	generate_typescript: bool,
	generate_jsdoc: bool,
	variadic: bool,
	ret_ty_override: Option<&'a str>,
	ret_desc: Option<&'a str>,
});

decode_struct!(FunctionArgumentData('a) {
	name: String,
	ty_override: Option<&'a str>,
	optional: bool,
	desc: Option<&'a str>,
});

decode_struct!(Struct('a) {
	name: &'a str,
	fields: Vec<StructField<'a>>,
	comments: Vec<&'a str>,
	is_inspectable: bool,
	generate_typescript: bool,
	js_namespace: Option<Vec<&'a str>>,
	private: bool,
	extends: Option<&'a str>,
	extends_js_class: Option<&'a str>,
	extends_js_namespace: Option<Vec<&'a str>>,
});

decode_struct!(StructField('a) {
	name: &'a str,
	readonly: bool,
	comments: Vec<&'a str>,
	generate_typescript: bool,
	generate_jsdoc: bool,
});

decode_struct!(LocalModule('a) {
	identifier: &'a str,
	contents: &'a str,
	linked_module: bool,
});

/// Each program in a `__wasm_bindgen_unstable` static's bytes, with its schema: a
/// length-prefixed JSON prefix naming the schema, then a length-prefixed program, as many
/// times as the static holds. A schema other than the supported ones is an error naming
/// them, since its programs would decode wrongly.
pub fn section(mut bytes: &[u8]) -> Result<Vec<(Schema, &[u8])>, Error> {
	fn chunk<'a>(bytes: &mut &'a [u8]) -> Result<&'a [u8], Error> {
		let (len, rest) = bytes.split_first_chunk::<4>().ok_or("a section ends inside a length")?;
		let len = u32::from_le_bytes(*len) as usize;
		if len > rest.len() {
			return Err("a section's length past its end".into());
		}
		let (chunk, rest) = rest.split_at(len);
		*bytes = rest;
		Ok(chunk)
	}
	let mut programs = Vec::new();
	while !bytes.is_empty() {
		let prefix = str::from_utf8(chunk(&mut bytes)?).map_err(|_| "a schema prefix that is not UTF-8")?;
		let version = prefix
			.split_once("\"schema_version\":\"")
			.and_then(|(_, rest)| rest.split_once('"'))
			.map(|(version, _)| version)
			.ok_or_else(|| format!("a schema prefix without its version: {prefix}"))?;
		let schema = Schema::of(version).ok_or_else(|| {
			let supported: Vec<_> = Schema::ALL.iter().map(|s| s.version()).collect();
			format!(
				"wasm-bindgen schema {version}, while translation reads {}",
				supported.join(" and ")
			)
		})?;
		programs.push((schema, chunk(&mut bytes)?));
	}
	Ok(programs)
}

#[cfg(test)]
mod tests {
	use super::*;

	/// A section of one program of `schema_version`, holding `program`.
	fn section_of(schema_version: &str, program: &[u8]) -> Vec<u8> {
		let prefix = format!("{{\"schema_version\":\"{schema_version}\",\"version\":\"{schema_version}\"}}");
		let mut bytes = Vec::new();
		bytes.extend((prefix.len() as u32).to_le_bytes());
		bytes.extend(prefix.as_bytes());
		bytes.extend((program.len() as u32).to_le_bytes());
		bytes.extend(program);
		bytes
	}

	/// An empty program, as each schema encodes it.
	fn empty(schema: Schema) -> Vec<u8> {
		// No exports, enums, imports, structs, sections, modules, or snippets, then the
		// crate's identifier.
		let mut program = vec![0, 0, 0, 0, 0, 0, 0, 3, b'a', b'-', b'b'];
		if schema >= Schema::V0_2_130 {
			program.extend([4, b'a', b':', b':', b'b']);
		}
		// No `package.json` and no linked modules.
		program.extend([0, 0]);
		program
	}

	#[test]
	fn each_schema() {
		for schema in Schema::ALL {
			let bytes = section_of(schema.version(), &empty(schema));
			let programs = section(&bytes).unwrap();
			let [(got, data)] = programs[..] else {
				panic!("one program")
			};
			assert_eq!(got, schema);
			let program = super::program(data, schema).unwrap();
			assert_eq!(program.unique_crate_identifier, "a-b");
			assert_eq!(
				program.module_path.map(|p| p.0),
				(schema >= Schema::V0_2_130).then_some("a::b")
			);
		}
		// One schema's program is malformed in the other's.
		assert!(super::program(&empty(Schema::V0_2_130), Schema::V0_2_128).is_err());
		assert!(super::program(&empty(Schema::V0_2_128), Schema::V0_2_130).is_err());
	}

	#[test]
	fn unsupported_schema() {
		for version in ["0.2.122", "0.2.129", "0.2.131"] {
			let err = section(&section_of(version, &[])).err().unwrap();
			assert_eq!(
				err,
				format!("wasm-bindgen schema {version}, while translation reads 0.2.128 and 0.2.130")
			);
		}
	}
}
