use std::collections::HashMap;
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
struct GeneLength {
    startvec: usize,
    endvec: usize,
    length: usize,
}

pub async fn threadedlengthmouse(count: &str) -> Result<String, Box<dyn Error>> {
    let pathadd = Path::new("gencode.v48.primary_assembly.annotation.gtf.gz");
    if !pathadd.exists() {
        let _ = Command::new("wget").
         arg("https://ftp.ebi.ac.uk/pub/databases/gencode/Gencode_mouse/release_M37/gencode.vM37.primary_assembly.annotation.gtf.gz")
         .output()
            .expect("command failed");
        let _ = Command::new("gunzip")
            .arg("gencode.vM37.primary_assembly.annotation.gtf.gz")
            .output()
            .expect("Command failed");
        let _ = Command::new("gunzip")
            .arg("*")
            .output()
            .expect("Command failed");

        let fileall =
            File::open("gencode.vM37.primary_assembly.annotation.gtf").expect("file not present");
        let filecontent = BufReader::new(fileall);
        let mut id: Vec<String> = Vec::new();
        let mut start: Vec<usize> = Vec::new();
        let mut end: Vec<usize> = Vec::new();
        for i in filecontent.lines() {
            let line = i.expect("line not present");
            if !line.starts_with("#") {
                let linevec: Vec<_> = line
                    .split("\t")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>();
                for _i in 0..linevec.len() {
                    if linevec[2].clone().to_string() == "gene" {
                        start.push(linevec[3].parse::<usize>().unwrap());
                        end.push(linevec[4].parse::<usize>().unwrap());
                        id.push(
                            linevec[8].split(";").collect::<Vec<_>>()[0]
                                .to_string()
                                .replace(" ", "-")
                                .split("-")
                                .collect::<Vec<_>>()[1]
                                .to_string(),
                        )
                    }
                }
            }
        }
        let mut genevecrwrite: HashMap<String, GeneLength> = HashMap::new();
        for i in 0..id.len() {
            genevecrwrite.insert(
                id[i].to_string(),
                GeneLength {
                    startvec: start[i],
                    endvec: end[i],
                    length: end[i] - start[i],
                },
            );
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

        let mut expressmatrix: Vec<(String, usize, usize, usize)> = Vec::new();
        for (i, value) in averagecount.iter() {
            for (j, val) in genevecrwrite.iter() {
                if i == j {
                    let divide: usize = 1000000000usize * value;
                    let mapdivide = divide / allmapped;
                    let finalmap = mapdivide / val.length;
                    let matrixinsert: (String, usize, usize, usize) =
                        (i.clone(), finalmap, *value, val.length);
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

        let mut filewrite = File::create("express-count.txt").expect("File not present");
        for i in expressmatrix.iter() {
            writeln!(filewrite, "{:?}\t{}\t{}\t{}", i.0, i.1, i.2, i.3).expect("no file present");
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
