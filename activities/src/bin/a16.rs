// given a database of students(name, and locker number) print the name and locker number for each student
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

fn read_locker_number() -> u32 {
    println!("please input a locker number!");
    let mut input = String::new();
    loop {
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("could't read line!");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please input only integers between 0..300");
                continue;
            }
        };
        if input > 300 {
            println!("Please input a number smaller than 300");
            continue;
        } else {
            println!("input: {}", input);
            return input;
        }
    }
}

fn main() {
    let mut students: Vec<StudentLockers> = Vec::new();

    students.push(StudentLockers {
        student_name: "Djibril".to_owned(),
        locker_no: Some(0),
    });
    students.push(StudentLockers {
        student_name: "Fane".to_owned(),
        locker_no: Some(1),
    });
    students.push(StudentLockers {
        student_name: "Lili".to_owned(),
        locker_no: Some(2),
    });
    students.push(StudentLockers {
        student_name: "Vijelie".to_owned(),
        locker_no: Some(3),
    });
    students.push(StudentLockers {
        student_name: "Spoitoru".to_owned(),
        locker_no: None,
    });
    students.push(StudentLockers {
        student_name: "Rene".to_owned(),
        locker_no: None,
    });
    students.push(StudentLockers {
        student_name: "Menolau".to_owned(),
        locker_no: None,
    });

    // creating a vector of current assigned locker numbers
    let mut current_assigned_lockers: Vec<u32> = Vec::new();
    let mut new_students: Vec<StudentLockers> = Vec::new();

    // create the current_assigned_lockers vector
    for student in &students {
        match student.locker_no {
            Some(locker) => current_assigned_lockers.push(locker),
            None => (),
        }
    }

    for student in &students {
        match student.locker_no {
            Some(locker) => {
                println!(
                    "student: {} has assigned {} for locker",
                    student.student_name, locker
                );
                new_students.push(StudentLockers {
                    student_name: student.student_name.to_owned(),
                    locker_no: Some(locker),
                });
            }
            None => {
                println!(
                    "student: {} doesn't have assigned a locker",
                    student.student_name
                );
                println!("Do you want to assign a new locker number now? y/n");
                let answer = validate_read_input();
                match answer {
                    false => {
                        new_students.push(StudentLockers {
                            student_name: student.student_name.to_owned(),
                            locker_no: None,
                        });
                    }
                    true => loop {
                        let new_locker = read_locker_number();
                        let mut found_locker = false;
                        for num in &current_assigned_lockers {
                            if new_locker == *num {
                                println!("{} already assigned!", num);
                                found_locker = true;
                                break;
                            }
                        }
                        if found_locker == false {
                            new_students.push(StudentLockers {
                                student_name: student.student_name.to_owned(),
                                locker_no: Some(new_locker),
                            });
                            current_assigned_lockers.push(new_locker);
                            break;
                        } else {
                            continue;
                        }
                    },
                }
            }
        }
    }

    println!("NEW MODIFIED STUDENTS VECTOR");

    for student in &new_students {
        match student.locker_no {
            Some(locker) => {
                println!(
                    "student: {} has assigned {} for locker",
                    student.student_name, locker
                )
            }
            None => {
                println!(
                    "student: {} doesn't have assigned a locker",
                    student.student_name
                );
            }
        }
    }
}
