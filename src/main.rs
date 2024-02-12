use std::io;
use std::fs;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;

// fn getNextLine(lines: Vec<String>, current_line: usize) => {

// }
#[derive(Default, Clone)]
struct Day {
    day: String,
    shifts: Vec<Shift>
}

#[derive(Default, Clone)]
struct Shift {
    shift: String,
    people: Vec<String>,
    hours: f32
}

#[derive(Default, Clone)]
struct Staff {
    name: String,
    hours: f32,
    boss: bool,
    relationship: bool,
    significant_other: String,
    hour_cap: f32,
    availability: Vec<Vec<String>>,
}

fn get_staff(data: String, schedule: &Vec<Day>) -> Vec<Staff> {
    let lines = data.split("#").collect::<Vec<_>>();
                
    let mut staff_members = Vec::<Vec<String>>::new();
    let mut staff_vec = Vec::<Staff>::new();

    for line in 0..lines.len() {
        let mut person = lines[line].split("\n").collect::<Vec<_>>();
        if line == (lines.len()-1) {
            person.push("");
        }
        let mut placeholder = Vec::<String>::new();
        println!("{:?}", person);
        for i in 0..person.len() {
            let mut new_x = person[i].to_string();
            println!("{}", new_x);
            if line != (lines.len()-1) {
                new_x.pop();
            }else {
                if i != (person.len()-1) {
                    new_x.pop();
                }
            }
            
            placeholder.push(new_x);
        }
        placeholder.pop().expect("Could not get last element");
        if placeholder[0] == "" { 
            placeholder.remove(0);
        }
        //println!("{:?}", person);
        staff_members.push(placeholder);
    }

    println!("{:?}", staff_members);

    for person in &staff_members {
        let mut staff_member = Staff {
            name: person[0].to_string(),
            hours: 0.0,
            boss: if person[1] == "n" { false } else { true },
            relationship: if person[2] == "n" { false } else { true },
            significant_other: person[3].to_string(),
            hour_cap: if person[4] == "n" { 20.0 } else { person[5].to_string().trim().parse::<f32>().unwrap() },
            availability: Vec::<Vec<String>>::new()
        };

        let mut person_availability = Vec::<Vec<String>>::new();

        let mut index: usize = 6;

        for day in schedule {
            let mut day_availability = Vec::<String>::new();
            for shift in &day.shifts {
                day_availability.push(person[index].to_string());
                index = index + 1;
            }
            person_availability.push(day_availability);
        }

        println!("availability: ");
        println!("{:?}", person_availability);

        staff_member.availability = person_availability;
        staff_vec.push(staff_member);



        //staff_member.availability = person_availability;
        println!("working");
        //println!("{:?}", person_availability);
    }
    return staff_vec
}
//schedule_str.push_str();
fn print_schedule(schedule: &Vec<Day>, staff_members: &Vec<Staff>) -> String {
    let mut schedule_str: String = String::new();
    schedule_str.push_str("----------------------schedule------------------------\n");
    for i in 0..schedule.len() {
        schedule_str.push_str("------");
        schedule_str.push_str(&schedule[i].day);
        schedule_str.push_str("------\n");
        //println!("------{}------", schedule[i].day);
        for j in 0..schedule[i].shifts.len() {
            schedule_str.push_str(" ");
            schedule_str.push_str(&schedule[i].shifts[j].shift);
            schedule_str.push_str("\n");
            for person in 0..schedule[i].shifts[j].people.len() {
                if schedule[i].day == "saturday".to_string() || schedule[i].day == "sunday".to_string() || schedule[i].shifts[j].shift != "morning".to_string() {
                    if person == 0 {
                        schedule_str.push_str("    ID: ");
                        schedule_str.push_str(&schedule[i].shifts[j].people[person]);
                        schedule_str.push_str("\n");
                    }else if person == 1 {
                        schedule_str.push_str("    BOS: ");
                        schedule_str.push_str(&schedule[i].shifts[j].people[person]);
                        schedule_str.push_str("\n");
                    }else{
                        schedule_str.push_str("    OA: ");
                        schedule_str.push_str(&schedule[i].shifts[j].people[person]);
                        schedule_str.push_str("\n");
                    }
                }else {
                    schedule_str.push_str("    ID: ");
                    schedule_str.push_str(&schedule[i].shifts[j].people[person]);
                    schedule_str.push_str("\n");
                }
            }
        }
    }

    schedule_str.push_str("\n");
    schedule_str.push_str("----------------------staff hours------------------------\n");
    let mut average = 0.0;

    for person in staff_members {
        schedule_str.push_str(&person.name);
        schedule_str.push_str(": ");
        schedule_str.push_str(&person.hours.to_string());
        schedule_str.push_str("\n");
        average = average + person.hours;
    }
    schedule_str.push_str("\n");
    schedule_str.push_str("Average staff hours: ");
    schedule_str.push_str(&(average / staff_members.len() as f32 ).to_string());
    schedule_str
}

