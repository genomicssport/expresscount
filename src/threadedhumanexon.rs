use crate::exon::exonunwrap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::Command;

/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-7-21
*/

#[derive(Debug, Clone, PartialOrd, PartialEq)]
struct ExonCap {
    name: String,
    length: usize,
}

pub async fn threadedlengthhumanexon(count: &str) -> Result<String, Box<dyn Error>> {
    let pathadd = Path::new("gencode.v48.primary_assembly.annotation.gtf.gz");
    if !pathadd.exists() {
        let _ = Command::new("wget").
         arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_human/release_48/gencode.v48.primary_assembly.annotation.gtf.gz")
         .output()
            .expect("command failed");
        let _ = Command::new("gunzip")
            .arg("gencode.v48.primary_assembly.annotation.gtf.gz")
            .output()
            .expect("Command failed");
        let _ = Command::new("gunzip")
            .arg("*")
            .output()
            .expect("Command failed");

        let _ = exonunwrap(count).unwrap();
        let exonopen = File::open("exonsortedlength.txt").expect("file not present");
        let exonopenread = BufReader::new(exonopen);
        let mut exonvector: Vec<ExonCap> = Vec::new();
        for i in exonopenread.lines() {
            let line = i.expect("line not present");
            exonvector.push(ExonCap {
                name: line.split("\t").collect::<Vec<_>>()[0].to_string(),
                length: line.split("\t").collect::<Vec<_>>()[1]
                    .parse::<usize>()
                    .unwrap(),
            })
        }

        let countfile = File::open(count).expect("file not present");
        let mut averagecount: Vec<(String, usize)> = Vec::new();
        let countread = BufReader::new(countfile);
        for i in countread.lines() {
            let line = i.expect("line not present");
            let linevec = line
                .split("\t")
                .collect::<Vec<_>>()
                .into_iter()
                .filter(|x| x.to_string() != " ")
                .collect::<Vec<_>>()
                .into_iter()
                .filter(|x| x.to_string() != " ")
                .collect::<Vec<_>>();
            let linestring = linevec[0].to_string();
            let linecount: Vec<_> = linevec[1..linevec.len()].iter().collect::<Vec<_>>();
            let mut count = 0usize;
            for i in 0..linecount.len() {
                count += linecount[i].parse::<usize>().unwrap();
            }
            let insertvec: (String, usize) = (linestring, count / linecount.len());
            averagecount.push(insertvec);
        }

        let mut allmapped: usize = 0usize;
        for i in averagecount.iter() {
            allmapped += i.1;
        }

        let mut expressmatrix: Vec<(String, f64, usize, usize)> = Vec::new();
        for (i, value) in averagecount.iter() {
            for j in exonvector.iter() {
                if *i == j.name {
                    let divide: usize = 1000000000usize * value;
                    let mapdivide = divide as f64 / allmapped as f64;
                    let finalmap = mapdivide as f64 / j.length as f64;
                    let matrixinsert: (String, f64, usize, usize) =
                        (i.clone(), finalmap, *value, j.length);
                    expressmatrix.push(matrixinsert);
                }
            }
        }

        let mut totalmap: usize = 0usize;
        let mut totallength: usize = 0usize;

        for i in expressmatrix.iter() {
            totalmap += i.2;
            totallength += i.3;
        }

        let mut fpkmwrite = File::create("fpkm-express-count.txt").expect("File not present");
        for i in expressmatrix.iter() {
            writeln!(fpkmwrite, "{:?}\t{}\t{}\t{}", i.0, i.1, i.2, i.3).expect("no file present");
        }

        let mut tpmwrite = File::create("TPM-express-count.txt").expect("File not present");
        for i in expressmatrix.iter() {
            writeln!(
                tpmwrite,
                "{}\t{}\t{}\t{}",
                i.0,
                (i.1 as f64 / (totalmap as f64 / totallength as f64) as f64).to_string(),
                i.2,
                i.3
            )
            .expect("file not present");
        }

        let mut rpkmwrite = File::create("rpkm-express-count.txt").expect("file not present");
        for i in expressmatrix.iter() {
            writeln!(
                rpkmwrite,
                "{}\t{}\t{}",
                i.0,
                i.2 as usize * 1000000000usize / (allmapped * i.3),
                i.3
            )
            .expect("file not present");
        }

        let _ = Command::new("rm")
            .arg("-rf")
            .arg("gencode.v48.primary_assembly.annotation.gtf")
            .output()
            .expect("file not found");

        Ok("The gene length have been written".to_string())
    } else {
        Ok("no option supplied".to_string())
    }
}
