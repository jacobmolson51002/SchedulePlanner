use std::io;
use std::fs;

// fn getNextLine(lines: Vec<String>, current_line: usize) => {

// }

struct Time {
    day: String,
    availability: Box<Time>
}

struct Staff {
    name: String,
    hours: f32,
    boss: bool,
    relationship: bool,
    significant_other: String,
    hour_cap: f32,
    availability: Vec<Time>,
}

fn main() {
    loop {
        let mut choice: String = String::new();

        //clear screen
        //print!("\x1B[2J");
        println!("Select an option:");
        println!("1. generate weekly schedule");
        println!("2. view/edit staff file");
        println!("3. exit");
        io::stdin().read_line(&mut choice).expect("Could not read input");
    
        if choice.trim() == "1" {
            loop {

                struct Day {
                    day: String,
                    shifts: Vec<Shift>
                }
                struct Shift {
                    shift: String,
                    people: Vec<String>,
                    hours: f32
                }

                let mut staff = Vec::<Staff>::new();
                // let mut schedule = vec![
                //     vec!["monday", vec!["morning", Vec::new(), 4.0], vec!["afternoon", Vec::new(), 4.5], vec!["evening", Vec::new(), 4.5]],
                //     vec!["tuesday", vec!["morning", Vec::new(), 4.0], vec!["afternoon", Vec::new(), 4.5], vec!["evening", Vec::new(), 4.5]],
                //     vec!["wednesday", vec!["morning", Vec::new(), 4.0], vec!["afternoon", Vec::new(), 4.5], vec!["evening", Vec::new(), 4.5]],
                //     vec!["thursday", vec!["morning", Vec::new(), 4.0], vec!["afternoon", Vec::new(), 4.5], vec!["evening", Vec::new(), 4.5]],
                //     vec!["friday", vec!["morning", Vec::new(), 4.0], vec!["afternoon", Vec::new(), 4.5], vec!["evening", Vec::new(), 4.5]],
                //     vec!["saturday", vec!["morning", Vec::new(), 6.5], vec!["evening", Vec::new(), 6.5]],
                //     vec!["sunday", vec!["morning", Vec::new(), 5.5], vec!["evening", Vec::new(), 6.5]],
                // ];

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
        
                //read data from file
                let data = fs::read_to_string("staff.txt").expect("unable to read file");
                //split data into a Vector of each line
                let lines = data.split("#").collect::<Vec<_>>();
                
                let mut staff_members = Vec::<Vec<&str>>::new();

                for line in &lines {
                    
                    let mut person = line.split("\n").collect::<Vec<_>>();
                    person.pop().expect("Could not get last element");
                    if person[0] == "" { 
                        person.remove(0);
                    }
                    staff_members.push(person);
                }

                for person in &staff_members {
                    let mut staff_member = Staff {
                        name: person[0].to_string(),
                        hours: 0.0,
                        boss: if person[1] == "n" { false } else { true },
                        relationship: if person[2] == "n" { false } else { true },
                        significant_other: if person[2] == "n" { person[3].to_string() } else { "".to_string() },
                        hour_cap: if person[4] == "n" { 20.0 } else { person[5].to_string().trim().parse::<f32>().unwrap() },
                        availability: Vec::<Time>::new()
                    };

                    let mut person_availability = Vec::<Time>::new();

                    staff_member.availability = person_availability;

                    for day in schedule {
                        for shift in day.shifts {
                            
                        }
                    }





                }

                break;
                
                //initiate current_line variable
                // let current_line: usize = 0;

                // for line in &lines {

                // }
        
            }
        }else if choice.trim() == "2" {
            let mut staff_choice: String = String::new();

            let mut staff_raw = fs::read_to_string("staff.txt").expect("Unable to read file");
            let mut staff_raw = staff_raw.split("#").collect::<Vec<&str>>();
            
            let mut staff = Vec::<Staff>::new();

            struct Time {
                day: String,
                availability: Box<Time>
            }

            struct Staff {
                name: String,
                boss: bool,
                relationship: bool,
                significant_other: String,
                hour_cap: u16,
                availability: Vec<Time>,
                
            }

            for person in &staff_raw {
                let mut person = person.split("\n").collect::<Vec<&str>>();
                let mut staff_member = Staff { 
                    name: person[0].to_string(),
                    boss: if person[1] == "y" {true} else {false},
                    relationship: if person[2] == "y" {true} else {false},
                    significant_other: person[3].to_string(),
                    hour_cap: person[4].parse::<u16>().unwrap(),
                    availability: {
                        Vec::<Time>::new()
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