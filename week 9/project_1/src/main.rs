use std::io::Write;

fn main(){
    let lager = "\n1.) LAGER \ni.)33Export, \nii.)Desperados,\niii.) Goldberg,\niv.) Gulder, \nv.)Heineken,\nvi.) Star ";
    let stout = "\n2.)STOUT \ni.)Legend,\nii.) \niii.) Turbo King,\niv.) Williams ";
    let non_alcoholic = "\n3.)NON ALCHOLIC \ni.)Maltina, \nii.)Amstel,\niii.) Malta Gold,\niv.) Fayrouz ";

    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all("Welcome to the NBL'S; \nthe following are our high quality drinks."
        .as_bytes()).expect("write failed");
    file.write_all(lager.as_bytes()).expect("write failed");
    file.write_all(stout.as_bytes()).expect("write failed");
    file.write_all(non_alcoholic.as_bytes()).expect("write failed");



}