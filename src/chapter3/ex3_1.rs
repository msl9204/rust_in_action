#![allow(unused_variables)]

type File = String;

pub fn example() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    // read(f1, vec![]);

}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}