fn not_working(day: &Vec<Shift>, name: &str) -> bool {
    let mut not_working: bool = true;
    for shift in day {
        if shift.people.contains(&name.to_string()) {
            not_working = false;
            break;
        }
    }
    return not_working;
}

fn print_staff(staff: &Vec<Staff>) {
    for person in staff {
        println!("--{}--", person.name);
        println!("boss: {}", person.boss);
        println!("hour cap: {}", person.hour_cap);
        if person.relationship {
            println!("significant other: {}", person.significant_other);
        }
        println!("availability: |    Monday     |    Tuesday    |   Wednesday   |   Thursday    |    Friday     |   Saturday    |    Sunday     |");
        for i in 0..3 {
            if i == 0 {
                print!("morning:     ");
            }
            if i == 1 {
                print!("afternoon:   ");
            }
            if i == 2 {
                print!("evening:     ");
            }
            for day in 0..person.availability.len() {
                if person.availability[day].len() >= (i + 1) {
                    if day == person.availability.len() - 1 {
                        println!("        {}       ", person.availability[day][i]);
                    }else {
                        print!("        {}       ", person.availability[day][i]);
                    }
                    
                }
            }
        }
        println!("");
        println!("");
    }
}

fn save_to_file(staff_members: &Vec<Staff>) {
    let mut new_file: String = "".to_owned();
    for staff in 0..staff_members.len() {
        new_file.push_str(&staff_members[staff].name);
        new_file.push_str("\r\n");
        if staff_members[staff].boss {
            new_file.push_str("y\r\n");
        }else{
            new_file.push_str("n\r\n");
        }
        if staff_members[staff].relationship {
            new_file.push_str("y\r\n");
        }else{
            new_file.push_str("n\r\n");
        }
        new_file.push_str(&staff_members[staff].significant_other);
        new_file.push_str("\r\n");
        if staff_members[staff].hour_cap < 20.0{
            new_file.push_str("y\r\n");
        }else{
            new_file.push_str("n\r\n");
        }
        new_file.push_str(&staff_members[staff].hour_cap.to_string());
        new_file.push_str("\r\n");
        for day in &staff_members[staff].availability {
            for shift in day {
                new_file.push_str(&shift.to_string());
                new_file.push_str("\r\n");
            }
        }
        if staff != (staff_members.len()-1) {
            new_file.push_str("#\r\n");
        }
    }
    fs::write("./staff.txt", new_file).expect("couldn't write to file");
}

