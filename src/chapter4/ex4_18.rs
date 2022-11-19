#[derive(Copy, Clone, Debug)]
struct CubeSat {
    id: u64
}

#[derive(Copy, Clone, Debug)]
enum StatusMessage {
    OK,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::OK
}

pub fn example() {

    let sat_a = CubeSat { id: 0 };

    let a_status = check_status(sat_a.clone());
    println!("a: {:?}", a_status);

    // '대기 중...'
    let a_status = check_status(sat_a);
    println!("a: {:?}", a_status);
}