use std::{fmt::Error, fs::File, io::{BufReader, Cursor, Read}, iter};

pub struct LabelsContainer {
  magic_nunber: u32,
  items_count: u32,
  data: Vec<u8>
}

pub struct ImagesContainer {
  magic_nunber: u32,
  items_count: u32,
  rows: u32,
  cols: u32,
  data: Vec<u8>
}

pub fn decode_labels(path: String) -> Result<LabelsContainer, Error> {
  let mut content = std::fs::read(path).unwrap();

  let m: [u8; 4] = content[0..4].try_into().unwrap();
  let s: [u8; 4] = content[4..8].try_into().unwrap();

  let magic_number = u32::from_be_bytes(m);
  let count = u32::from_be_bytes(s);
  let data = &content[8..];
  
  return Ok(LabelsContainer{
    magic_nunber: magic_number,
    items_count: count,
    data: data.to_vec()
  });
}

pub fn decode_images(path: String) -> ImagesContainer {
  let mut content = std::fs::read(path).unwrap();

  let m: [u8; 4] = content[0..4].try_into().unwrap();
  let s: [u8; 4] = content[4..8].try_into().unwrap();
  let r: [u8; 4] = content[8..12].try_into().unwrap();
  let c: [u8; 4] = content[12..16].try_into().unwrap();

  let magic_number = u32::from_be_bytes(m);
  let count = u32::from_be_bytes(s);
  let rows = u32::from_be_bytes(r);
  let cols = u32::from_be_bytes(c);
  let data = &content[16..];

  return ImagesContainer{
    magic_nunber: magic_number,
    items_count: count,
    rows: rows,
    cols: cols,
    data: data.to_vec()
  };
}

#[test]
fn test_label_decode() {
  let labels = decode_labels("MNIST/raw/train-labels-idx1-ubyte".to_string());
  assert!(labels.is_ok());
}

#[test]
fn test_image_decode() {
  let labels = decode_labels("MNIST/raw/train-images-idx3-ubyte".to_string());
  assert!(labels.is_ok());
}