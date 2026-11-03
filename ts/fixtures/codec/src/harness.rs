//! The one entry point both sides of the differential test run: native Rust built for
//! wasm32, and its TypeScript translation. The first input byte picks what to
//! exercise and the rest is its input; the output records every result and error, so
//! the two sides agree only if each step does.

use crate::coding::{
	BoundsExceeded, Decode, DecodeError, Decoder, Encode, EncodeError, Encoder, Form, Version, Versions, varint,
};
use crate::{SessionError, lite};

/// What a case runs on the rest of the input.
type Case = fn(&[u8], &mut Vec<u8>);

/// Each case's name and what it runs. The first byte picks one modulo their count, so a
/// case added at the end moves no other.
fn cases() -> Vec<(&'static str, Case)> {
	vec![
		("decode_script", decode_script),
		("encode_script", encode_script),
		("zigzag", zigzag),
		("sizes", sizes),
		("roundtrip", roundtrip::<Versions>),
		("roundtrip_lite::<Group>", roundtrip_lite::<lite::Group>),
		("roundtrip_lite::<Probe>", roundtrip_lite::<lite::Probe>),
		("encode_messages", encode_messages),
		("errors", errors),
	]
}

/// Run one case.
pub fn run(input: &[u8]) -> Vec<u8> {
	let mut out = Vec::new();
	let Some((&op, data)) = input.split_first() else {
		return out;
	};
	let cases = cases();
	let (_, case) = cases[usize::from(op) % cases.len()];
	case(data, &mut out);
	out
}

pub(crate) fn form(selector: u8) -> Form {
	match selector % 3 {
		0 => Form::Quic,
		1 => Form::LeadingOnes { seven: false },
		_ => Form::LeadingOnes { seven: true },
	}
}

pub(crate) fn lite_version(selector: u8) -> lite::Version {
	match selector % 7 {
		0 => lite::Version::Lite01,
		1 => lite::Version::Lite02,
		2 => lite::Version::Lite03,
		3 => lite::Version::Lite04,
		4 => lite::Version::Lite05,
		5 => lite::Version::Lite06,
		_ => lite::Version::Lite07,
	}
}

pub(crate) fn put_u64(out: &mut Vec<u8>, value: u64) {
	out.extend_from_slice(&value.to_be_bytes());
}

pub(crate) fn put_usize(out: &mut Vec<u8>, value: usize) {
	put_u64(out, value as u64);
}

/// Take up to 8 big-endian bytes as a `u64`, zero-padded on the right.
pub(crate) fn take_u64(data: &[u8]) -> (u64, &[u8]) {
	let mut buf = [0u8; 8];
	let n = data.len().min(8);
	buf[..n].copy_from_slice(&data[..n]);
	(u64::from_be_bytes(buf), &data[n..])
}

/// A decode error's code, then its text.
fn decode_failed(out: &mut Vec<u8>, err: &DecodeError) {
	decode_error(out, err);
	put_text(out, err.to_string(), format!("{err:?}"));
}

/// An encode error's code, then its text.
fn encode_failed(out: &mut Vec<u8>, err: &EncodeError) {
	encode_error(out, err);
	put_text(out, err.to_string(), format!("{err:?}"));
}

/// An error's thiserror `Display` and derived `Debug`, which callers see.
fn put_text(out: &mut Vec<u8>, display: String, debug: String) {
	out.extend_from_slice(display.as_bytes());
	out.push(0);
	out.extend_from_slice(debug.as_bytes());
	out.push(0);
}

pub(crate) fn decode_error(out: &mut Vec<u8>, err: &DecodeError) {
	out.push(0xee);
	match err {
		DecodeError::Short => out.push(0),
		DecodeError::Long => out.push(1),
		DecodeError::InvalidString(_) => out.push(2),
		DecodeError::InvalidMessage(id) => {
			out.push(3);
			put_u64(out, *id);
		}
		DecodeError::InvalidSubscribeLocation => out.push(4),
		DecodeError::InvalidValue => out.push(5),
		DecodeError::TooMany => out.push(6),
		DecodeError::BoundsExceeded => out.push(7),
		DecodeError::ExpectedEnd => out.push(8),
		DecodeError::MessageTooLarge { size, max } => {
			out.push(9);
			put_usize(out, *size);
			put_usize(out, *max);
		}
		DecodeError::ExpectedData => out.push(10),
		DecodeError::Duplicate => out.push(11),
		DecodeError::Missing => out.push(12),
		DecodeError::Unsupported => out.push(13),
		DecodeError::TrailingBytes => out.push(14),
		DecodeError::Version => out.push(15),
		DecodeError::Session(_) => out.push(16),
		DecodeError::MalformedAuthToken => out.push(17),
		DecodeError::MultipleAuthTokens => out.push(18),
	}
}

