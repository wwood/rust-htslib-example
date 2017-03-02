extern crate rust_htslib;
use rust_htslib::bam;
use rust_htslib::prelude::*;

fn main() {
    let bam = bam::Reader::from_path(&"test_member.fa.bam").unwrap();

    // pileup over all covered sites
    for p in bam.pileup() {
        let pileup = p.unwrap();
        println!("{}:{} depth {}", pileup.tid(), pileup.pos(), pileup.depth());

        for alignment in pileup.alignments() {
            match alignment.indel() {
                bam::pileup::Indel::Ins(len) => println!("Insertion of length {}", len),
                bam::pileup::Indel::Del(len) => println!("Deletion of length {}", len),
                _ => println!("Base {}", alignment.record().seq()[alignment.qpos()])
            }
        }
    }
}
