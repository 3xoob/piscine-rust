pub mod mall;

// Re-export types at crate root
pub use mall::{Mall, Guard, Floor, Store, Employee};

use std::collections::HashMap;

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    let mut biggest_name = String::new();
    let mut biggest_store = Store::new(HashMap::<String, Employee>::new(), 0);

    for (_floor_name, floor) in &mall.floors {
        for (store_name, store) in &floor.stores {
            if store.square_meters > biggest_store.square_meters {
                biggest_store = store.clone();
                biggest_name = store_name.clone();
            }
        }
    }

    (biggest_name, biggest_store)
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(String, Employee)> {
    let mut highest_paid = Vec::new();
    let mut max_salary = 0.0;

    for (_floor_name, floor) in &mall.floors {
        for (_store_name, store) in &floor.stores {
            for (_employee_name, employee) in &store.employees {
                if employee.salary > max_salary {
                    max_salary = employee.salary;
                }
            }
        }
    }

    for (_floor_name, floor) in &mall.floors {
        for (_store_name, store) in &floor.stores {
            for (employee_name, employee) in &store.employees {
                if employee.salary == max_salary {
                    highest_paid.push((employee_name.clone(), *employee));
                }
            }
        }
    }

    highest_paid
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut counter = 0;

    for (_floor_name, floor) in &mall.floors {
        for (_store_name, store) in &floor.stores {
            counter += store.employees.len();
        }
    }

    counter += mall.guards.len();
    counter
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let mut mall_area = 0;

    for (_floor_name, floor) in &mall.floors {
        for (_store_name, store) in &floor.stores {
            mall_area += store.square_meters;
        }
    }

    let required_guards: usize = ((mall_area + 199) / 200).try_into().unwrap();
    let current_guards = mall.guards.len();

    if current_guards < required_guards {
        for (name, guard) in guards {
            if mall.guards.len() < required_guards {
                mall.guards.insert(name, guard);
            } else {
                break;
            }
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for (_floor_name, floor) in &mut mall.floors {
        for (_store_name, store) in &mut floor.stores {
            for (_employee_name, employee) in &mut store.employees {
                let work_hours = employee.working_hours.1 - employee.working_hours.0;
                if work_hours >= 10 {
                    employee.salary *= 1.1;
                } else {
                    employee.salary *= 0.9;
                }
            }
        }
    }
}
