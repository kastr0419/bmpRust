use std::env;
use std::fs::File; //ファイルを扱うライブラリ
use std::io::prelude::*; //ファイル入出力ライブラリ
std::io::BufReader; 

struct FileHeader{
    filesize:u32,
    offset: u32,
}

struct bitmapinfoheader{
    
}