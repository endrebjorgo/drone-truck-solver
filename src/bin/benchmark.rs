use drone_truck_solver::report::InstanceReport;

const INPUT_FILES: [&'static str; 8] = [
    "./assets/F_10.txt",
    "./assets/R_10.txt",
    "./assets/F_20.txt",
    "./assets/R_20.txt",
    "./assets/F_50.txt",
    "./assets/R_50.txt",
    "./assets/F_100.txt",
    "./assets/R_100.txt",
];

fn main() {
    for file_path in &INPUT_FILES {
        let report = InstanceReport::generate(file_path);
        report.print();
    }
}
