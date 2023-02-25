// given a database of students(name, and locker number) print the name and locker nober for each student
// if the student hasn't been assigned a locker no yet, ask the user for assigning a new locker no
// the new locker no must be in range 1..300 and it shouldn't have been assignet yet
use std::io;

struct StudentLockers {
    student_name: String,
    locker_no: Option<u32>,
}

fn validate_read_input() -> bool {
    let mut input = String::new();
    loop {
        io::stdin()
            .read_line(&mut input)
            .expect("couldn't read line!");
        let input_slice: &str = &input[..];
        match input_slice.trim() {
            "y" => return true,
            "Y" => return true,
            "n" => return false,
            "N" => return false,
            _ => {
                println!("Please input only y/Y/n/N");
                input.clear();
                continue;
            }
        }
    }
}

fn read_locker() {}

fn main() {
    let mut students: Vec<StudentLockers> = Vec::new();

    students.push(StudentLockers {
        student_name: "Djibril".to_owned(),
        locker_no: Some(7),
    });
    students.push(StudentLockers {
        student_name: "Fane".to_owned(),
        locker_no: Some(2),
    });
    students.push(StudentLockers {
        student_name: "Spoitoru".to_owned(),
        locker_no: None,
    });

    // creating a vector of current assigned locker numbers
    let mut current_assigned_lockers: Vec<u32> = Vec::new();
    for student in &students {
        match student.locker_no {
            Some(locker) => current_assigned_lockers.push(locker),
            None => (),
        }
    }

    // for mut student in students {
    //     match student.locker_no {
    //         Some(locker) => println!(
    //             "{:?} has assigned locker: {:?}",
    //             student.student_name, locker
    //         ),
    //         None => {
    //             println!(
    //                 "{:?} hasn't been assigned a locker yet.",
    //                 student.student_name
    //             );
    //             println!("Do you want to assign a new locker number now? y/n");
    //             let answer = validate_read_input();
    //             match answer {
    //                 true => {
    //                     student.locker_no = Some(5);
    //                 }
    //                 false => (),
    //             }
    //         }
    //     }
    // }
}
