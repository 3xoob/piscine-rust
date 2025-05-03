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

pub fn check_for_securities(mall: &mut Mall, mut guards: HashMap<String, Guard>) {
    let mut mall_area = 0;

    for (_floor_name, floor) in &mall.floors {
        for (_store_name, store) in &floor.stores {
            mall_area += store.square_meters;
        }
    }

    let guards_needed = (mall_area + 199) / 200;
    let mut current_guards = mall.guards.len() as u64;

    // Add guards as needed
    for (name, guard) in guards {
        if current_guards < guards_needed {
            mall.guards.insert(name, guard);
            current_guards += 1;
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