pub(crate) fn encode_error(out: &mut Vec<u8>, err: &EncodeError) {
	out.push(0xef);
	out.push(match err {
		EncodeError::BoundsExceeded => 0,
		EncodeError::TooLarge => 1,
		EncodeError::Short => 2,
		EncodeError::InvalidState => 3,
		EncodeError::TooMany => 4,
		EncodeError::Version => 5,
		EncodeError::Unsupported => 6,
	});
}

/// Read primitives as a script says until one fails: the script is the first
/// `data[1]` bytes after the form selector, and the wire is everything after it.
fn decode_script(data: &[u8], out: &mut Vec<u8>) {
	let Some((&selector, data)) = data.split_first() else {
		return;
	};
	let Some((&len, data)) = data.split_first() else {
		return;
	};
	let (script, wire) = data.split_at(data.len().min(len as usize));

	let mut r = Decoder::new(wire, form(selector));
	for &step in script {
		match decode_step(&mut r, step, out) {
			Ok(()) => out.push(0),
			Err(err) => {
				decode_failed(out, &err);
				put_usize(out, r.need());
				break;
			}
		}
		put_usize(out, r.remaining());
	}
	out.push(r.is_empty() as u8);
}

fn decode_step(r: &mut Decoder<'_>, step: u8, out: &mut Vec<u8>) -> Result<(), DecodeError> {
	match step % 10 {
		0 => out.push(r.u8()?),
		1 => out.extend_from_slice(&r.u16()?.to_be_bytes()),
		2 => out.push(r.bool()? as u8),
		3 => put_u64(out, r.varint()?),
		4 => match r.varint_opt()? {
			Some(v) => {
				out.push(1);
				put_u64(out, v);
			}
			None => out.push(0),
		},
		5 => {
			let bytes = r.bytes()?;
			put_usize(out, bytes.len());
			out.extend_from_slice(bytes);
		}
		6 => {
			let s = r.string()?;
			out.extend_from_slice(s.as_bytes());
		}
		7 => {
			let slice = r.slice((step / 10) as usize)?;
			out.extend_from_slice(slice);
		}
		8 => {
			let mut sub = r.sub((step / 10) as usize)?;
			put_u64(out, sub.varint()?);
			put_usize(out, sub.rest().len());
		}
		_ => put_usize(out, r.rest().len()),
	}
	Ok(())
}

/// Write primitives as a script says, taking each value from the data that follows.
fn encode_script(data: &[u8], out: &mut Vec<u8>) {
	let Some((&selector, data)) = data.split_first() else {
		return;
	};
	let Some((&len, data)) = data.split_first() else {
		return;
	};
	let (script, mut values) = data.split_at(data.len().min(len as usize));

	let mut buf = Vec::new();
	let mut w = Encoder::new(&mut buf, form(selector));
	let mut prefixes = Vec::new();
	for &step in script {
		let (value, rest) = take_u64(values);
		values = rest;
		let result = match step % 10 {
			0 => {
				w.u8(value as u8);
				Ok(())
			}
			1 => {
				w.u16(value as u16);
				Ok(())
			}
			2 => {
				w.bool(value & 1 == 1);
				Ok(())
			}
			3 => w.varint(value),
			4 => w.varint_opt(match value % 3 {
				0 => None,
				1 => Some(value),
				_ => Some(u64::MAX),
			}),
			5 => w.bytes(&value.to_be_bytes()[..(step / 10 % 9) as usize]),
			6 => match String::from_utf8(value.to_be_bytes().to_vec()) {
				Ok(s) => w.string(&s),
				Err(_) => w.string("not utf-8"),
			},
			7 => {
				prefixes.push(match value % 2 {
					0 => w.prefix_varint(),
					_ => w.prefix_u16(),
				});
				Ok(())
			}
			8 => match prefixes.pop() {
				Some(prefix) => {
					put_usize(out, w.since(&prefix));
					w.fill(prefix)
				}
				None => Ok(()),
			},
			_ => {
				if let Some(prefix) = prefixes.pop() {
					w.discard(prefix);
				}
				Ok(())
			}
		};
		if let Err(err) = result {
			encode_failed(out, &err);
		}
	}
	while let Some(prefix) = prefixes.pop() {
		if let Err(err) = w.fill(prefix) {
			encode_failed(out, &err);
		}
	}

	put_usize(out, buf.len());
	out.extend_from_slice(&buf);
}

