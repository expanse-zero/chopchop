use geo::polygon;

fn main() {
    let poly = polygon![
        (x: -111., y: 45.),
        (x: -111., y: 41.),
        (x: -104., y: 41.),
        (x: -104., y: 45.),
    ];
    let poly2 = polygon![
        (x: -123., y: 34.),
        (x: -144., y: 34.),
        (x: -120., y: 41.),
        (x: -123., y: 41.),
    ];
    let multi = vec![poly, poly2];
    println!("{:?}", multi);
}
