wit_bindgen_rust::import!("../../host.wit");

fn main() {
    let args = host::get_args();

    for i in 0..1000 {
        host::upsert(
            format!("{}", i).as_str(),
            vec![
                ("double", format!("{}", i * 2).as_str()),
                ("half", format!("{}", i / 2).as_str()),
            ]
            .as_slice(),
        );
    }

    // println!("{:?}", host::get(&args[0]).1);

    return_record(host::get(&args[0]));

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

fn return_record(record: (String, Vec<(String, String)>)) {
    let vals = record
        .1
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect::<Vec<_>>();
    let val_slice = vals.as_slice();

    let req = (record.0.as_str(), val_slice);

    host::return_record(req);
}