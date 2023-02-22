#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let worker1 = Employee {
        position: Position::Worker,
        work_hours: 40,
    };
    // match worker1.position {
    //     Position::Manager => println!("Manager"),
    //     Position::Supervisor => println!("Supervisor"),
    //     Position::Worker => println!("Worker"),
    // }

    // with derive functionality
    print_employee(worker1);
    print_employee(worker1);
}
