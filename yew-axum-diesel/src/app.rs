use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Deserialize, Debug)]
pub struct Employee {
    name: String,
    age: i32,
    department: String,
    working: bool,
}

#[derive(Properties, PartialEq)]
pub struct EmployeeListProp {
    employees: Vec<Employee>,
}

#[derive(Properties, PartialEq)]
pub struct EmployeeDetailProp {
    employee: Employee,
}

#[function_component(EmpDetail)]
pub fn emp_detail(emp: &EmployeeDetailProp) -> Html {
    html! {
        <div>
            <h2>{"EmployeeName:"}<span>{emp.employee.name.clone()}</span></h2>
            <p>{"Age:"}<span>{emp.employee.age}</span></p>
            <p>{"Department:"}<span>{emp.employee.department.clone()}</span></p>
        </div>
    }
}

#[function_component(EmpList)]
pub fn emp_list(allemps: &EmployeeListProp) -> Html {
    allemps
        .employees
        .iter()
        .map(|emp| {
            html! {
            <div>
                <h2>{"EmployeeName:"}<span>{emp.name.clone()}</span></h2>
                <p>{"Age:"}<span>{emp.age}</span></p>
                <p>{"Department:"}<span>{emp.department.clone()}</span></p>
            </div>
            }
        })
        .collect()
}

#[function_component(App)]
pub fn app() -> Html {
    let emp_list = use_state(|| Vec::new());

    {
        let data = emp_list.clone();

        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_data: Vec<Employee> = Request::get("http://127.0.0.1:3000/all")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    log!(format!("Fetched Data: {:?}", fetched_data));
                    data.set(fetched_data);
                });
                || ()
            },
            (),
        );
    }
    html! {
        <main>
            <h1>{ "Employee Data" }</h1>
            <EmpList employees={(*emp_list).clone()} />
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
