use std::{env, io};
use std::fs::{self, File};
use std::io::{BufReader, BufRead, Write};
use std::collections::HashSet;

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: make_batch_script <mzxml directory> <psm file URL>");
        std::process::exit(1);
    }

    let mut param_cnt : i32 = 0;
    println!("------------------ Parameters ----------------");

    let mzxml_dir  = &args[1];    
    let psm_url = &args[2];
    

    for arg in &args {
        
        if param_cnt == 1 {
            println!("mzxml url : {arg}");
        } else if param_cnt == 2 {
            println!("peptide list url : {arg}");            
        } else if param_cnt == 3 {
            println!("PSM url : {arg}" );
        }
        param_cnt += 1;
    }

    if let Err(e) = make_script(mzxml_dir, psm_url) {
        eprintln!("Error creating batch script:{}", e);
        std::process::exit(1);
    }

}

fn make_script(mzxml_dir : &str, psm_url:&str) -> io::Result<()> {

    let batch_script_url = "./PeptideXICAnnotation_Batch.cmd";

    let file = File::open(psm_url)?;
    let mut reader = BufReader::new(file);
    let mut peptide_set = HashSet::new();

    let mut peptide_index = -1;
    let mut head_line = String::new();
    reader.read_line(&mut head_line)?;

    let head_cols : Vec<&str> = head_line.split('\t').collect();

    let mut col_index : i32 = 0;
    for head_col in head_cols {
        if head_col.to_uppercase() == "PEPTIDE" {
            peptide_index = col_index;
            // println!("peptide index is {}", peptide_index);
            break;
        }
        col_index += 1;
    }

    if peptide_index == -1 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "peptide column not found!"));
    }

    for line in reader.lines() {
        let line = line?;
        let columns:  Vec<&str> = line.split('\t').collect();
        if let Some(peptide) = columns.get(peptide_index as usize) {
            peptide_set.insert(peptide.to_string());
        }
    }
    println!("No. Non redundant peptides : {}", peptide_set.len());
    
    let mut batch_script = File::create(batch_script_url)?;

    for peptide in peptide_set {

        writeln!(batch_script, "Call PeptideXICAnnotation -mzxml {} -PEPTIDE {} -PSM {}", mzxml_dir, peptide, psm_url)?;
        writeln!(batch_script, "timeout /t 5 /nobreak")?;
    }

    Ok(())
}