use serde::Serialize;
use serde::ser::Error as _;


#[derive(Debug)]
pub struct Error {
    message: String,
}

impl std::fmt::Display for Error
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error
{
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl serde::ser::Error for Error
{
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

impl serde::de::Error for Error
{
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

const TRUE: u8 = 0xf5;
const FALSE: u8 = 0xf4;
const NONE: u8 = 0xf6;
const SOME: u8 = 0xf7;

pub fn to_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.into_inner())
}

pub struct Serializer {
    buffer: Vec<u8>,
}

impl Serializer
{
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.buffer
    }
}

impl<'a> serde::Serializer for &'a mut Serializer
{
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        if v {
            TRUE.serialize(&mut *self)
        } else {
            FALSE.serialize(&mut *self)
        }
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer.extend(v.to_be_bytes().to_vec()))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer.extend(v.to_be_bytes().to_vec()))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer.extend(v.to_be_bytes().to_vec()))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer.extend(v.to_be_bytes().to_vec()))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer.extend(v.to_be_bytes().to_vec()))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer.extend(v.to_be_bytes().to_vec()))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer.extend(v.to_be_bytes().to_vec()))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer.extend(v.to_be_bytes().to_vec()))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer.extend(v.to_be_bytes().to_vec()))
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        Ok(self.buffer.extend(v.to_be_bytes().to_vec()))
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let bytes = v.to_string().into_bytes();
        self.serialize_u8(bytes.len() as u8)?;
        Ok(self.buffer.extend(bytes))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        let bytes = v.to_string().into_bytes();
        // assert that the string length fits into a u32
        if bytes.len() > 0xffffffff {
            return Err(Error::custom("string length is too large"));
        }
        self.serialize_u32(bytes.len() as u32)?;
        Ok(self.buffer.extend(bytes))
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        // assert that the byte length fits into a u32
        if v.len() > 0xffffffff {
            return Err(Error::custom("byte length is too large"));
        }
        self.serialize_u32(v.len() as u32)?;
        Ok(self.buffer.extend(v))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        NONE.serialize(&mut *self)
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        SOME.serialize(&mut *self)?;
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(_variant_index)
    }
    
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, variant: &'static str, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::Serialize,
    {
        _variant_index.serialize(&mut *self)?;
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        // error if len is None
        let len = _len.ok_or(Error::custom("sequence length is unknown"))?;
        // check if the length fits into a u32
        if len > 0xffffffff {
            return Err(Error::custom("sequence length is too large"));
        }
        (len as u32).serialize(&mut *self)?;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeTupleVariant, Self::Error> {
        _variant_index.serialize(&mut *self)?;
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        // error if len is None
        let len = _len.ok_or(Error::custom("map length is unknown"))?;
        // check if the length fits into a u32
        if len > 0xffffffff {
            return Err(Error::custom("map length is too large"));
        }
        (len as u32).serialize(&mut *self)?;
        Ok(self)
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, variant: &'static str, len: usize) -> Result<Self::SerializeStructVariant, Self::Error> {
        _variant_index.serialize(&mut *self)?;
        Ok(self)
    }
}

impl<'a> serde::ser::SerializeSeq for &'a mut Serializer
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTuple for &'a mut Serializer
{
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTupleStruct for &'a mut Serializer
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeTupleVariant for &'a mut Serializer
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeMap for &'a mut Serializer
{
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStruct for &'a mut Serializer
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

impl<'a> serde::ser::SerializeStructVariant for &'a mut Serializer
{
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: serde::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

pub fn from_bytes<'de, T>(bytes: &'de [u8]) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
{
    let boxed: Box<dyn std::io::Read + 'de> = Box::new(bytes);
    let mut deserializer = Deserializer::new(boxed);
    T::deserialize(&mut deserializer)
}

pub fn from_reader<'de, T, De>(reader: De) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned,
    De: std::io::Read + 'de,
{
    let boxed: Box<dyn std::io::Read + 'de> = Box::new(reader);
    let mut deserializer = Deserializer::new(boxed);
    T::deserialize(&mut deserializer)
}

pub struct Deserializer<'de>
{
    reader: Box<dyn std::io::Read + 'de>,
}

impl<'de> Deserializer<'de>
{
    pub fn new(reader: Box<dyn std::io::Read + 'de>) -> Self {
        Self {
            reader,
        }
    }
}

impl<'de> std::io::Read for &mut Deserializer<'de>
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.reader.read(buf)
    }
}

