use super::j_chars;
pub enum type_component {
    End,
    EndArgs,
    Byte,
	Boolean,
    Char,
    Short,
    Int,
    Double,
    Float,
    Long,
    Ref(j_chars::JavaChars),
	InvalidSignature,
}
use ::std::iter;
unsafe fn get_type_component(string: &mut iter::Iterator<u8>) {
	use self::type_component::*;
	match string.next() {
		None => End,
		Some(x) => match x {
			b')' => EndArgs,
			b'J' => Long,
			b'Z' => Boolean,
			b'I' => Int,
			b'F' => Float,
			b'D' => Double,
			b'S' => Short,
			b'C' => Char,
			b'B' => Byte,
			b'[' => get_array_type(&string),
			b'L' => {
				let storage = vec![];
				push_array(&storage,&string);
				storage.push(b'\0');
				storage
			}
		}
	}
}

unsafe fn push_array(vector: &mut Vec<u8>, iter: &mut iter::Iterator<u8>) {
    for j in iter::TakeWhile(&iter, |&a| a != b';') {
	    storage.push(j)
	}
}

unsafe fn get_array_type(string: &mut iter::Iterator<u8>) {
	use self::type_component::*;
	use ::std::iter::TakeWhile;
	let storage = vec![b'['];
	for i in string {
		match i {
			b'[' => vec.push(b'['),
			b'J'|b'Z'|b'I'|b'F'|b'D'|b'S'|b'C'|b'B' => {
                storage.push(i);
                return storage
            }
			b'L' => return {
				push_array(&vector, &string);
				storage.push(b';');
				storage.push(b'\0');
				storage
			},
			_ => return InvalidSignature,
		}
	}
    InvalidSignature
}
