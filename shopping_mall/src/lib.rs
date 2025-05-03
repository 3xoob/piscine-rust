pub mod mall;
pub use mall::floor::store;
pub use mall::floor::store::employee::Employee;
pub use mall::floor::store::Store;
pub use mall::guard::Guard;
pub use mall::*;

pub fn biggest_store(mall: Mall) -> Store {
    mall.biggest_store().unwrap().clone()
}

pub fn highest_paid_employee(mall: Mall) -> Vec<Employee> {
    mall.highest_paid_employee().into_iter().cloned().collect()
}

pub fn nbr_of_employees(mall: Mall) -> usize {
    mall.nbr_of_employees()
}

pub fn check_for_securities(mall: &mut Mall, new_guards: Vec<Guard>) {
    mall.check_for_securities(new_guards)
}

pub fn cut_or_raise(mall: &mut Mall) {
    mall.cut_or_raise()
}

impl Mall {
    pub fn biggest_store(&self) -> Option<&store::Store> {
        self.floors
            .iter()
            .flat_map(|floor| &floor.stores)
            .max_by_key(|store| store.square_meters)
    }

    pub fn highest_paid_employee(&self) -> Vec<&store::employee::Employee> {
        let mut highest_salary = 0.0;
        let mut highest_paid_employees = Vec::new();

        for floor in &self.floors {
            for store in &floor.stores {
                for employee in &store.employees {
                    if employee.salary > highest_salary {
                        highest_salary = employee.salary;
                        highest_paid_employees.clear();
                        highest_paid_employees.push(employee);
                    } else if employee.salary == highest_salary {
                        highest_paid_employees.push(employee);
                    }
                }
            }
        }

        highest_paid_employees
    }

    pub fn nbr_of_employees(&self) -> usize {
        let mut count = self.guards.len();

        for floor in &self.floors {
            for store in &floor.stores {
                count += store.employees.len();
            }
        }

        count
    }

    pub fn check_for_securities(&mut self, new_guards: Vec<guard::Guard>) {
        let total_square_meters: u64 = self.floors.iter().map(|floor| floor.size_limit).sum();
        let required_guards = (total_square_meters / 200) as usize;

        if self.guards.len() < required_guards {
            let additional_guards = required_guards - self.guards.len();
            self.guards
                .extend(new_guards.into_iter().take(additional_guards));
        }
    }

    pub fn cut_or_raise(&mut self) {
        for floor in &mut self.floors {
            for store in &mut floor.stores {
                for employee in &mut store.employees {
                    if employee.working_hours.1 - employee.working_hours.0 > 10 {
                        employee.raise(employee.salary * 0.10);
                    } else {
                        employee.cut(employee.salary * 0.10);
                    }
                }
            }
        }
    }
}
