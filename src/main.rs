use std::io;
use std::fs;

// fn getNextLine(lines: Vec<String>, current_line: usize) => {

// }

struct Day {
    day: String,
    shifts: Vec<Shift>
}
struct Shift {
    shift: String,
    people: Vec<String>,
    hours: f32
}

struct Staff {
    name: String,
    hours: f32,
    boss: bool,
    relationship: bool,
    significant_other: String,
    hour_cap: f32,
    availability: Vec<Vec<String>>,
}

/*const SCHEDULE_TEMPLATE: Vec<Day> = vec![
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
*/

fn get_staff(data: String, schedule: &Vec<Day>) -> Vec<Staff> {
    let lines = data.split("#").collect::<Vec<_>>();
                
    let mut staff_members = Vec::<Vec<String>>::new();
    let mut staff_vec = Vec::<Staff>::new();

    for line in &lines {
        
        let mut person = line.split("\n").collect::<Vec<_>>();
        let mut placeholder = Vec::<String>::new();
        for mut x in &person {
            let mut new_x = x.to_string();
            let nex_x = new_x.pop();
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
            significant_other: if person[2] == "n" { person[3].to_string() } else { "".to_string() },
            hour_cap: if person[4] == "n" { 20.0 } else { person[5].to_string().trim().parse::<f32>().unwrap() },
            availability: Vec::<Vec<String>>::new()
        };

        let mut person_availability = Vec::<Vec<String>>::new();

        let mut index: usize = 5;

        for day in schedule {
            let mut day_availability = Vec::<String>::new();
            for shift in &day.shifts {
                day_availability.push(person[index].to_string());
                index = index + 1;
            }
            person_availability.push(day_availability);
        }

        staff_member.availability = person_availability;
        staff_vec.push(staff_member);



        //staff_member.availability = person_availability;
        println!("working");
        //println!("{:?}", person_availability);
    }
    return staff_vec
}

fn print_schedule(schedule: &Vec<Day>) {
    println!("----------------------schedule------------------------");
    for i in 0..schedule.len() {
        println!("------{}------", schedule[i].day);
        for j in 0..schedule[i].shifts.len() {
            println!(" {}", schedule[i].shifts[j].shift);
            for person in 0..schedule[i].shifts[j].people.len() {
                if schedule[i].day == "saturday".to_string() || schedule[i].day == "sunday".to_string() || schedule[i].shifts[j].shift != "morning".to_string() {
                    if person == 0 {
                        println!("    ID: {}", schedule[i].shifts[j].people[person]);
                    }else if person == 1 {
                        println!("    BOS: {}", schedule[i].shifts[j].people[person]);
                    }else{
                        println!("    OA: {}", schedule[i].shifts[j].people[person]);
                    }
                }else {
                    println!("    ID: {}", schedule[i].shifts[j].people[person]);
                }
            }
        }
    }
}

fn main() {
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

                let mut staff = Vec::<Staff>::new();
        
                //read data from file
                let data = fs::read_to_string("staff.txt").expect("unable to read file");
                //split data into a Vector of each line

                let mut schedule = vec![
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

                let mut staff_members = get_staff(data, &schedule);

                //scheduling algorithm

                for run in 0..2 {
                    if run == 0 {
                        for i in 0..schedule.len() {
                            for j in 0..schedule[i].shifts.len() {
                                if schedule[i].day == "saturday".to_string() || schedule[i].day == "sunday".to_string() || schedule[i].shifts[j].shift != "morning".to_string() && run == 0 {
                                    for person in &mut staff_members {
                                        if person.boss == true && person.availability[i][j] == 'a'.to_string() && person.hours <= person.hour_cap - 6.5 {
                                            schedule[i].shifts[j].people.push(person.name.clone());
                                            person.hours = person.hours + schedule[i].shifts[j].hours;
                                            break;
                                        }
                                    }
                                    if schedule[i].shifts[j].people.len() == 0 {
                                        for person in &mut staff_members {
                                            if person.boss == true && (person.availability[i][j] == 'a'.to_string() || person.availability[i][j] == 'm'.to_string()) && person.hours <= person.hour_cap - 6.5 {
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
                                        if person.availability[i][j] == 'a'.to_string() && person.hours <= person.hour_cap - 6.5 && !schedule[i].shifts[j].people.contains(&person.name.clone()) && !schedule[i].shifts[j].people.contains(&person.significant_other.clone()) {
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
                                        if person.availability[i][j] == 'a'.to_string() && person.hours <= person.hour_cap - 6.5 && !schedule[i].shifts[j].people.contains(&person.name.clone()) && !schedule[i].shifts[j].people.contains(&person.significant_other.clone()){
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
                    }else{
                        for i in (0..schedule.len()).rev() {
                            for j in 0..schedule[i].shifts.len() {
                                if schedule[i].day == "saturday".to_string() || schedule[i].day == "sunday".to_string() || schedule[i].shifts[j].shift != "morning".to_string(){
                                    let mut threshold = 3 + run;
                                    for person in &mut staff_members {
                                        if person.availability[i][j] == 'a'.to_string() && person.hours <= person.hour_cap - 6.5 && !schedule[i].shifts[j].people.contains(&person.name.clone()) && !schedule[i].shifts[j].people.contains(&person.significant_other.clone()){
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


                print_schedule(&schedule);

                let mut choice = String::new();

                while choice != "1".to_string() && choice != "2".to_string() && choice != "3".to_string() {
                    println!("Select an option:");
                    println!("1. generate another weekly schedule.");
                    println!("2. save schedule to file");
                    println!("3. back");
    
                    
                    io::stdin().read_line(&mut choice).expect("can't read line");
    
                    if choice.trim() == "1"{
                        continue 'generate;
                    }else if choice.trim() == "2" {
                        println!("just saved to file");
                        continue;
                    }else if choice.trim() == "3" {
                        break 'generate;
                    }else{
                        println!("Please choose one of the options.");
                        continue;
                    }
                }
        
            }
        }else if choice.trim() == "2" {
            let mut staff_choice: String = String::new();

            let mut staff_raw = fs::read_to_string("staff.txt").expect("Unable to read file");
            let mut staff_raw = staff_raw.split("#").collect::<Vec<&str>>();
            
            let mut staff = Vec::<Staff>::new();

            for person in &staff_raw {
                let mut person = person.split("\n").collect::<Vec<&str>>();
                let mut staff_member = Staff { 
                    name: person[0].to_string(),
                    hours: 0.0,
                    boss: if person[1] == "y" {true} else {false},
                    relationship: if person[2] == "y" {true} else {false},
                    significant_other: person[3].to_string(),
                    hour_cap: person[4].parse::<f32>().unwrap(),
                    availability: {
                        Vec::<Vec<String>>::new()
                    },
                 };
            }

            loop {
                print!("\x1B[2J");
                println!("{}", staff_raw.len().to_string());
                println!("Please choose an option:");
                println!("1. search staff");
                println!("2. back");
    
                io::stdin().read_line(&mut staff_choice).expect("unable to read line");

                if staff_choice.trim() == "2" {
                    break;
                } else if staff_choice.trim() == "1" {
                    println!("going to search for user");
                } else {
                    println!("Please choose a valid option.");
                }
            }




            /*
                staff_raw = readfile(staff.txt)
                split file into vector of staff members (split by #)
                staff = []
                staffmember = {}
                staffmember



            */
            
        } else if choice.trim() == "3" {
            break;
        }else {
            println!("Please choose a valid option.");
        }
    }

}