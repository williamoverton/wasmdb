wit_bindgen_rust::import!("../../host.wit");

fn main() {
    let args = host::get_args();
    let num = args[1].parse::<f64>().unwrap();

    host::print(format!("Hello {}", args[0]).as_str());

    for i in 0..1000 {
        host::upsert(
            format!("{}" , i).as_str(),
            vec![
                ("double", format!("{}", i * 2).as_str()),
                ("half", format!("{}", i / 2).as_str()),
            ].as_slice(),
        );
    }

    println!("{:?}", host::get("420").1);

    // host::upsert(
    //     "hello",
    //     vec![
    //         ("name", "jeff")
    //     ].as_slice(),
    // );

    // host::upsert(
    //     "hello",
    //     vec![
    //         ("surname", "frank")
    //     ].as_slice(),
    // );

    // host::upsert(
    //     "hello",
    //     vec![
    //         ("surname", "johnson")
    //     ].as_slice(),
    // );

    // let values = host::get_all();

    // println!("{:?}", values);
}
