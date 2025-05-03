pub mod mall;
pub use mall::{Employee, Floor, Guard, Mall, Store};

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
    pub fn biggest_store(&self) -> Option<&Store> {
        self.floors
            .values()
            .flat_map(|floor| floor.stores.values())
            .max_by_key(|store| store.square_meters)
    }

    pub fn highest_paid_employee(&self) -> Vec<&Employee> {
        let mut highest_salary = 0.0;
        let mut highest_paid_employees = Vec::new();

        for floor in self.floors.values() {
            for store in floor.stores.values() {
                for employee in store.employees.values() {
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

        for floor in self.floors.values() {
            for store in floor.stores.values() {
                count += store.employees.len();
            }
        }

        count
    }

    pub fn check_for_securities(&mut self, new_guards: Vec<Guard>) {
        let total_square_meters: u64 = self.floors.values().map(|floor| floor.size_limit).sum();
        let required_guards = (total_square_meters / 200) as usize;

        if self.guards.len() < required_guards {
            let additional_guards = required_guards - self.guards.len();
            for guard in new_guards.into_iter().take(additional_guards) {
                self.guards.insert(format!("Guard_{}", self.guards.len()), guard);
            }
        }
    }

    pub fn cut_or_raise(&mut self) {
        for floor in self.floors.values_mut() {
            for store in floor.stores.values_mut() {
                for employee in store.employees.values_mut() {
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
