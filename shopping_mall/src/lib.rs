pub mod mall;

use std::collections::HashMap;
use crate::mall::{Mall, Floor, Store, Employee, Guard};

pub fn biggest_store(mall: mall::Mall) -> mall::Store {
    let mut biggest = mall::Store::new(HashMap::new(), 0);

    for (_floor_name, floor) in mall.floors {
        for (_store_name, store) in floor.stores {
            if store.square_meters > biggest.square_meters {
                biggest = store;
            }
        }
    }

    biggest
}

pub fn highest_paid_employee(mall: mall::Mall) -> Vec<mall::Employee> {
    let mut highest_paid = Vec::new();

    for (_floor_name, floor) in mall.floors {
        for (_store_name, store) in floor.stores {
            for (_employee_name, employee) in store.employees {
                if highest_paid.is_empty() {
                    highest_paid.push(employee);
                } else if employee.salary == highest_paid[0].salary {
                    highest_paid.push(employee);
                } else if employee.salary > highest_paid[0].salary {
                    highest_paid[0] = employee;
                }
            }
        }
    }

    highest_paid
}

pub fn nbr_of_employees(mall: mall::Mall) -> usize {
    let mut counter = 0;

    for (_floor_name, floor) in mall.floors {
        for (_store_name, store) in floor.stores {
            counter += store.employees.len();
        }
    }

    counter += mall.guards.len();
    counter
}

pub fn check_for_securities(mall: &mut mall::Mall, guards: Vec<mall::Guard>) {
    let mut mall_area = 0;

    for (_floor_name, floor) in &mall.floors {
        for (_store_name, store) in &floor.stores {
            mall_area += store.square_meters;
        }
    }

    let mut counter = 0;
    for guard in guards {
        if counter == 0 || mall_area / (counter + 1) > 200 {
            mall.guards.insert(format!("Guard #{}", counter + 1), guard);
            counter += 1;
        } else {
            break;
        }
    }
}

pub fn cut_or_raise(mall: &mut mall::Mall) {
    for (_floor_name, floor) in &mut mall.floors {
        for (_store_name, store) in &mut floor.stores {
            for (_employee_name, employee) in &mut store.employees {
                let hours = employee.working_hours.1 - employee.working_hours.0;
                if hours > 10 {
                    employee.salary += employee.salary * 0.1;
                } else {
                    employee.salary -= employee.salary * 0.1;
                }
            }
        }
    }
}
