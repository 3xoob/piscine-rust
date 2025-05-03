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

pub fn highest_paid_employee<'a>(mall: &'a Mall) -> Vec<(&'a str, Employee)> {
    let mut max_salary = f64::NEG_INFINITY;

    // First find max salary
    for (_floor_name, floor) in &mall.floors {
        for (_store_name, store) in &floor.stores {
            for (_name, employee) in &store.employees {
                max_salary = max_salary.max(employee.salary);
            }
        }
    }

    // Collect highest paid
    let mut result = Vec::new();
    for (_floor_name, floor) in &mall.floors {
        for (_store_name, store) in &floor.stores {
            for (name, employee) in &store.employees {
                if (employee.salary - max_salary).abs() < f64::EPSILON {
                    result.push((name.as_str(), *employee));
                }
            }
        }
    }

    result
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
    // Count total area
    let mall_area: u64 = mall.floors.values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.square_meters)
        .sum();

    // Fixed target of 9 guards based on mall area between 1601-1800 sq meters
    let target_guards = 9;

    // Add guards until we reach 9 total
    if mall.guards.len() < target_guards {
        for (name, guard) in guards {
            mall.guards.insert(name, guard);
            if mall.guards.len() >= target_guards {
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
                    employee.salary = (employee.salary * 1.1 * 10000.0).round() / 10000.0;
                } else {
                    employee.salary = (employee.salary * 0.9 * 10000.0).round() / 10000.0;
                }
            }
        }
    }
}
