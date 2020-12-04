
pub enum TaskOfDay {
    First,
    Second,
}

pub fn split_in2_tuple(s_: &str, ssplit: &str) -> (String, String) {
    let mut splt = s_.split(ssplit).map(|s| s.trim().to_string());
    (splt.next().unwrap(), splt.next().unwrap())
}

pub fn to_string_vec(v: &Vec<&str>) -> Vec<String>
{
    v.iter().map(|elt| elt.to_string()).collect::<Vec<String>>()
}