fn main() {
    let mut schedule_template = vec![
        Day {
            day: "monday".to_string(),
            shifts: vec![
                Shift {
                    shift: "morning".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.0
                },
                Shift {
                    shift: "afternoon".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.5
                },
                Shift {
                    shift: "evening".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.5
                },
            ]
        },
        Day {
            day: "tuesday".to_string(),
            shifts: vec![
                Shift {
                    shift: "morning".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.0
                },
                Shift {
                    shift: "afternoon".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.5
                },
                Shift {
                    shift: "evening".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.5
                },
            ]
        },
        Day {
            day: "wednesday".to_string(),
            shifts: vec![
                Shift {
                    shift: "morning".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.0
                },
                Shift {
                    shift: "afternoon".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.5
                },
                Shift {
                    shift: "evening".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.5
                },
            ]
        },
        Day {
            day: "thursday".to_string(),
            shifts: vec![
                Shift {
                    shift: "morning".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.0
                },
                Shift {
                    shift: "afternoon".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.5
                },
                Shift {
                    shift: "evening".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.5
                },
            ]
        },
        Day {
            day: "friday".to_string(),
            shifts: vec![
                Shift {
                    shift: "morning".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.0
                },
                Shift {
                    shift: "afternoon".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.5
                },
                Shift {
                    shift: "evening".to_string(),
                    people: Vec::<String>::new(),
                    hours: 4.5
                },
            ]
        },
        Day {
            day: "saturday".to_string(),
            shifts: vec![
                Shift {
                    shift: "morning".to_string(),
                    people: Vec::<String>::new(),
                    hours: 6.5
                },
                Shift {
                    shift: "evening".to_string(),
                    people: Vec::<String>::new(),
                    hours: 6.5
                }
            ]
        },
        Day {
            day: "sunday".to_string(),
            shifts: vec![
                Shift {
                    shift: "morning".to_string(),
                    people: Vec::<String>::new(),
                    hours: 5.5
                },
                Shift {
                    shift: "evening".to_string(),
                    people: Vec::<String>::new(),
                    hours: 6.5
                }
            ]
        }
    ];

    loop {
        let mut choice: String = String::new();

        //clear screen
        print!("\x1B[2J");
        println!("Select an option:");
        println!("1. generate weekly schedule");
        println!("2. view/edit staff file");
        println!("3. exit");
        io::stdin().read_line(&mut choice).expect("Could not read input");
    
        if choice.trim() == "1" {
            'generate: loop {
                print!("\x1B[2J");
        
                //read data from file
                let data = fs::read_to_string("staff.txt").expect("unable to read file");
                //split data into a Vector of each line

                let mut schedule = schedule_template.clone();

                let mut staff_members = get_staff(data, &schedule);

                let mut rng = StepRng::new(2, 13);
                let mut irs = Irs::default();

                //scheduling algorithm

                for run in 0..2 {
                    if run == 0 {
                        for i in 0..schedule.len() {
                            for j in 0..schedule[i].shifts.len() {
                                if schedule[i].day == "saturday".to_string() || schedule[i].day == "sunday".to_string() || schedule[i].shifts[j].shift != "morning".to_string() && run == 0 {
                                    for person in &mut staff_members {
                                        println!("{}", &person.name.clone());
                                        println!("{}", &person.significant_other.clone());
                                        if not_working(&schedule[i].shifts, &person.name) && person.boss == true && person.availability[i][j] == 'a'.to_string() && person.hours <= person.hour_cap - 6.5 && !schedule[i].shifts[j].people.contains(&person.name.clone()) && !schedule[i].shifts[j].people.contains(&person.significant_other.clone()) {
                                            schedule[i].shifts[j].people.push(person.name.clone());
                                            person.hours = person.hours + schedule[i].shifts[j].hours;
                                            break;
                                        }
                                    }
                                    if schedule[i].shifts[j].people.len() == 0 {
                                        for person in &mut staff_members {
                                            if person.boss == true && (person.availability[i][j] == 'a'.to_string() || person.availability[i][j] == 'm'.to_string()) && person.hours <= person.hour_cap - 6.5 && !schedule[i].shifts[j].people.contains(&person.name.clone()) && !schedule[i].shifts[j].people.contains(&person.significant_other.clone()) {
                                                schedule[i].shifts[j].people.push(person.name.clone());
                                                person.hours = person.hours + schedule[i].shifts[j].hours;
                                                break;
                                            }
                                        }
                                    }
                                }
                                if schedule[i].day != "saturday".to_string() && schedule[i].day != "sunday".to_string() && schedule[i].shifts[j].shift == "morning".to_string() && schedule[i].shifts[j].people.len() < 1 {
                                    let mut added: bool = false;
                                    for person in &mut staff_members {
                                        if not_working(&schedule[i].shifts, &person.name) && person.availability[i][j] == 'a'.to_string() && person.hours <= person.hour_cap - 6.5 && !schedule[i].shifts[j].people.contains(&person.name.clone()) && !schedule[i].shifts[j].people.contains(&person.significant_other.clone()) {
                                            schedule[i].shifts[j].people.push(person.name.clone());
                                            person.hours = person.hours + schedule[i].shifts[j].hours;
                                            added = true;
                                            break;
                                        }
                                    }
                                    if added == false {
                                        for person in &mut staff_members {
                                            if (person.availability[i][j] == 'a'.to_string() || person.availability[i][j] == 'm'.to_string()) && person.hours <= person.hour_cap - 6.5 && !schedule[i].shifts[j].people.contains(&person.name.clone()) && !schedule[i].shifts[j].people.contains(&person.significant_other.clone()){
                                                schedule[i].shifts[j].people.push(person.name.clone());
                                                person.hours = person.hours + schedule[i].shifts[j].hours;
                                                break;
                                            }
                                        }
                                    }
                                }
                                if schedule[i].day == "saturday".to_string() || schedule[i].day == "sunday".to_string() || schedule[i].shifts[j].shift != "morning".to_string(){
                                    let mut threshold = 3 + run;
                                    for person in &mut staff_members {
                                        if not_working(&schedule[i].shifts, &person.name) && person.availability[i][j] == 'a'.to_string() && person.hours <= person.hour_cap - 6.5 && !schedule[i].shifts[j].people.contains(&person.name.clone()) && !schedule[i].shifts[j].people.contains(&person.significant_other.clone()){
                                            schedule[i].shifts[j].people.push(person.name.clone());
                                            person.hours = person.hours + schedule[i].shifts[j].hours;
                                            if schedule[i].shifts[j].people.len() >= threshold {
                                                break;
                                            }
                                        }
                                    }
                                    if schedule[i].shifts[j].people.len() < 1 {
                                        for person in &mut staff_members {
                                            if (person.availability[i][j] == 'a'.to_string() || person.availability[i][j] == 'm'.to_string()) && person.hours <= person.hour_cap - 6.5 && !schedule[i].shifts[j].people.contains(&person.name.clone()) && !schedule[i].shifts[j].people.contains(&person.significant_other.clone()){
                                                schedule[i].shifts[j].people.push(person.name.clone());
                                                person.hours = person.hours + schedule[i].shifts[j].hours;
                                                if schedule[i].shifts[j].people.len() >= threshold {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            irs.shuffle(&mut staff_members, &mut rng);
                        }
                    }else{
                        for i in (0..schedule.len()).rev() {
                            for j in 0..schedule[i].shifts.len() {
                                if schedule[i].day == "saturday".to_string() || schedule[i].day == "sunday".to_string() || schedule[i].shifts[j].shift != "morning".to_string(){
                                    let mut threshold = 3 + run;
                                    for person in &mut staff_members {
                                        if not_working(&schedule[i].shifts, &person.name) && person.availability[i][j] == 'a'.to_string() && person.hours <= person.hour_cap - 6.5 && !schedule[i].shifts[j].people.contains(&person.name.clone()) && !schedule[i].shifts[j].people.contains(&person.significant_other.clone()){
                                            schedule[i].shifts[j].people.push(person.name.clone());
                                            person.hours = person.hours + schedule[i].shifts[j].hours;
                                            if schedule[i].shifts[j].people.len() >= threshold {
                                                break;
                                            }
                                        }
                                    }
                                    if schedule[i].shifts[j].people.len() < 1 {
                                        for person in &mut staff_members {
                                            if (person.availability[i][j] == 'a'.to_string() || person.availability[i][j] == 'm'.to_string()) && person.hours <= person.hour_cap - 6.5 && !schedule[i].shifts[j].people.contains(&person.name.clone()) && !schedule[i].shifts[j].people.contains(&person.significant_other.clone()){
                                                schedule[i].shifts[j].people.push(person.name.clone());
                                                person.hours = person.hours + schedule[i].shifts[j].hours;
                                                if schedule[i].shifts[j].people.len() >= threshold {
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }


                let schedule_str = print_schedule(&schedule, &staff_members);
                println!("{}", schedule_str);

                let mut choice = String::new();

                while choice != "1".to_string() && choice != "2".to_string() && choice != "3".to_string() {
                    println!("Select an option:");
                    println!("1. generate another weekly schedule.");
                    println!("2. save schedule to file");
                    println!("3. back");
    
                    
                    io::stdin().read_line(&mut choice).expect("can't read line");
    
                    if choice.trim() == "1"{
                        choice = "".to_string();
                        continue 'generate;
                    }else if choice.trim() == "2" {
                        fs::write("./schedule.txt", schedule_str.clone()).expect("couldn't write to file");
                        println!("File saved");
                        choice = "".to_string();
                        continue;
                    }else if choice.trim() == "3" {
                        choice = "".to_string();
                        break 'generate;
                    }else{
                        println!("Please choose one of the options.");
                        choice = "".to_string();
                        continue;
                    }
                }
        
            }
        }else if choice.trim() == "2" {
            let mut staff_choice: String = String::new();

            let data = fs::read_to_string("staff.txt").expect("unable to read file");

            let mut staff_members = get_staff(data, &schedule_template);

            loop {
                print!("\x1B[2J");
                print_staff(&staff_members);
                println!("");
                println!("Please choose an option:");
                println!("1. edit staff member");
                println!("2. add new staff member");
                println!("3. back");
    
                io::stdin().read_line(&mut staff_choice).expect("unable to read line");

                

                if staff_choice.trim() == "3" {
                    staff_choice = "".to_string();
                    break;
                } else if staff_choice.trim() == "1" {
                    println!("working");

                    println!("\x1B[2J");

                    let mut staff_member: String = String::new();

                    println!("Search by name (case sensitive): ");

                    io::stdin().read_line(&mut staff_member).expect("could not read line");

                    let mut index: i8 = -1;

                    for staff in 0..staff_members.len() {
                        if staff_members[staff].name == staff_member.trim() {
                            index = staff as i8;
                            staff_choice = "".to_string();
                            break;
                        }
                    }


                    if index == -1 {
                       println!("Could not find staff member."); 
                       //thread::sleep(Duration::from_millis(500));
                       staff_choice = "".to_string();
                       continue;
                    }else {
                        loop {
                            println!("\x1B[2J");
                            print_staff(&vec![staff_members[index as usize].clone()]);
    
                            let mut staff_options: String = String::new();
    
                            println!("Editing options:");
                            println!("1. Change name");
                            println!("2. Change BOS status");
                            println!("3. Change hour cap");
                            println!("4. Change significant other");
                            println!("5. Change availability");
                            println!("6. back");
    
                            io::stdin().read_line(&mut staff_options).expect("could not read line");
    
                            if staff_options.trim() == "1" {
                                let mut new_value: String = String::new();
                                println!("Enter new name: ");
                                io::stdin().read_line(&mut new_value).expect("could not read line");
    
                                staff_members[index as usize].name = new_value.trim().to_string();
                                save_to_file(&staff_members);
                                staff_options = "".to_string();
                                
                            }else if staff_options.trim() == "2" {
                                let mut new_value: String = String::new();
                                println!("Enter new BOS status(y/n): ");
                                io::stdin().read_line(&mut new_value).expect("could not read line");
    
                                if new_value.trim().to_string() == "y" {
                                    staff_members[index as usize].boss = true;
                                }else{
                                    staff_members[index as usize].boss = false;
                                }
                                
                                save_to_file(&staff_members);
                                staff_options = "".to_string();
    
                            }else if staff_options.trim() == "3" {
                                let mut new_value: String = String::new();
                                println!("Enter new hour_cap with one decimal point(Ex: 15.0): ");
                                io::stdin().read_line(&mut new_value).expect("could not read line");
    
                                staff_members[index as usize].hour_cap = new_value.to_string().trim().parse::<f32>().unwrap();
                                save_to_file(&staff_members);
                                staff_options = "".to_string();
                                
                            }else if staff_options.trim() == "4" {
                                let mut new_value: String = String::new();
                                println!("Enter new significant other (case sensitive, must be an existing name in staff file. Enter NOBODY if it's nobody): ");
                                io::stdin().read_line(&mut new_value).expect("could not read line");
    
                                staff_members[index as usize].significant_other = new_value.trim().to_string();
                                if new_value.trim().to_string() == "NOBODY" {
                                    staff_members[index as usize].relationship = false;
                                }else {
                                    staff_members[index as usize].relationship = true;
                                }
                                save_to_file(&staff_members);
                                staff_options = "".to_string();
                            }else if staff_options.trim() == "5" {
                                let mut day: String = String::new();
                                println!("Enter numeric day of the week(IE: Monday = 1): ");
                                io::stdin().read_line(&mut day).expect("could not read line");

                                let mut shift: String = String::new();
                                println!("Enter numeric shift num(IE: Morning = 1, Afternoon = 2) ");
                                io::stdin().read_line(&mut shift).expect("could not read line");

                                let mut new_value: String = String::new();
                                println!("Enter new availability status(a=available, n=unavailable, m=available, but prefers not to work): ");
                                io::stdin().read_line(&mut new_value).expect("could not read line");
    
                                staff_members[index as usize].availability[day.to_string().trim().parse::<usize>().unwrap()-1][shift.to_string().trim().parse::<usize>().unwrap()-1] = new_value.trim().to_string();
                                save_to_file(&staff_members);
                                staff_options = "".to_string();
                            }else if staff_options.trim() == "6" {
                                staff_options = "".to_string();
                                break;
                            }else {
                                println!("Please choose one of the options");
                                staff_options = "".to_string();
                                continue;
                            }
                        }

                    }
                    staff_choice = "".to_string();

                } else if staff_choice.trim() == "2"{
                    println!("Adding new staff member");
                    staff_choice = "".to_string();
                } else {
                    println!("Please choose a valid option");
                    staff_choice = "".to_string();
                }
            }
            
        } else if choice.trim() == "3" {
            break;
        }else {
            println!("Please choose a valid option.");
        }
    }

}