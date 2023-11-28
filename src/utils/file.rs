use std::{fs::{self, DirEntry}, ffi::OsString, collections::VecDeque, io};
use super::file_type::FileType;

#[derive(Clone,Debug)]
pub struct File{
    name:OsString,
    size:u64,
    path:String,
    // extention:String,
    file_type:FileType,
    folder:bool,
}

impl File{
    pub fn new(entry:DirEntry) -> File {
        File{
            name:entry.file_name(),
            size:fs::metadata(entry.path()).unwrap().len(),
            path:String::from(entry.path().to_str().unwrap()),
            // extention:
            file_type:FileType::UnDefinded,
            folder:fs::metadata(entry.path()).unwrap().is_dir(),
        }
    }
}

impl std::fmt::Display for File{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"name: {:?}, \nfile size: {} bytes, \npath: {}, \nIs Dir: {}",self.name,self.size,self.path,self.folder)
    }
}

pub fn get_path_contents(path:&str) -> Result<(Vec<File>,VecDeque<File>),io::Error> {
    let dir: fs::ReadDir = fs::read_dir(path)?;
    let mut return_vec: Vec<File> = Vec::new();
    let mut search_queue = VecDeque::new();

    for file in dir{
        match file {
            Ok(entry) => {
                let return_file: File = File::new(entry);

                match return_file.folder {
                    true => {
                        search_queue.push_back(return_file);
                    }
                    false => {
                        return_vec.push(return_file);
                    }
                }
            }
            Err(error) => {
                return Err(error);
            }
        }
    }

    Ok((return_vec,search_queue))
}

pub fn search_files_bfs(search_queue:VecDeque<File>, depth:i32) -> Vec<File> {
    let mut file_list: Vec<File> = Vec::new();

    //base case
    if search_queue.len() == 0 {
        return file_list
    } else if depth == 0{
        file_list.append(&mut Vec::from(search_queue));
        return file_list
    }

    // recursive case
    for folder in search_queue{
        let (mut files, folders) = match get_path_contents(&folder.path){
            Ok(value) => {value} 
            Err(_error) => {(Vec::new(),VecDeque::new())}
        };
        let mut deep_files = search_files_bfs(folders, depth-1);
        file_list.append(&mut files);
        file_list.append(&mut deep_files)
    }

    file_list
}

pub fn sum_file_size(folder:Vec<File>) -> u64 {
    let mut file_size = 0u64;
    for file in folder{
        file_size += file.size
    }
    file_size
}