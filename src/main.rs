use std::fs::File;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let in_path = Path::new("../data.csv");
    let _data_out_path=Path::new("../data_out.csv");
    let _count_out_path=Path::new("../count_out.csv");
    
    let mut buffer = Vec::new();
    let mut sets=vec![vec![];5];
    let mut total_count=vec![0;5];
    let mut set_count=vec![0;5];
    let mut pre_index=vec![false;5];
    let mut cur_sum=vec![0;5];
    let mut write_str=Vec::new();
    
    let file = File::open(in_path).expect("unable to open");
    
    //store in buffer
    for line in BufReader::new(file).lines() {
        let line=line.unwrap();
        let vec:Vec<String> =line.split(',').map(str::to_owned).collect();
        buffer.push(vec);
    }
    
    for i in 1..buffer.len() {
        for j in 8..buffer[i].len() {
            let cur_val:usize= buffer[i][j].parse().unwrap();
            let col = j - 8;
            total_count[col] += cur_val;
            if i != 1 && pre_index[col] && cur_val == 0 {
                set_count[col] += 1;
                pre_index[col] = false;
                sets[col].push(cur_sum[col]);
                cur_sum[col] = 0;
            } else if i == buffer.len() - 1 && cur_val == 1 {
                set_count[col] += 1;
                cur_sum[col] += 1;
                sets[col].push(cur_sum[col]);
            } else {
                pre_index[col] = cur_val!=0;
                cur_sum[col] += cur_val;
            }
        }
    }
    
    //write on data_out
    let mut outfile=File::create(&_data_out_path).expect("unable to create");
    {
        write!(&mut write_str, "{}", ',').unwrap();
        for i in 0..5 {
            write!(&mut write_str, "{}{}", ',', i + 14).unwrap();
        }
        writeln!(&mut write_str).unwrap();
    
        write!(&mut write_str, "{}", "total count").unwrap();
        for i in 0..5 {
            write!(&mut write_str, "{}{}", ',', total_count[i]).unwrap()
        }
        writeln!(&mut write_str).unwrap();
        
        write!(&mut write_str, "{}", "set count").unwrap();
        for i in 0..5 {
            write!(&mut write_str, "{}{}", ',', set_count[i]).unwrap();
        }
        writeln!(&mut write_str).unwrap();
        outfile.write_all(&write_str).expect("unable to write");
        write_str.clear();
    }
    
    //write on count_out
    outfile=File::create(&_count_out_path).unwrap();
    {
        for i in 0..5 {
            write!(&mut write_str,"{}",i+14).unwrap();
            for word in &sets[i]{
                write!(&mut write_str,"{}{}",',',word).unwrap();
            }
            writeln!(&mut write_str).unwrap();
        }
        outfile.write_all(&write_str).expect("unable to write");
    }
}