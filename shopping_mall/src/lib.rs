use std::collections::HashMap;
mod mall;
use mall::{Employee, Guard, Mall, Store};

pub fn biggest_store(mall: &Mall) -> &Store {
    mall.floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .max_by_key(|store| store.square_meters)
        .expect("Mall must have at least one store")
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<&Employee> {
    let mut max_salary = 0.0;
    let mut highest_paid = Vec::new();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for employee in store.employees.values() {
                if employee.salary > max_salary {
                    max_salary = employee.salary;
                }
            }
        }
    }

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            for employee in store.employees.values() {
                if (employee.salary - max_salary).abs() < f64::EPSILON {
                    highest_paid.push(employee);
                }
            }
        }
    }

    highest_paid
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    let mut total = mall.guards.len();

    for floor in mall.floors.values() {
        for store in floor.stores.values() {
            total += store.employees.len();
        }
    }

    total
}

pub fn check_for_securities(mall: &mut Mall, guards: HashMap<String, Guard>) {
    let total_size: u64 = mall
        .floors
        .values()
        .flat_map(|floor| floor.stores.values())
        .map(|store| store.square_meters)
        .sum();

    let required_guards = (total_size as f64 / 200.0).ceil() as usize;

    if mall.guards.len() < required_guards {
        for (name, guard) in guards {
            if mall.guards.len() >= required_guards {
                break;
            }
            mall.hire_guard(name, guard);
        }
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for employee in store.employees.values_mut() {
                let hours = employee.working_hours.1 - employee.working_hours.0;
                if hours >= 10 {
                    employee.raise(employee.salary * 0.10);
                } else {
                    employee.cut(employee.salary * 0.10);
                }
            }
        }
    }
}
