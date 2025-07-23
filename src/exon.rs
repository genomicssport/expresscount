use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::thread;
/*
 Author Gaurav Sablok
 Instytut Chemii Bioorganicznej
 Polskiej Akademii Nauk
 ul. Noskowskiego 12/14 | 61-704, Poznań
 Date: 2025-7-23
*/

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Exoncapture {
    name: String,
    exonstart: Vec<usize>,
    exonend: Vec<usize>,
    length: usize,
}

// threaded spwan so that it can be asynchronous for the xon based fpkm estimation.

pub fn exonunwrap(exonlist: &str) -> Result<String, Box<dyn Error>> {
    thread::scope(|scope| {
        scope.spawn(|| {
            let fileopen = File::open(exonlist).expect("file not present");
            let fileread = BufReader::new(fileopen);
            let mut ensemblhashset: HashSet<_> = HashSet::new();
            let mut lineveccollect: Vec<_> = Vec::new();
            for i in fileread.lines() {
                let hashline = i.expect("line not present");
                if !hashline.starts_with("#") {
                    let hashvec: String = hashline.split("\t").collect::<Vec<_>>()[8]
                        .split(";")
                        .collect::<Vec<_>>()[0]
                        .split(" ")
                        .collect::<Vec<_>>()[1]
                        .to_string();
                    ensemblhashset.insert(hashvec.replace("\"", ""));
                    lineveccollect.push(hashline);
                }
            }
            let mut exonvec: Vec<Exoncapture> = Vec::new();
            for i in ensemblhashset.iter() {
                let mut exonst: Vec<usize> = Vec::new();
                let mut exoned: Vec<usize> = Vec::new();
                for val in lineveccollect.iter() {
                    if *i
                        == val.split("\t").collect::<Vec<_>>()[8]
                            .split(";")
                            .collect::<Vec<_>>()[0]
                            .split(" ")
                            .collect::<Vec<_>>()[1]
                            .to_string()
                            .replace("\"", "")
                        && val.split("\t").collect::<Vec<_>>()[2].to_string() == "exon"
                    {
                        exonst.push(
                            val.split("\t").collect::<Vec<_>>()[3]
                                .parse::<usize>()
                                .unwrap(),
                        );
                        exoned.push(
                            val.split("\t").collect::<Vec<_>>()[4]
                                .parse::<usize>()
                                .unwrap(),
                        );
                    }
                }
                let mut exoncum: Vec<usize> = Vec::new();
                for i in 0..exonst.len() {
                    exoncum.push(exoned[i] - exonst[i])
                }
                let cummulativeexon: usize = exoncum.iter().sum();
                exonvec.push(Exoncapture {
                    name: i.clone(),
                    exonstart: exonst,
                    exonend: exoned,
                    length: cummulativeexon,
                });
            }
            let mut exonwrite = File::create("exonsortedlength.txt").expect("file not found");
            for i in exonvec.iter() {
                writeln!(exonwrite, "{}\t{}", i.name, i.length).expect("line not found");
            }
        });
    });
    Ok("The file has been written with the following ids".to_string())
}
