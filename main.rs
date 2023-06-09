use std::env;
use std::fs::File; //ファイルを扱うライブラリ
use std::io::prelude::*; //ファイル入出力ライブラリ
std::io::BufReader; 

struct FileHeader{
    filesize: u32,
    filetype: String,
    offset: u32,
}

struct bitmapinfoheader{
    width: i32,
    height: i32,
    bitperpix: u16,
    datasize: u32,
}

struct Color{
    red: i32,
    grean: i32,
    blue: i32,
}

fn bit2integer()