impl<'de> serde::Deserializer<'de> for &'de mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        Err(Error::custom("not implemented"))
    }

    fn deserialize_bool<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        // Get one byte from self.reader
        let mut bytes = [0; 1];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        visitor.visit_bool(match bytes[0] {
            TRUE => true,
            FALSE => false,
            _ => return Err(Error::custom("invalid boolean value")),
        })
    }

    fn deserialize_i8<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let mut bytes = [0; 1];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        visitor.visit_i8(i8::from_be_bytes(bytes))
    }

    fn deserialize_i16<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let mut bytes = [0; 2];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        visitor.visit_i16(i16::from_be_bytes(bytes))
    }

    fn deserialize_i32<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let mut bytes = [0; 4];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        visitor.visit_i32(i32::from_be_bytes(bytes))
    }

    fn deserialize_i64<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let mut bytes = [0; 8];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        visitor.visit_i64(i64::from_be_bytes(bytes))
    }

    fn deserialize_u8<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let mut bytes = [0; 1];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        visitor.visit_u8(bytes[0])
    }

    fn deserialize_u16<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let mut bytes = [0; 2];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        visitor.visit_u16(u16::from_be_bytes(bytes))
    }

    fn deserialize_u32<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let mut bytes = [0; 4];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        visitor.visit_u32(u32::from_be_bytes(bytes))
    }

    fn deserialize_u64<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let mut bytes = [0; 8];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        visitor.visit_u64(u64::from_be_bytes(bytes))
    }

    fn deserialize_f32<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let mut bytes = [0; 4];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        visitor.visit_f32(f32::from_be_bytes(bytes))
    }

    fn deserialize_f64<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let mut bytes = [0; 8];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        visitor.visit_f64(f64::from_be_bytes(bytes))
    }

    fn deserialize_char<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let len: u8 = from_reader(&mut *self)?;
        let mut bytes = vec![0; len as usize];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        let s = String::from_utf8(bytes).map_err(|_| Error::custom("invalid utf-8"))?;
        let c = s.chars().next().ok_or(Error::custom("empty string"))?;
        visitor.visit_char(c)
    }

    fn deserialize_str<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let len: u32 = from_reader(&mut *self)?;
        let mut bytes = vec![0; len as usize];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        let s = String::from_utf8(bytes).map_err(|_| Error::custom("invalid utf-8"))?;
        visitor.visit_str(&s)
    }

    fn deserialize_string<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let len: u32 = from_reader(&mut *self)?;
        let mut bytes = vec![0; len as usize];
        self.reader.read_exact(&mut bytes).map_err(|_| Error::custom("failed to read"))?;
        visitor.visit_bytes(&bytes)
    }

    fn deserialize_byte_buf<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let b: u8 = from_reader(&mut *self)?;
        match b {
            NONE => visitor.visit_none(),
            SOME => visitor.visit_some(self),
            _ => Err(Error::custom("invalid option value")),
        }
    }

    fn deserialize_unit<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V: serde::de::Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V: serde::de::Visitor<'de>>(self, _name: &'static str, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let len: u32 = from_reader(&mut *self)?;
        visitor.visit_seq(Walk { de: self, len: len as usize })
    }

    fn deserialize_tuple<V: serde::de::Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_seq(Walk { de: self, len })
    }

    fn deserialize_tuple_struct<V: serde::de::Visitor<'de>>(self, _name: &'static str, len: usize, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_seq(Walk { de: self, len })
    }

    fn deserialize_map<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        let len: u32 = from_reader(&mut *self)?;
        visitor.visit_map(Walk { de: self, len: len as usize })
    }

    fn deserialize_struct<V: serde::de::Visitor<'de>>(self, _name: &'static str, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_seq(Walk { de: self, len: _fields.len() })
    }

    fn deserialize_enum<V: serde::de::Visitor<'de>>(self, _name: &'static str, _variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
        let variant_index: u32 = from_reader(&mut *self)?;
        visitor.visit_enum(Walk { de: self, len: 1 })
    }

    fn deserialize_identifier<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        Err(Error::custom("not implemented"))
    }

    fn deserialize_ignored_any<V: serde::de::Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        Err(Error::custom("not implemented"))
    }
}

struct Walk<'de>
{
    de: &'de mut Deserializer<'de>,
    len: usize,
}

impl<'de> serde::de::SeqAccess<'de> for Walk<'de>
{
    type Error = Error;

    fn next_element_seed<T: serde::de::DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
        if self.len > 0 {
            self.len -= 1;
            let value = seed.deserialize(&mut *self.de)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'de> serde::de::MapAccess<'de> for Walk<'de>
{
    type Error = Error;

    fn next_key_seed<K: serde::de::DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
        if self.len > 0 {
            self.len -= 1;
            let key = seed.deserialize(&mut *self.de)?;
            Ok(Some(key))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V: serde::de::DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
        let value = seed.deserialize(&mut *self.de)?;
        Ok(value)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.len)
    }
}

impl<'de> serde::de::EnumAccess<'de> for Walk<'de>
{
    type Error = Error;
    type Variant = Self;

    fn variant_seed<V: serde::de::DeserializeSeed<'de>>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error> {
        let value = seed.deserialize(&mut *self.de)?;
        Ok((value, self))
    }
}

impl<'de> serde::de::VariantAccess<'de> for Walk<'de>
{
    type Error = Error;

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T: serde::de::DeserializeSeed<'de>>(self, seed: T) -> Result<T::Value, Self::Error> {
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V: serde::de::Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_seq(Walk { de: self.de, len })
    }

    fn struct_variant<V: serde::de::Visitor<'de>>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error> {
        visitor.visit_seq(Walk { de: self.de, len: _fields.len() })
    }
}