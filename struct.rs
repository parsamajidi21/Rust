struct Uni {
    name: String,
    city: String,
    rank: u8,
}


fn descripe(uni: &Uni){
    println!("{} is in {} and has {} QS ranking", uni.name, uni.city, uni.rank);
}

fn main(){
    let unipd = Uni {name: String::from("University of Padova"), city: String::from("Padua"), rank: 100};

    descripe(&unipd);
}