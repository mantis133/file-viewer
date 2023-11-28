use std::collections::VecDeque;
use file_viewer::utils::file::*;









fn main() {
    let _pathh = r"C:\Users\OEM\Desktop\desktop\creative apps\Rust\fidk\file-viewer";
    let _desk_path = r"C:\Users\OEM\Desktop";

    let (files, search_queue) = match get_path_contents(_desk_path){
        Ok(value) => {value} 
        Err(_error) => {(Vec::new(),VecDeque::new())}
    };

    let deep_files: Vec<File> = search_files_bfs(search_queue, 99);

    // for file in &files{
    //     println!("{}\n",file);
    // }

    // println!("\n \n \n");

    // for file in &deep_files{
    //     println!("{}\n",file);
    // }

    println!("{}",sum_file_size(deep_files) + sum_file_size(files))

}