fn zigzag(data: &[u8], out: &mut Vec<u8>) {
	let (value, _) = take_u64(data);
	let signed = value as i64;
	put_u64(out, varint::zigzag(signed));
	put_u64(out, varint::unzigzag(value) as u64);
	put_u64(out, varint::unzigzag(varint::zigzag(signed)) as u64);
}

fn sizes(data: &[u8], out: &mut Vec<u8>) {
	let (value, _) = take_u64(data);
	for selector in 0..3 {
		match varint::size(value, form(selector)) {
			Ok(size) => put_usize(out, size),
			Err(_) => out.push(0xff),
		}
	}
}

/// Decode a `T` from the wire, then encode what came out.
fn roundtrip<T: Decode<Form> + Encode<Form>>(data: &[u8], out: &mut Vec<u8>) {
	let Some((&selector, wire)) = data.split_first() else {
		return;
	};
	let form = form(selector);
	match T::decode_slice(wire, form) {
		Ok((value, used)) => {
			put_usize(out, used);
			let mut buf = Vec::new();
			match value.encode(&mut Encoder::new(&mut buf, form), form) {
				Ok(()) => out.extend_from_slice(&buf),
				Err(err) => encode_failed(out, &err),
			}
		}
		Err(err) => decode_failed(out, &err),
	}
}

/// [`roundtrip`] for a lite message, size-prefixed at a lite version.
fn roundtrip_lite<T: lite::Message>(data: &[u8], out: &mut Vec<u8>) {
	let Some((&selector, wire)) = data.split_first() else {
		return;
	};
	let version = lite_version(selector);
	match T::decode_slice(wire, version) {
		Ok((value, used)) => {
			put_usize(out, used);
			let mut buf = Vec::new();
			match value.encode(&mut Encoder::new(&mut buf, version.into()), version) {
				Ok(()) => out.extend_from_slice(&buf),
				Err(err) => encode_failed(out, &err),
			}
		}
		Err(err) => decode_failed(out, &err),
	}
}

/// Encode messages built from the data, at a lite version.
fn encode_messages(data: &[u8], out: &mut Vec<u8>) {
	let Some((&selector, data)) = data.split_first() else {
		return;
	};
	let version = lite_version(selector);
	let (subscribe, data) = take_u64(data);
	let (sequence, data) = take_u64(data);
	let (frame_start, data) = take_u64(data);
	let (bitrate, data) = take_u64(data);
	let (rtt, _) = take_u64(data);

	let group = lite::Group {
		subscribe,
		sequence,
		frame_start: frame_start % 4,
	};
	encode_message(&group, version, out);

	let probe = lite::Probe {
		bitrate: bitrate.checked_sub(1),
		rtt: (rtt % 3 != 0).then_some(rtt),
	};
	encode_message(&probe, version, out);

	let versions = Versions::from([Version(subscribe), Version(sequence)]);
	let mut buf = Vec::new();
	match versions.encode(&mut Encoder::new(&mut buf, version.into()), version) {
		Ok(()) => out.extend_from_slice(&buf),
		Err(err) => encode_failed(out, &err),
	}
}

fn encode_message<T: lite::Message>(msg: &T, version: lite::Version, out: &mut Vec<u8>) {
	let mut buf = Vec::new();
	let mut w = Encoder::new(&mut buf, version.into());
	match msg.encode(&mut w, version) {
		Ok(()) => {
			out.push(w.control.is_some() as u8);
			put_usize(out, buf.len());
			out.extend_from_slice(&buf);
		}
		Err(err) => encode_failed(out, &err),
	}
}

/// Each error the codec can return, built from the data, as callers see it.
fn errors(data: &[u8], out: &mut Vec<u8>) {
	let Some((&selector, data)) = data.split_first() else {
		return;
	};
	let (value, _) = take_u64(data);
	let err = match selector % 8 {
		0 => DecodeError::InvalidMessage(value),
		1 => DecodeError::MessageTooLarge {
			size: value as usize,
			max: (value >> 32) as usize,
		},
		2 => DecodeError::Session(SessionError::App(value as u16)),
		3 => DecodeError::Session(SessionError::ProtocolViolation),
		4 => match String::from_utf8(data.to_vec()) {
			Ok(_) => DecodeError::Missing,
			Err(e) => DecodeError::InvalidString(e),
		},
		5 => DecodeError::from(BoundsExceeded),
		6 => {
			let bounds = BoundsExceeded;
			put_text(out, bounds.to_string(), format!("{bounds:?} {bounds:#?}"));
			return;
		}
		_ => DecodeError::MalformedAuthToken,
	};
	put_text(out, format!("{err} {err:>30}|"), format!("{err:#?}"));
	decode_failed(out, &err);
}
